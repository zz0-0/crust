mod operation_convergence {
    use std::{collections::HashMap, time::Duration};

    use crust_config::instance::{
        get_state_from_instance, send_command_to_instance, setup_remote_test_environement,
        teardown_remote_test_environment, update_replicas, DeploymentConfig,
    };
    use crust_core::{
        command::{CounterInnerCommand, CrdtInnerCommand},
        core::counter::gcounter::GCounter,
        r#type::{CrdtType, CrdtTypeVariant},
    };
    use futures::future::try_join_all;

    use crate::remote_validation::OperationBasedDistributedConvergenceValidation;

    impl OperationBasedDistributedConvergenceValidation<GCounter<String>> for GCounter<String> {
        async fn operation_based_converge_concurrent_operations() -> bool {
            println!("\n=== TEST: Operation-Based CRDT Convergence with Concurrent Operations ===");

            println!("Step 1/6: Setting up test environment with 3 replicas");
            let client = kube::Client::try_default().await.unwrap();
            let namespace = "default";
            let deployment_name = "crust-network";
            let replicas = 3;
            let test_config = DeploymentConfig::new(
                replicas.try_into().unwrap(),
                "gcounter",
                "operation", // Using operation-based synchronization
                "immediate", // With causal delivery
                None,
                None,
            );
            let service_urls = setup_remote_test_environement(&test_config).await;
            update_replicas(client, namespace, deployment_name, replicas).await;

            if service_urls.is_empty() {
                eprintln!("❌ Failed: Error setting up test environment");
                teardown_remote_test_environment(&test_config).await;
                return false;
            }
            println!("✅ Environment setup complete");

            println!("\nStep 2/6: Preparing test operations");
            let increment_values = vec!["5".to_string(), "3".to_string(), "7".to_string()];
            let commands: Vec<_> = increment_values
                .iter()
                .map(|v| {
                    CrdtInnerCommand::Counter(CounterInnerCommand::Increment { value: v.clone() })
                })
                .collect();
            let instance_ids = service_urls.keys().cloned().collect::<Vec<String>>();
            println!(
                "✅ Operations prepared: increment values {:?}",
                increment_values
            );

            println!("\nStep 3/6: Sending concurrent operations to different replicas");
            let handles = instance_ids
                .iter()
                .enumerate()
                .take(commands.len())
                .map(|(index, instance_id)| {
                    let service_url = service_urls.get(instance_id).unwrap().clone();
                    let command = commands[index].clone();
                    let instance_id_clone = instance_id.clone();
                    let test_config_clone = test_config.clone();
                    let value = increment_values[index].clone();

                    tokio::spawn(async move {
                        println!(
                            "→ Sending operation increment {} to instance {}",
                            value, instance_id_clone
                        );
                        send_command_to_instance(
                            &instance_id_clone,
                            &test_config_clone.crdt_type.name(),
                            &test_config_clone.sync_type.to_string(),
                            &test_config_clone.sync_mode.to_string(),
                            command,
                            &service_url,
                        )
                        .await
                    })
                })
                .collect::<Vec<_>>();

            if let Err(e) = try_join_all(handles).await {
                eprintln!("❌ Failed: Error executing concurrent operations: {:?}", e);
                teardown_remote_test_environment(&test_config).await;
                return false;
            }
            println!("✅ All operations sent successfully");

            println!("\nStep 4/6: Waiting for operation propagation (5 seconds)");
            println!(
                "→ In operation-based mode, individual operations are propagated to all replicas"
            );
            println!("→ Operations must be delivered in causal order");
            tokio::time::sleep(Duration::from_secs(5)).await;
            println!("✅ Wait complete");

            println!("\nStep 5/6: Retrieving final states from all instances");
            let mut final_states: HashMap<String, CrdtType<String>> = HashMap::new();
            for instance_id in instance_ids.iter() {
                println!("→ Fetching state from instance {}", instance_id);
                let service_url = service_urls.get(instance_id).unwrap();
                match get_state_from_instance(instance_id, &test_config.crdt_type, service_url)
                    .await
                {
                    Ok(state) => {
                        final_states.insert(instance_id.clone(), state);
                        println!("✅ State retrieved successfully");
                    }
                    Err(e) => {
                        eprintln!(
                            "❌ Error getting state from instance {}: {}",
                            instance_id, e
                        );
                        teardown_remote_test_environment(&test_config).await;
                        return false;
                    }
                }
            }
            println!("✅ All states retrieved");

            println!("\nStep 6/6: Verifying convergence across all instances");
            let expected_final_value = increment_values
                .iter()
                .map(|v| v.parse::<i32>().unwrap())
                .sum::<i32>()
                .to_string();
            println!("→ Expected final counter value: {}", expected_final_value);

            let mut all_converged = true;
            for (instance_id, state) in final_states.iter() {
                match &state.variant {
                    CrdtTypeVariant::GCounter(_) => {
                        let counter_value = state.get_state();
                        let actual_value = counter_value["value"].as_str().unwrap_or("0");

                        println!(
                            "→ Instance {}: counter value = {}",
                            instance_id, actual_value
                        );

                        if actual_value != expected_final_value {
                            eprintln!(
                                "❌ Failed: Instance {} did not converge to expected value",
                                instance_id
                            );
                            eprintln!(
                                "Expected: {}, Actual: {}",
                                expected_final_value, actual_value
                            );
                            all_converged = false;
                        } else {
                            println!("✅ Correct value confirmed");
                        }
                    }
                    _ => {
                        eprintln!("❌ Unexpected CRDT variant returned");
                        all_converged = false;
                    }
                }
            }

            println!("\nTest cleanup: Tearing down test environment");
            teardown_remote_test_environment(&test_config).await;

            if all_converged {
                println!("\n✅ TEST PASSED: All instances correctly converged with operation-based synchronization");
                println!(
                    "→ This validates that operations were correctly propagated between replicas"
                );
                println!("→ And that concurrent operations were handled properly");
            } else {
                println!("\n❌ TEST FAILED: Some instances did not converge properly with operation-based sync");
            }

            all_converged
        }

        async fn operation_based_converge_delayed_deliveries() -> bool {
            println!("\n=== TEST: Operation-Based CRDT Convergence with Delayed Deliveries ===");

            println!("Step 1/6: Setting up test environment with 2 replicas");
            let client = kube::Client::try_default().await.unwrap();
            let namespace = "default";
            let deployment_name = "crust-network";
            let replicas = 2;
            let test_config = DeploymentConfig::new(
                replicas.try_into().unwrap(),
                "gcounter",
                "operation", // Using operation-based synchronization
                "immediate", // With causal delivery
                None,
                None,
            );
            let service_urls = setup_remote_test_environement(&test_config).await;
            update_replicas(client, namespace, deployment_name, replicas).await;

            if service_urls.is_empty() {
                eprintln!("❌ Failed: Error setting up test environment");
                teardown_remote_test_environment(&test_config).await;
                return false;
            }
            println!("✅ Environment setup complete");

            let instance_ids = service_urls.keys().cloned().collect::<Vec<String>>();
            let instance_1 = &instance_ids[0];
            let instance_2 = &instance_ids[1];
            let service_url_1 = service_urls.get(instance_1).unwrap().clone();
            let service_url_2 = service_urls.get(instance_2).unwrap().clone();
            println!("→ Selected instances: {} and {}", instance_1, instance_2);

            println!("\nStep 2/6: Sending first operation to instance 1");
            let increment_value_1 = "5";
            let command_1 = CrdtInnerCommand::Counter(CounterInnerCommand::Increment {
                value: increment_value_1.to_string(),
            });

            println!(
                "→ Sending increment {} to instance {}",
                increment_value_1, instance_1
            );
            if let Err(e) = send_command_to_instance(
                instance_1,
                &test_config.crdt_type.name(),
                &test_config.sync_type.to_string(),
                &test_config.sync_mode.to_string(),
                command_1,
                &service_url_1,
            )
            .await
            {
                eprintln!("❌ Error sending command to instance {}: {}", instance_1, e);
                teardown_remote_test_environment(&test_config).await;
                return false;
            }
            println!("✅ First operation sent successfully");

            // Short delay for operation to be processed locally
            tokio::time::sleep(Duration::from_millis(500)).await;

            println!("\nStep 3/6: Verifying local state of first instance");
            println!(
                "  → Checking if operation was applied locally to instance {}",
                instance_1
            );
            let state_1 =
                match get_state_from_instance(instance_1, &test_config.crdt_type, &service_url_1)
                    .await
                {
                    Ok(state) => {
                        let counter_value = state.get_state();
                        println!("✅ State retrieved: {:?}", counter_value);
                        state
                    }
                    Err(e) => {
                        eprintln!("❌ Error getting state from instance {}: {}", instance_1, e);
                        teardown_remote_test_environment(&test_config).await;
                        return false;
                    }
                };

            // Check instance 2 to see if it's received the operation yet (may or may not have)
            println!(
                "→ Checking if instance {} has received the operation yet",
                instance_2
            );
            let intermediate_state_2 =
                match get_state_from_instance(instance_2, &test_config.crdt_type, &service_url_2)
                    .await
                {
                    Ok(state) => {
                        let counter_value = state.get_state();
                        let value = counter_value["value"].as_str().unwrap_or("0");
                        if value == "0" {
                            println!(
                                "✅ As expected, operation hasn't propagated to instance {} yet",
                                instance_2
                            );
                        } else {
                            println!(
                                "⚠️ Operation has already propagated to instance {}",
                                instance_2
                            );
                        }
                        state
                    }
                    Err(e) => {
                        eprintln!("❌ Error getting state from instance {}: {}", instance_2, e);
                        teardown_remote_test_environment(&test_config).await;
                        return false;
                    }
                };

            println!("\nStep 4/6: Sending second operation to instance 2 (before operation 1 is received)");
            let increment_value_2 = "3";
            let command_2 = CrdtInnerCommand::Counter(CounterInnerCommand::Increment {
                value: increment_value_2.to_string(),
            });

            println!(
                "→ Sending increment {} to instance {}",
                increment_value_2, instance_2
            );
            if let Err(e) = send_command_to_instance(
                instance_2,
                &test_config.crdt_type.name(),
                &test_config.sync_type.to_string(),
                &test_config.sync_mode.to_string(),
                command_2,
                &service_url_2,
            )
            .await
            {
                eprintln!("❌ Error sending command to instance {}: {}", instance_2, e);
                teardown_remote_test_environment(&test_config).await;
                return false;
            }
            println!("✅ Second operation sent successfully");

            // Check local state of instance 2 after its own operation
            println!(
                "→ Checking local state of instance {} after applying its operation",
                instance_2
            );
            let state_2_after_local =
                match get_state_from_instance(instance_2, &test_config.crdt_type, &service_url_2)
                    .await
                {
                    Ok(state) => {
                        let counter_value = state.get_state();
                        println!("✅ State retrieved: {:?}", counter_value);

                        // For operation-based, this should reflect only the local operation
                        // if the first operation hasn't been delivered yet
                        let value = counter_value["value"].as_str().unwrap_or("0");
                        if value == increment_value_2 {
                            println!(
                                "✅ Instance {} has only applied its own operation so far",
                                instance_2
                            );
                        } else {
                            println!(
                            "⚠️ Instance {} may have already received operations from instance {}",
                            instance_2, instance_1
                        );
                        }
                        state
                    }
                    Err(e) => {
                        eprintln!(
                            " ❌ Error getting state from instance {}: {}",
                            instance_2, e
                        );
                        teardown_remote_test_environment(&test_config).await;
                        return false;
                    }
                };

            println!("\nStep 5/6: Waiting for delayed operation propagation (10 seconds)");
            println!("→ In operation-based mode, operations must eventually be delivered to all replicas");
            println!("→ Allowing time for operations to propagate between instances...");
            tokio::time::sleep(Duration::from_secs(10)).await;
            println!("✅ Wait complete");

            println!("\nStep 6/6: Verifying final convergence after delayed operations");
            let expected_final_value = (increment_value_1.parse::<i32>().unwrap()
                + increment_value_2.parse::<i32>().unwrap())
            .to_string();
            println!("→ Expected final counter value: {}", expected_final_value);

            let mut all_converged = true;
            for instance_id in &instance_ids {
                println!("→ Checking final state of instance {}", instance_id);
                let service_url = service_urls.get(instance_id).unwrap();
                match get_state_from_instance(instance_id, &test_config.crdt_type, service_url)
                    .await
                {
                    Ok(state) => {
                        let counter_value = state.get_state();
                        let actual_value = counter_value["value"].as_str().unwrap_or("0");

                        println!(
                            "→ Instance {}: counter value = {}",
                            instance_id, actual_value
                        );

                        if actual_value != expected_final_value {
                            eprintln!(
                                "❌ Failed: Instance {} did not converge to expected value",
                                instance_id
                            );
                            eprintln!(
                                "Expected: {}, Actual: {}",
                                expected_final_value, actual_value
                            );
                            all_converged = false;
                        } else {
                            println!("✅ Correct value confirmed");
                        }
                    }
                    Err(e) => {
                        eprintln!(
                            "❌ Error getting state from instance {}: {}",
                            instance_id, e
                        );
                        all_converged = false;
                    }
                }
            }

            println!("\nTest cleanup: Tearing down test environment");
            teardown_remote_test_environment(&test_config).await;

            if all_converged {
                println!("\n✅ TEST PASSED: All instances correctly converged after delayed operation delivery");
                println!(
                    "→ This validates that operation-based CRDTs handle network delays properly"
                );
                println!("→ And that operations are eventually delivered to all replicas");
            } else {
                println!("\n❌ TEST FAILED: Some instances did not converge properly after delayed delivery");
            }

            all_converged
        }
        async fn operation_based_converge_mixed_operations() -> bool {
            println!("=== TEST: Operation-Based CRDT Convergence with Mixed Operations ===");
            println!("GCounter only have increment operations, so this test is not applicable");
            true
        }

        async fn operation_based_converge_under_load() -> bool {
            println!("\n=== TEST: Operation-Based CRDT Convergence Under Load ===");

            println!("Step 1/6: Setting up test environment with 3 replicas");
            let client = kube::Client::try_default().await.unwrap();
            let namespace = "default";
            let deployment_name = "crust-network";
            let replicas = 3;
            let test_config = DeploymentConfig::new(
                replicas.try_into().unwrap(),
                "gcounter",
                "operation", // Using operation-based synchronization
                "immediate", // With causal delivery
                None,
                None,
            );
            let service_urls = setup_remote_test_environement(&test_config).await;
            update_replicas(client, namespace, deployment_name, replicas).await;

            if service_urls.is_empty() {
                eprintln!("❌ Failed: Error setting up test environment");
                teardown_remote_test_environment(&test_config).await;
                return false;
            }
            println!("✅ Environment setup complete");

            println!("\nStep 2/6: Preparing high-load test operations");
            let num_operations = 50; // High load with 50 operations
            let instance_ids = service_urls.keys().cloned().collect::<Vec<String>>();

            // Create sequential increment values to make validation simpler
            let mut total_expected = 0;
            let mut operations = Vec::with_capacity(num_operations);
            for i in 1..=num_operations {
                operations.push((i.to_string(), i));
                total_expected += i;
            }
            println!(
                "✅ Prepared {} operations with total expected sum: {}",
                num_operations, total_expected
            );

            println!("\nStep 3/6: Distributing operations across instances");
            println!("→ In operation-based mode, each operation is propagated to all replicas");
            let mut handles = Vec::new();

            for (i, (value, _)) in operations.iter().enumerate() {
                // Round-robin distribution across instances
                let target_idx = i % instance_ids.len();
                let instance_id = &instance_ids[target_idx];
                let service_url = service_urls.get(instance_id).unwrap().clone();
                let test_config_clone = test_config.clone();
                let instance_id_clone = instance_id.clone();
                let value_clone = value.clone();

                // Create a task for each operation
                handles.push(tokio::spawn(async move {
                    println!(
                        "→ Sending increment {} to instance {}",
                        value_clone, instance_id_clone
                    );
                    let command = CrdtInnerCommand::Counter(CounterInnerCommand::Increment {
                        value: value_clone.clone(),
                    });

                    match send_command_to_instance(
                        &instance_id_clone,
                        &test_config_clone.crdt_type.name(),
                        &test_config_clone.sync_type.to_string(),
                        &test_config_clone.sync_mode.to_string(),
                        command,
                        &service_url,
                    )
                    .await
                    {
                        Ok(_) => true,
                        Err(e) => {
                            eprintln!(
                                "❌ Failed to send operation {} to {}: {}",
                                value_clone, instance_id_clone, e
                            );
                            false
                        }
                    }
                }));
            }

            println!("⏳ Executing all operations concurrently...");
            let results = try_join_all(handles).await.unwrap_or_else(|e| {
                eprintln!("❌ Error joining operation tasks: {:?}", e);
                vec![false]
            });

            if results.iter().any(|&success| !success) {
                eprintln!("❌ Some operations failed");
                teardown_remote_test_environment(&test_config).await;
                return false;
            }
            println!("✅ All operations sent successfully");

            println!("\nStep 4/6: Allowing extended time for convergence under load (15 seconds)");
            println!(
                "→ System processing {} operations across {} instances...",
                num_operations, replicas
            );
            println!("→ Operation delivery and causal ordering is being tested under load");
            tokio::time::sleep(Duration::from_secs(15)).await;
            println!("✅ Wait complete");

            println!("\nStep 5/6: Retrieving final states from all instances");
            let mut final_states: HashMap<String, CrdtType<String>> = HashMap::new();
            for instance_id in instance_ids.iter() {
                println!("→ Fetching state from instance {}", instance_id);
                let service_url = service_urls.get(instance_id).unwrap();
                match get_state_from_instance(instance_id, &test_config.crdt_type, service_url)
                    .await
                {
                    Ok(state) => {
                        final_states.insert(instance_id.clone(), state);
                        println!("✅ State retrieved successfully");
                    }
                    Err(e) => {
                        eprintln!(
                            "❌ Error getting state from instance {}: {}",
                            instance_id, e
                        );
                        teardown_remote_test_environment(&test_config).await;
                        return false;
                    }
                }
            }
            println!("✅ All states retrieved");

            println!("\nStep 6/6: Verifying convergence across all instances");
            let expected_final_value = total_expected.to_string();
            println!("→ Expected final counter value: {}", expected_final_value);

            let mut all_converged = true;
            for (instance_id, state) in final_states.iter() {
                match &state.variant {
                    CrdtTypeVariant::GCounter(_) => {
                        let counter_value = state.get_state();
                        let actual_value = counter_value["value"].as_str().unwrap_or("0");

                        println!(
                            "→ Instance {}: counter value = {}",
                            instance_id, actual_value
                        );

                        if actual_value != expected_final_value {
                            eprintln!(
                                "❌ Failed: Instance {} did not converge to expected value",
                                instance_id
                            );
                            eprintln!(
                                "   Expected: {}, Actual: {}",
                                expected_final_value, actual_value
                            );
                            all_converged = false;
                        } else {
                            println!("✅ Correct value confirmed");
                        }
                    }
                    _ => {
                        eprintln!("❌ Unexpected CRDT variant returned");
                        all_converged = false;
                    }
                }
            }

            println!("\nTest cleanup: Tearing down test environment");
            teardown_remote_test_environment(&test_config).await;

            if all_converged {
                println!("\n✅ TEST PASSED: All instances correctly converged to {} after processing {} operations", 
                expected_final_value, num_operations);
                println!("→ This validates that operation-based CRDTs handle high load correctly");
                println!("→ And maintain convergence even under many concurrent operations");
            } else {
                println!("\n❌ TEST FAILED: Some instances did not converge properly under load");
            }

            all_converged
        }
    }
}
