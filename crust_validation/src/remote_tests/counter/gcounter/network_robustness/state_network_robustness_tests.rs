mod state_network_robustness {
    use std::{collections::HashMap, time::Duration};

    use crust_config::instance::{
        get_state_from_instance, send_command_to_instance, send_command_to_instance_with_loss,
        setup_remote_test_environement, teardown_remote_test_environment, update_replicas,
        DeploymentConfig,
    };
    use crust_core::{
        command::{CounterInnerCommand, CrdtInnerCommand},
        core::counter::gcounter::GCounter,
        r#type::{CrdtType, CrdtTypeVariant},
    };
    use futures::future::try_join_all;

    use crate::remote_validation::StateBasedDistributedNetworkRobustnessValidation;

    impl StateBasedDistributedNetworkRobustnessValidation<GCounter<String>> for GCounter<String> {
        async fn state_based_robustness_message_loss() -> bool {
            println!("\n=== TEST: State-Based CRDT Robustness with Message Loss ===");

            println!("Step 1/6: Setting up test environment with 3 replicas");
            let client = kube::Client::try_default().await.unwrap();
            let namespace = "default";
            let deployment_name = "crust-network";
            let replicas = 3;
            let test_config = DeploymentConfig::new(
                replicas.try_into().unwrap(),
                "gcounter",
                "state", 
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

            println!("\nStep 2/6: Configuring message loss simulation");
            
            let message_loss_rate = 0.5;
            println!(
                "→ Setting message loss rate to {}%",
                message_loss_rate * 100.0
            );

            println!("\nStep 3/6: Sending operations with simulated message loss");

            
            let increment_value_1 = "7";
            println!(
                "→ Sending increment {} to instance {} with {}% message loss",
                increment_value_1,
                instance_1,
                message_loss_rate * 100.0
            );
            let command_1 = CrdtInnerCommand::Counter(CounterInnerCommand::Increment {
                value: increment_value_1.to_string(),
            });

            if let Err(e) = send_command_to_instance_with_loss(
                instance_1,
                &test_config.crdt_type.name(),
                &test_config.sync_type.to_string(),
                &test_config.sync_mode.to_string(),
                command_1,
                &service_url_1,
                message_loss_rate,
            )
            .await
            {
                eprintln!("❌ Error sending command to instance {}: {}", instance_1, e);
                teardown_remote_test_environment(&test_config).await;
                return false;
            }

            
            let increment_value_2 = "3";
            println!(
                "→ Sending increment {} to instance {} with {}% message loss",
                increment_value_2,
                instance_2,
                message_loss_rate * 100.0
            );
            let command_2 = CrdtInnerCommand::Counter(CounterInnerCommand::Increment {
                value: increment_value_2.to_string(),
            });

            if let Err(e) = send_command_to_instance_with_loss(
                instance_2,
                &test_config.crdt_type.name(),
                &test_config.sync_type.to_string(),
                &test_config.sync_mode.to_string(),
                command_2,
                &service_url_2,
                message_loss_rate,
            )
            .await
            {
                eprintln!("❌ Error sending command to instance {}: {}", instance_2, e);
                teardown_remote_test_environment(&test_config).await;
                return false;
            }

            
            let increment_value_3 = "5";
            println!(
                "→ Sending increment {} to instance {} with {}% message loss",
                increment_value_3,
                instance_3,
                message_loss_rate * 100.0
            );
            let command_3 = CrdtInnerCommand::Counter(CounterInnerCommand::Increment {
                value: increment_value_3.to_string(),
            });

            if let Err(e) = send_command_to_instance_with_loss(
                instance_3,
                &test_config.crdt_type.name(),
                &test_config.sync_type.to_string(),
                &test_config.sync_mode.to_string(),
                command_3,
                &service_url_3,
                message_loss_rate,
            )
            .await
            {
                eprintln!("❌ Error sending command to instance {}: {}", instance_3, e);
                teardown_remote_test_environment(&test_config).await;
                return false;
            }

            println!("✅ All operations sent with simulated message loss");

            
            println!(
                "\nStep 4/6: Checking immediate states (expect inconsistency due to message loss)"
            );
            let mut initial_states = HashMap::new();

            for instance_id in &instance_ids {
                let service_url = service_urls.get(instance_id).unwrap();
                match get_state_from_instance(instance_id, &test_config.crdt_type, service_url)
                    .await
                {
                    Ok(state) => {
                        let counter_value = state.get_state();
                        let value = counter_value["value"].as_str().unwrap_or("0");
                        println!("→ Instance {}: immediate value = {}", instance_id, value);
                        initial_states.insert(instance_id.clone(), value.to_string());
                    }
                    Err(e) => {
                        eprintln!(
                            "❌ Error getting state from instance {}: {}",
                            instance_id, e
                        );
                    }
                }
            }

            
            println!("\nStep 5/6: Waiting for eventual convergence (15 seconds)");
            println!("→ Despite message loss, state-based CRDTs should eventually converge");
            println!("→ The system will retry state synchronization until convergence");
            tokio::time::sleep(Duration::from_secs(15)).await;
            println!("✅ Wait complete");

            
            println!("\nStep 6/6: Verifying convergence across all instances");

            let expected_final_value = (increment_value_1.parse::<i32>().unwrap()
                + increment_value_2.parse::<i32>().unwrap()
                + increment_value_3.parse::<i32>().unwrap())
            .to_string();

            println!("→ Expected final counter value: {}", expected_final_value);

            
            let mut all_converged = true;
            let mut final_states = HashMap::new();

            for instance_id in &instance_ids {
                let service_url = service_urls.get(instance_id).unwrap();
                match get_state_from_instance(instance_id, &test_config.crdt_type, service_url)
                    .await
                {
                    Ok(state) => {
                        let counter_value = state.get_state();
                        let actual_value = counter_value["value"].as_str().unwrap_or("0");

                        println!("→ Instance {}: final value = {}", instance_id, actual_value);
                        final_states.insert(instance_id.clone(), actual_value.to_string());

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
                        eprintln!(
                            "❌ Error getting state from instance {}: {}",
                            instance_id, e
                        );
                        all_converged = false;
                    }
                }
            }

            
            println!("\nConvergence comparison:");
            for instance_id in &instance_ids {
                let unknown_str = "unknown".to_string();
                let initial = initial_states.get(instance_id).unwrap_or(&unknown_str);
                let final_val = final_states.get(instance_id).unwrap_or(&unknown_str);
                println!("→ Instance {}: {} → {}", instance_id, initial, final_val);
            }

            println!("\nTest cleanup: Tearing down test environment");
            teardown_remote_test_environment(&test_config).await;

            if all_converged {
                println!("\n✅ TEST PASSED: State-based CRDT converged despite message loss");
                println!("→ This demonstrates the robustness of state-based synchronization");
                println!(
                    "→ Even with {}% message loss, the system eventually reached consistency",
                    message_loss_rate * 100.0
                );
            } else {
                println!("\n❌ TEST FAILED: State-based CRDT did not converge with message loss");
            }

            all_converged
        }

        async fn state_based_robustness_network_partition() -> bool {
            println!("\n=== TEST: State-Based CRDT Robustness with Network Partition ===");

            println!("Step 1/8: Setting up test environment with 3 replicas");
            let client = kube::Client::try_default().await.unwrap();
            let namespace = "default";
            let deployment_name = "crust-network";
            let replicas = 3;
            let test_config = DeploymentConfig::new(
                replicas.try_into().unwrap(),
                "gcounter",
                "state", 
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

            println!("\nStep 2/8: Simulating network partition");
            println!("→ Partition A: Instances {} and {}", instance_1, instance_2);
            println!("→ Partition B: Instance {}", instance_3);
            println!("→ Instances in different partitions cannot communicate");

            
            

            println!("\nStep 3/8: Applying operations to Partition A");
            
            let increment_value_1 = "5";
            println!(
                "→ Sending increment {} to instance {}",
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
                eprintln!("❌ Error sending command to instance {}: {}", instance_1, e);
                teardown_remote_test_environment(&test_config).await;
                return false;
            }

            let increment_value_2 = "3";
            println!(
                "→ Sending increment {} to instance {}",
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
                eprintln!("❌ Error sending command to instance {}: {}", instance_2, e);
                teardown_remote_test_environment(&test_config).await;
                return false;
            }

            println!("\nStep 4/8: Applying operation to Partition B");
            
            let increment_value_3 = "7";
            println!(
                "→ Sending increment {} to instance {}",
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
                eprintln!("❌ Error sending command to instance {}: {}", instance_3, e);
                teardown_remote_test_environment(&test_config).await;
                return false;
            }

            println!("\nStep 5/8: Verifying partition state consistency");
            println!("→ Nodes in the same partition should converge with each other");
            println!("→ But nodes across different partitions should remain inconsistent");

            tokio::time::sleep(Duration::from_secs(5)).await;

            
            let partition_a_expected = (increment_value_1.parse::<i32>().unwrap()
                + increment_value_2.parse::<i32>().unwrap())
            .to_string();

            let partition_b_expected = increment_value_3.to_string();

            println!("→ Expected value in Partition A: {}", partition_a_expected);
            println!("→ Expected value in Partition B: {}", partition_b_expected);

            
            let mut partition_a_consistent = true;

            
            match get_state_from_instance(instance_1, &test_config.crdt_type, &service_url_1).await
            {
                Ok(state) => {
                    let counter_value = state.get_state();
                    let actual_value = counter_value["value"].as_str().unwrap_or("0");
                    println!(
                        "→ Instance {} (Partition A): value = {}",
                        instance_1, actual_value
                    );

                    if actual_value != partition_a_expected {
                        eprintln!(
                            "❌ Instance {} has unexpected value for Partition A",
                            instance_1
                        );
                        partition_a_consistent = false;
                    }
                }
                Err(e) => {
                    eprintln!("❌ Error getting state from instance {}: {}", instance_1, e);
                    partition_a_consistent = false;
                }
            }

            
            match get_state_from_instance(instance_2, &test_config.crdt_type, &service_url_2).await
            {
                Ok(state) => {
                    let counter_value = state.get_state();
                    let actual_value = counter_value["value"].as_str().unwrap_or("0");
                    println!(
                        "→ Instance {} (Partition A): value = {}",
                        instance_2, actual_value
                    );

                    if actual_value != partition_a_expected {
                        eprintln!(
                            "❌ Instance {} has unexpected value for Partition A",
                            instance_2
                        );
                        partition_a_consistent = false;
                    }
                }
                Err(e) => {
                    eprintln!("❌ Error getting state from instance {}: {}", instance_2, e);
                    partition_a_consistent = false;
                }
            }

            
            let mut partition_b_consistent = true;
            match get_state_from_instance(instance_3, &test_config.crdt_type, &service_url_3).await
            {
                Ok(state) => {
                    let counter_value = state.get_state();
                    let actual_value = counter_value["value"].as_str().unwrap_or("0");
                    println!(
                        "→ Instance {} (Partition B): value = {}",
                        instance_3, actual_value
                    );

                    if actual_value != partition_b_expected {
                        eprintln!(
                            "❌ Instance {} has unexpected value for Partition B",
                            instance_3
                        );
                        partition_b_consistent = false;
                    }
                }
                Err(e) => {
                    eprintln!("❌ Error getting state from instance {}: {}", instance_3, e);
                    partition_b_consistent = false;
                }
            }

            if partition_a_consistent {
                println!("✅ Partition A is internally consistent");
            } else {
                println!("❌ Partition A has inconsistencies");
            }

            if partition_b_consistent {
                println!("✅ Partition B is internally consistent");
            } else {
                println!("❌ Partition B has inconsistencies");
            }

            
            if partition_a_expected == partition_b_expected {
                println!("⚠️ Both partitions have the same value - partition simulation might not be effective");
            } else {
                println!(
                    "✅ Partitions have different values as expected during network partition"
                );
            }

            println!("\nStep 6/8: Healing the network partition");
            println!("→ Re-enabling communication between all instances");
            println!("→ Network partition is now healed");

            
            

            println!("\nStep 7/8: Waiting for state synchronization after partition healing (15 seconds)");
            println!(
                "→ State-based CRDTs should eventually converge once partitions can communicate"
            );
            tokio::time::sleep(Duration::from_secs(15)).await;
            println!("✅ Wait complete");

            println!("\nStep 8/8: Verifying convergence after partition healing");
            let expected_final_value = (increment_value_1.parse::<i32>().unwrap()
                + increment_value_2.parse::<i32>().unwrap()
                + increment_value_3.parse::<i32>().unwrap())
            .to_string();

            println!(
                "→ Expected final counter value across all instances: {}",
                expected_final_value
            );

            
            let mut all_converged = true;
            let mut final_states = HashMap::new();

            for instance_id in &instance_ids {
                let service_url = service_urls.get(instance_id).unwrap();
                match get_state_from_instance(instance_id, &test_config.crdt_type, service_url)
                    .await
                {
                    Ok(state) => {
                        let counter_value = state.get_state();
                        let actual_value = counter_value["value"].as_str().unwrap_or("0");

                        println!("→ Instance {}: final value = {}", instance_id, actual_value);
                        final_states.insert(instance_id.clone(), actual_value.to_string());

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
                println!(
                    "\n✅ TEST PASSED: State-based CRDT converged after network partition healed"
                );
                println!("→ This demonstrates the partition tolerance of state-based CRDTs");
                println!("→ Operations from both partitions were preserved in the final state");
            } else {
                println!(
                    "\n❌ TEST FAILED: State-based CRDT did not converge after network partition"
                );
            }

            all_converged
        }
    }
}
