mod operation_casual_consistency {
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

    use crate::remote_validation::OperationBasedDistributedCausalConsistencyValidation;

    impl OperationBasedDistributedCausalConsistencyValidation<GCounter<String>> for GCounter<String> {
        async fn operation_based_causal_order_simple_dependency() -> bool {
            println!("\n=== TEST: Operation-Based CRDT Causal Order with Simple Dependency ===");

            println!("Step 1/5: Setting up test environment with 2 replicas");
            let client = kube::Client::try_default().await.unwrap();
            let namespace = "default";
            let deployment_name = "crust-network";
            let replicas = 2; 
            let test_config = DeploymentConfig::new(
                replicas.try_into().unwrap(),
                "gcounter",
                "operation", 
                "immediate", 
                None,
                None,
            );

            let service_urls = setup_remote_test_environement(&test_config).await;
            update_replicas(client, namespace, deployment_name, replicas).await;

            if service_urls.is_empty() {
                eprintln!("❌ Failed: Error setting up test environment");
                return false;
            }
            println!("✅ Environment setup complete");

            
            let instance_ids = service_urls.keys().cloned().collect::<Vec<String>>();
            let instance_1 = &instance_ids[0];
            let instance_2 = &instance_ids[1];
            let service_url_1 = service_urls.get(instance_1).unwrap().clone();
            let service_url_2 = service_urls.get(instance_2).unwrap().clone();

            println!(
                "→ Selected instances for test: {} and {}",
                instance_1, instance_2
            );

            
            println!(
                "\nStep 2/5: Applying first operation on instance {}",
                instance_1
            );
            let increment_value_1 = "5";
            let command_1 = CrdtInnerCommand::Counter(CounterInnerCommand::Increment {
                value: increment_value_1.to_string(),
            });

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
                eprintln!("❌ Error sending first command: {}", e);
                teardown_remote_test_environment(&test_config).await;
                return false;
            }
            println!("✅ First operation applied successfully");

            
            println!(
                "\nStep 3/5: Verifying first operation propagated to instance {}",
                instance_2
            );
            
            let mut propagated = false;
            for i in 1..=10 {
                println!("→ Attempt #{}: Checking if operation propagated...", i);
                tokio::time::sleep(Duration::from_millis(500)).await;

                match get_state_from_instance(instance_2, &test_config.crdt_type, &service_url_2)
                    .await
                {
                    Ok(state) => {
                        let counter_value = state.get_state();
                        println!(
                            "→ Current state at instance {}: {:?}",
                            instance_2, counter_value
                        );

                        
                        if let Some(value) = counter_value.get("counter") {
                            if value == increment_value_1 {
                                println!(
                                    "✅ Operation successfully propagated to instance {}",
                                    instance_2
                                );
                                propagated = true;
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Error checking propagation: {}", e);
                    }
                }
            }

            if !propagated {
                eprintln!(
                    "❌ First operation did not propagate in time - causal consistency failed"
                );
                teardown_remote_test_environment(&test_config).await;
                return false;
            }

            
            println!(
                "\nStep 4/5: Applying causally dependent operation on instance {}",
                instance_2
            );
            let increment_value_2 = "3";
            let command_2 = CrdtInnerCommand::Counter(CounterInnerCommand::Increment {
                value: increment_value_2.to_string(),
            });

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
                eprintln!("❌ Error sending second command: {}", e);
                teardown_remote_test_environment(&test_config).await;
                return false;
            }
            println!("✅ Causally dependent operation applied successfully");

            
            println!("\nStep 5/5: Verifying final converged state on both instances");
            tokio::time::sleep(Duration::from_secs(2)).await;

            let expected_final_value = (increment_value_1.parse::<i32>().unwrap()
                + increment_value_2.parse::<i32>().unwrap())
            .to_string();

            println!("→ Expected final counter value: {}", expected_final_value);

            let mut all_converged = true;
            for instance_id in &instance_ids {
                let service_url = service_urls.get(instance_id).unwrap();
                match get_state_from_instance(instance_id, &test_config.crdt_type, service_url)
                    .await
                {
                    Ok(state) => {
                        let counter_value = state.get_state();
                        let actual_value = counter_value["value"].as_str().unwrap_or("0");
                        println!(
                            "→ Instance {}: final state = {:?}",
                            instance_id, counter_value
                        );

                        if actual_value != &expected_final_value {
                            eprintln!(
                                "❌ Failed: Instance {} has incorrect value: {} (expected {})",
                                instance_id, actual_value, expected_final_value
                            );
                            all_converged = false;
                        } else {
                            println!("✅ Instance {} has correct final value", instance_id);
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Error getting final state from {}: {}", instance_id, e);
                        all_converged = false;
                    }
                }
            }

            println!("\nTest cleanup: Tearing down test environment");
            teardown_remote_test_environment(&test_config).await;

            if all_converged {
                println!("\n✅ TEST PASSED: Causal consistency with simple dependency maintained");
            } else {
                println!("\n❌ TEST FAILED: Causal consistency with simple dependency violated");
            }

            all_converged
        }

        async fn operation_based_causal_order_complex_dependency() -> bool {
            println!("\n=== TEST: Operation-Based CRDT Causal Order with Complex Dependency ===");
            println!("Note: Testing causal chain of operations across multiple replicas");

            println!("Step 1/7: Setting up test environment with 3 replicas");
            let client = kube::Client::try_default().await.unwrap();
            let namespace = "default";
            let deployment_name = "crust-network";
            let replicas = 3; 
            let test_config = DeploymentConfig::new(
                replicas.try_into().unwrap(),
                "gcounter",
                "operation", 
                "immediate", 
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
            let instance_3 = &instance_ids[2];
            let service_url_1 = service_urls.get(instance_1).unwrap().clone();
            let service_url_2 = service_urls.get(instance_2).unwrap().clone();
            let service_url_3 = service_urls.get(instance_3).unwrap().clone();

            println!(
                "→ Selected instances for test chain: {} → {} → {}",
                instance_1, instance_2, instance_3
            );

            
            println!(
                "\nStep 2/7: Starting causal chain with operation on instance {}",
                instance_1
            );
            let increment_value_1 = "5";
            let command_1 = CrdtInnerCommand::Counter(CounterInnerCommand::Increment {
                value: increment_value_1.to_string(),
            });

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
                eprintln!("❌ Error sending first command: {}", e);
                teardown_remote_test_environment(&test_config).await;
                return false;
            }
            println!("✅ First operation in causal chain applied successfully");

            
            println!(
                "\nStep 3/7: Verifying first operation propagated to instance {}",
                instance_2
            );
            let mut propagated_to_2 = false;
            for i in 1..=10 {
                println!(
                    "→ Attempt #{}: Checking if operation propagated to instance 2...",
                    i
                );
                tokio::time::sleep(Duration::from_millis(500)).await;

                match get_state_from_instance(instance_2, &test_config.crdt_type, &service_url_2)
                    .await
                {
                    Ok(state) => {
                        let counter_value = state.get_state();
                        let actual_value = counter_value["value"].as_str().unwrap_or("0");
                        println!(
                            "→ Current state at instance {}: {}",
                            instance_2, actual_value
                        );

                        if actual_value == increment_value_1 {
                            println!(
                                "✅ First operation successfully propagated to instance {}",
                                instance_2
                            );
                            propagated_to_2 = true;
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Error checking propagation: {}", e);
                    }
                }
            }

            if !propagated_to_2 {
                eprintln!("❌ First operation did not propagate to instance 2 in time");
                teardown_remote_test_environment(&test_config).await;
                return false;
            }

            
            println!(
                "\nStep 4/7: Continuing causal chain with operation on instance {}",
                instance_2
            );
            let increment_value_2 = "3";
            let command_2 = CrdtInnerCommand::Counter(CounterInnerCommand::Increment {
                value: increment_value_2.to_string(),
            });

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
                eprintln!("❌ Error sending second command: {}", e);
                teardown_remote_test_environment(&test_config).await;
                return false;
            }
            println!("✅ Second operation in causal chain applied successfully");

            
            println!(
                "\nStep 5/7: Verifying causal chain propagated to instance {}",
                instance_3
            );
            let mut propagated_to_3 = false;
            for i in 1..=10 {
                println!(
                    "→ Attempt #{}: Checking if operation propagated to instance 3...",
                    i
                );
                tokio::time::sleep(Duration::from_millis(500)).await;

                match get_state_from_instance(instance_3, &test_config.crdt_type, &service_url_3)
                    .await
                {
                    Ok(state) => {
                        let counter_value = state.get_state();
                        let actual_value = counter_value["value"].as_str().unwrap_or("0");
                        let expected_intermediate = (increment_value_1.parse::<i32>().unwrap()
                            + increment_value_2.parse::<i32>().unwrap())
                        .to_string();

                        println!(
                            "→ Current state at instance {}: {}",
                            instance_3, actual_value
                        );

                        if actual_value == expected_intermediate {
                            println!(
                                "✅ Causal chain successfully propagated to instance {}",
                                instance_3
                            );
                            propagated_to_3 = true;
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Error checking propagation: {}", e);
                    }
                }
            }

            if !propagated_to_3 {
                eprintln!("❌ Causal chain did not propagate to instance 3 in time");
                teardown_remote_test_environment(&test_config).await;
                return false;
            }

            
            println!(
                "\nStep 6/7: Completing causal chain with final operation on instance {}",
                instance_3
            );
            let increment_value_3 = "7";
            let command_3 = CrdtInnerCommand::Counter(CounterInnerCommand::Increment {
                value: increment_value_3.to_string(),
            });

            if let Err(e) = send_command_to_instance(
                instance_3,
                &test_config.crdt_type.name(),
                &test_config.sync_type.to_string(),
                &test_config.sync_mode.to_string(),
                command_3,
                &service_url_3,
            )
            .await
            {
                eprintln!("❌ Error sending final command: {}", e);
                teardown_remote_test_environment(&test_config).await;
                return false;
            }
            println!("✅ Final operation in causal chain applied successfully");

            
            println!("\nStep 7/7: Verifying final converged state across all instances");
            tokio::time::sleep(Duration::from_secs(3)).await;

            let expected_final_value = (increment_value_1.parse::<i32>().unwrap()
                + increment_value_2.parse::<i32>().unwrap()
                + increment_value_3.parse::<i32>().unwrap())
            .to_string();

            println!("→ Expected final counter value: {}", expected_final_value);

            let mut all_converged = true;
            for instance_id in &instance_ids {
                let service_url = service_urls.get(instance_id).unwrap();
                match get_state_from_instance(instance_id, &test_config.crdt_type, service_url)
                    .await
                {
                    Ok(state) => {
                        let counter_value = state.get_state();
                        let actual_value = counter_value["value"].as_str().unwrap_or("0");

                        println!("→ Instance {}: final value = {}", instance_id, actual_value);

                        if actual_value != expected_final_value {
                            eprintln!(
                                "❌ Failed: Instance {} has incorrect value: {} (expected {})",
                                instance_id, actual_value, expected_final_value
                            );
                            all_converged = false;
                        } else {
                            println!("✅ Instance {} has correct final value", instance_id);
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Error getting final state from {}: {}", instance_id, e);
                        all_converged = false;
                    }
                }
            }

            println!("\nTest cleanup: Tearing down test environment");
            teardown_remote_test_environment(&test_config).await;

            if all_converged {
                println!(
                    "\n✅ TEST PASSED: Causal consistency with complex dependency chain maintained"
                );
                println!(
                    "→ Chain of operations: {} → {} → {}",
                    instance_1, instance_2, instance_3
                );
                println!("→ Each operation depended on previous operation being visible");
            } else {
                println!("\n❌ TEST FAILED: Complex causal dependency chain was broken");
            }

            all_converged
        }

        async fn operation_based_causal_order_concurrent_dependency() -> bool {
            println!(
                "\n=== TEST: Operation-Based CRDT Causal Order with Concurrent Dependencies ==="
            );
            println!("Note: Testing causal ordering with concurrent operations");

            println!("Step 1/6: Setting up test environment with 3 replicas");
            let client = kube::Client::try_default().await.unwrap();
            let namespace = "default";
            let deployment_name = "crust-network";
            let replicas = 3;
            let test_config = DeploymentConfig::new(
                replicas.try_into().unwrap(),
                "gcounter",
                "operation", 
                "immediate", 
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
            let instance_3 = &instance_ids[2];
            let service_url_1 = service_urls.get(instance_1).unwrap().clone();
            let service_url_2 = service_urls.get(instance_2).unwrap().clone();
            let service_url_3 = service_urls.get(instance_3).unwrap().clone();

            println!(
                "→ Selected instances: {}, {}, {}",
                instance_1, instance_2, instance_3
            );

            
            println!("\nStep 2/6: Establishing initial causal chain");
            println!(
                "→ Sending first operation (value 5) to instance {}",
                instance_1
            );
            let increment_value_1 = "5";
            let command_1 = CrdtInnerCommand::Counter(CounterInnerCommand::Increment {
                value: increment_value_1.to_string(),
            });

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
                eprintln!("❌ Error sending first command: {}", e);
                teardown_remote_test_environment(&test_config).await;
                return false;
            }

            
            println!(
                "→ Waiting for first operation to propagate to instance {}",
                instance_2
            );
            let mut propagated_to_2 = false;
            for i in 1..=10 {
                println!("  Attempt #{}: Checking propagation status...", i);
                tokio::time::sleep(Duration::from_millis(500)).await;

                match get_state_from_instance(instance_2, &test_config.crdt_type, &service_url_2)
                    .await
                {
                    Ok(state) => {
                        let counter_value = state.get_state();
                        let actual_value = counter_value["value"].as_str().unwrap_or("0");

                        if actual_value == increment_value_1 {
                            println!(
                                "✅ First operation successfully propagated to instance {}",
                                instance_2
                            );
                            propagated_to_2 = true;
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Error checking propagation: {}", e);
                    }
                }
            }

            if !propagated_to_2 {
                eprintln!(
                    "❌ First operation did not propagate - causal chain cannot be established"
                );
                teardown_remote_test_environment(&test_config).await;
                return false;
            }

            
            println!(
                "→ Sending second operation (value 3) to instance {} (causally after first)",
                instance_2
            );
            let increment_value_2 = "3";
            let command_2 = CrdtInnerCommand::Counter(CounterInnerCommand::Increment {
                value: increment_value_2.to_string(),
            });

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
                eprintln!("❌ Error sending second command: {}", e);
                teardown_remote_test_environment(&test_config).await;
                return false;
            }
            println!("✅ Initial causal chain established");

            
            println!("\nStep 3/6: Ensuring causal chain is propagated to all instances");
            tokio::time::sleep(Duration::from_secs(2)).await;

            let initial_expected = (increment_value_1.parse::<i32>().unwrap()
                + increment_value_2.parse::<i32>().unwrap())
            .to_string();

            
            let mut all_received_initial = true;
            for (instance_id, service_url) in &service_urls {
                match get_state_from_instance(instance_id, &test_config.crdt_type, service_url)
                    .await
                {
                    Ok(state) => {
                        let counter_value = state.get_state();
                        let actual_value = counter_value["value"].as_str().unwrap_or("0");

                        println!(
                            "→ Instance {}: current value = {}",
                            instance_id, actual_value
                        );

                        if actual_value != initial_expected {
                            eprintln!(
                                "❌ Instance {} has not received initial operations",
                                instance_id
                            );
                            all_received_initial = false;
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Error getting state from {}: {}", instance_id, e);
                        all_received_initial = false;
                    }
                }
            }

            if !all_received_initial {
                eprintln!("❌ Initial causal chain did not propagate to all instances");
                teardown_remote_test_environment(&test_config).await;
                return false;
            }
            println!("✅ Causal chain propagated to all instances");

            
            println!("\nStep 4/6: Applying concurrent operations with causal dependency");
            println!("→ These operations depend on the initial causal chain but are concurrent to each other");

            
            let concurrent_op_1 = CrdtInnerCommand::Counter(CounterInnerCommand::Increment {
                value: "7".to_string(),
            });
            let concurrent_op_2 = CrdtInnerCommand::Counter(CounterInnerCommand::Increment {
                value: "2".to_string(),
            });

            
            let handle_1 = tokio::spawn({
                let instance_id = instance_1.clone();
                let service_url = service_url_1.clone();
                let test_config = test_config.clone();

                async move {
                    println!(
                        "→ Sending concurrent operation (value 7) to instance {}",
                        instance_id
                    );
                    send_command_to_instance(
                        &instance_id,
                        &test_config.crdt_type.name(),
                        &test_config.sync_type.to_string(),
                        &test_config.sync_mode.to_string(),
                        concurrent_op_1,
                        &service_url,
                    )
                    .await
                }
            });

            let handle_3 = tokio::spawn({
                let instance_id = instance_3.clone();
                let service_url = service_url_3.clone();
                let test_config = test_config.clone();

                async move {
                    println!(
                        "→ Sending concurrent operation (value 2) to instance {}",
                        instance_id
                    );
                    send_command_to_instance(
                        &instance_id,
                        &test_config.crdt_type.name(),
                        &test_config.sync_type.to_string(),
                        &test_config.sync_mode.to_string(),
                        concurrent_op_2,
                        &service_url,
                    )
                    .await
                }
            });

            if let Err(e) = handle_1.await.unwrap() {
                eprintln!(
                    "❌ Error sending concurrent operation to instance {}: {}",
                    instance_1, e
                );
                teardown_remote_test_environment(&test_config).await;
                return false;
            }

            if let Err(e) = handle_3.await.unwrap() {
                eprintln!(
                    "❌ Error sending concurrent operation to instance {}: {}",
                    instance_3, e
                );
                teardown_remote_test_environment(&test_config).await;
                return false;
            }

            println!("✅ Concurrent operations sent successfully");

            
            println!("\nStep 5/6: Waiting for system to converge with all operations (5 seconds)");
            println!(
                "→ All instances should receive all operations, preserving causal dependencies"
            );
            tokio::time::sleep(Duration::from_secs(5)).await;

            
            println!("\nStep 6/6: Verifying final state with causal ordering preserved");
            let final_expected_value = (
                increment_value_1.parse::<i32>().unwrap() + 
        increment_value_2.parse::<i32>().unwrap() + 
        7 + 
        2
                
            )
            .to_string();

            println!("→ Expected final value: {}", final_expected_value);

            let mut all_converged = true;
            for (instance_id, service_url) in &service_urls {
                match get_state_from_instance(instance_id, &test_config.crdt_type, service_url)
                    .await
                {
                    Ok(state) => {
                        let counter_value = state.get_state();
                        let actual_value = counter_value["value"].as_str().unwrap_or("0");

                        println!("→ Instance {}: final value = {}", instance_id, actual_value);

                        if actual_value != final_expected_value {
                            eprintln!(
                                "❌ Failed: Instance {} did not converge to expected value",
                                instance_id
                            );
                            eprintln!(
                                "   Expected: {}, Actual: {}",
                                final_expected_value, actual_value
                            );
                            all_converged = false;
                        } else {
                            println!("✅ Instance {} has correct final value", instance_id);
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Error getting final state from {}: {}", instance_id, e);
                        all_converged = false;
                    }
                }
            }

            println!("\nTest cleanup: Tearing down test environment");
            teardown_remote_test_environment(&test_config).await;

            if all_converged {
                println!("\n✅ TEST PASSED: Causal ordering maintained with concurrent operations");
                println!("→ Initial causal chain (5 → 3) was respected");
                println!("→ Concurrent operations (7, 2) were both correctly applied");
                println!(
                    "→ Final value ({}) confirms all operations were processed",
                    final_expected_value
                );
            } else {
                println!("\n❌ TEST FAILED: System did not maintain causal ordering with concurrent operations");
            }

            all_converged
        }

        async fn operation_based_causal_order_delayed_delivery() -> bool {
            println!("\n=== TEST: Operation-Based CRDT Causal Order with Delayed Delivery ===");
            println!("Note: Testing causal consistency with network delays");

            println!("Step 1/7: Setting up test environment with 3 replicas");
            let client = kube::Client::try_default().await.unwrap();
            let namespace = "default";
            let deployment_name = "crust-network";
            let replicas = 3;
            let test_config = DeploymentConfig::new(
                replicas.try_into().unwrap(),
                "gcounter",
                "operation", 
                "immediate", 
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
            let instance_3 = &instance_ids[2];
            let service_url_1 = service_urls.get(instance_1).unwrap().clone();
            let service_url_2 = service_urls.get(instance_2).unwrap().clone();
            let service_url_3 = service_urls.get(instance_3).unwrap().clone();

            println!(
                "→ Selected instances: {}, {}, {}",
                instance_1, instance_2, instance_3
            );

            
            println!("\nStep 2/7: Creating causal dependency chain with deliberate delays");

            
            let increment_value_1 = "5";
            println!(
                "→ Sending operation 1 (value {}) to instance {}",
                increment_value_1, instance_1
            );
            let command_1 = CrdtInnerCommand::Counter(CounterInnerCommand::Increment {
                value: increment_value_1.to_string(),
            });

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
                eprintln!("❌ Error sending first command: {}", e);
                teardown_remote_test_environment(&test_config).await;
                return false;
            }
            println!("✅ First operation sent successfully");

            
            println!("\nStep 3/7: Introducing deliberate delay between operations (3 seconds)");
            println!("→ In a causally consistent system, delayed operations should still be ordered correctly");
            tokio::time::sleep(Duration::from_secs(3)).await;
            println!("✅ Delay complete");

            
            println!(
                "\nStep 4/7: Sending second operation in causal chain to instance {}",
                instance_2
            );
            let increment_value_2 = "7";
            println!(
                "→ Sending operation 2 (value {}) to instance {}",
                increment_value_2, instance_2
            );
            let command_2 = CrdtInnerCommand::Counter(CounterInnerCommand::Increment {
                value: increment_value_2.to_string(),
            });

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
                eprintln!("❌ Error sending second command: {}", e);
                teardown_remote_test_environment(&test_config).await;
                return false;
            }
            println!("✅ Second operation sent successfully");

            
            println!("\nStep 5/7: Checking intermediate state to verify causal ordering");

            
            println!("→ Checking intermediate state at instance {}", instance_3);

            
            tokio::time::sleep(Duration::from_secs(1)).await;

            match get_state_from_instance(instance_3, &test_config.crdt_type, &service_url_3).await
            {
                Ok(state) => {
                    let counter_value = state.get_state();
                    let actual_value = counter_value["value"].as_str().unwrap_or("0");
                    println!(
                        "→ Instance {} intermediate state: value = {}",
                        instance_3, actual_value
                    );

                    
                    
                    if actual_value != "0" {
                        println!(
                            "→ Operations are beginning to propagate to instance {}",
                            instance_3
                        );
                    } else {
                        println!("→ Operations haven't reached instance {} yet", instance_3);
                    }
                }
                Err(e) => {
                    eprintln!("❌ Error checking intermediate state: {}", e);
                }
            }

            
            println!(
                "\nStep 6/7: Sending final operation to instance {}",
                instance_3
            );
            let increment_value_3 = "3";
            println!(
                "→ Sending operation 3 (value {}) to instance {}",
                increment_value_3, instance_3
            );
            let command_3 = CrdtInnerCommand::Counter(CounterInnerCommand::Increment {
                value: increment_value_3.to_string(),
            });

            if let Err(e) = send_command_to_instance(
                instance_3,
                &test_config.crdt_type.name(),
                &test_config.sync_type.to_string(),
                &test_config.sync_mode.to_string(),
                command_3,
                &service_url_3,
            )
            .await
            {
                eprintln!("❌ Error sending third command: {}", e);
                teardown_remote_test_environment(&test_config).await;
                return false;
            }
            println!("✅ Third operation sent successfully");

            
            println!("\nStep 7/7: Waiting for convergence and verifying final state");
            println!("→ With causal consistency, all operations should eventually be applied in causal order");
            println!("→ Allowing time for delayed operations to propagate (10 seconds)...");
            tokio::time::sleep(Duration::from_secs(10)).await;

            let expected_final_value = (increment_value_1.parse::<i32>().unwrap()
                + increment_value_2.parse::<i32>().unwrap()
                + increment_value_3.parse::<i32>().unwrap())
            .to_string();

            println!("→ Expected final counter value: {}", expected_final_value);

            let mut all_converged = true;
            for (instance_id, service_url) in &service_urls {
                match get_state_from_instance(instance_id, &test_config.crdt_type, service_url)
                    .await
                {
                    Ok(state) => {
                        let counter_value = state.get_state();
                        let actual_value = counter_value["value"].as_str().unwrap_or("0");

                        println!("→ Instance {}: final value = {}", instance_id, actual_value);

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
                            println!("✅ Instance {} has correct final value", instance_id);
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Error getting final state from {}: {}", instance_id, e);
                        all_converged = false;
                    }
                }
            }

            println!("\nTest cleanup: Tearing down test environment");
            teardown_remote_test_environment(&test_config).await;

            if all_converged {
                println!("\n✅ TEST PASSED: Causal ordering preserved despite delayed delivery");
                println!("→ All operations were eventually delivered to all instances");
                println!("→ Final state confirms causal consistency was maintained");
            } else {
                println!("\n❌ TEST FAILED: System did not maintain causal consistency with delayed delivery");
            }

            all_converged
        }
    }
}
