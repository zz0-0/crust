mod tests {
    use crust_core::{
        core::counter::gcounter::GCounter,
        operation::CounterOperation,
        sync::{Crdt, DeltaBased, OperationBased, StateBased},
    };

    use crate::local_validation::{
        DeltaBasedValidation, OperationBasedValidation, StateBasedValidation,
    };

    impl StateBasedValidation<GCounter<String>> for GCounter<String> {
        fn state_associativity() -> bool {
            let mut a = GCounter::<String>::new();
            let mut b = GCounter::<String>::new();
            let mut c = GCounter::<String>::new();
            a.increment("a".to_string());
            b.increment("b".to_string());
            c.increment("c".to_string());
            let ab_c = a.merge(&b).merge(&c);
            let a_bc = a.merge(&b.merge(&c));
            ab_c == a_bc
        }

        fn state_commutativity() -> bool {
            let mut a = GCounter::<String>::new();
            let mut b = GCounter::<String>::new();
            a.increment("a".to_string());
            b.increment("b".to_string());
            let ab = a.merge(&b);
            let ba = b.merge(&a);
            ab == ba
        }

        fn state_idempotence() -> bool {
            let mut a = GCounter::<String>::new();
            a.increment("a".to_string());
            let aa = a.merge(&a.clone());
            aa == a
        }

        fn state_monotonicity() -> bool {
            let mut a = GCounter::<String>::new();
            let mut b = GCounter::<String>::new();
            a.increment("a".to_string());
            let a_before = a.clone();
            b.increment("b".to_string());
            let a_after = a.merge(&b);
            let value_before = a_before.get_state().counter.values().sum::<u64>();
            let value_after = a_after.get_state().counter.values().sum::<u64>();
            value_after >= value_before
        }
    }

    impl OperationBasedValidation<GCounter<String>> for GCounter<String> {
        fn operation_commutativity() -> bool {
            let mut a = GCounter::<String>::new();
            let mut b = GCounter::<String>::new();
            let op1 = CounterOperation::Increment {
                value: "a".to_string(),
            };
            let op2 = CounterOperation::Increment {
                value: "b".to_string(),
            };
            a.apply(&op1).apply(&op2);
            b.apply(&op2).apply(&op1);
            a == b
        }

        fn operation_delivery_precondition() -> bool {
            let counter = GCounter::<String>::new();
            let mut applied_successfully = true;
            let ops = vec![
                CounterOperation::Increment {
                    value: "a".to_string(),
                },
                CounterOperation::Increment {
                    value: "b".to_string(),
                },
                CounterOperation::Increment {
                    value: "c".to_string(),
                },
            ];
            for op in ops {
                let mut test_counter = counter.clone();
                let before = test_counter.clone();
                test_counter.apply(&op);
                if test_counter == before {
                    applied_successfully = false;
                    break;
                }
            }
            applied_successfully
        }

        fn operation_effect_relation() -> bool {
            let mut counter = GCounter::<String>::new();
            let test_cases = vec![
                ("a".to_string(), 1),
                ("b".to_string(), 1),
                ("c".to_string(), 2),
            ];
            let mut expected_values = std::collections::HashMap::new();
            for (key, _) in &test_cases {
                expected_values.entry(key.clone()).or_insert(0);
            }
            for (key, _) in test_cases {
                let op = CounterOperation::Increment { value: key.clone() };
                counter.apply(&op);
                let entry = expected_values.entry(key.clone()).or_insert(0);
                *entry += 1;
                let actual_value = *counter.get_state().counter.get(&key).unwrap_or(&0);
                let expected_value = *expected_values.get(&key).unwrap_or(&0);
                if actual_value != expected_value {
                    return false;
                }
            }
            true
        }
    }

    impl DeltaBasedValidation<GCounter<String>> for GCounter<String> {
        fn delta_associativity() -> bool {
            let mut counter = GCounter::<String>::new();
            counter.increment("a".to_string());
            let delta_a = counter.generate_delta();
            counter.increment("b".to_string());
            let delta_b = counter.generate_delta();
            counter.increment("c".to_string());
            let delta_c = counter.generate_delta();
            let mut counter1 = GCounter::<String>::new();
            counter1 = counter1.merge_delta(&delta_a);
            counter1 = counter1.merge_delta(&delta_b);
            counter1 = counter1.merge_delta(&delta_c);
            let mut counter2 = GCounter::<String>::new();
            let mut temp_counter = GCounter::<String>::new();
            temp_counter = temp_counter.merge_delta(&delta_b);
            temp_counter = temp_counter.merge_delta(&delta_c);
            let combined_delta = temp_counter.generate_delta();
            counter2 = counter2.merge_delta(&delta_a);
            counter2 = counter2.merge_delta(&combined_delta);
            counter1 == counter2
        }

        fn delta_commutativity() -> bool {
            let mut a = GCounter::<String>::new();
            a.increment("a".to_string());
            let delta1 = a.generate_delta();
            a.increment("b".to_string());
            let delta2 = a.generate_delta();
            let a1 = a.clone().merge_delta(&delta1).merge_delta(&delta2);
            let a2 = a.clone().merge_delta(&delta2).merge_delta(&delta1);
            a1 == a2
        }

        fn delta_idempotence() -> bool {
            let mut a = GCounter::<String>::new();
            a.increment("a".to_string());
            let delta = a.generate_delta();
            let a1 = a.clone().merge_delta(&delta).merge_delta(&delta);
            let a2 = a.clone().merge_delta(&delta);
            a1 == a2
        }

        fn delta_state_composability() -> bool {
            let mut counter_a = GCounter::<String>::new();
            let mut counter_b = GCounter::<String>::new();
            counter_a.increment("x".to_string());
            counter_a.increment("y".to_string());
            let delta = counter_a.generate_delta();
            counter_b.increment("z".to_string());
            let mut path1 = counter_b.clone();
            path1 = path1.merge_delta(&delta);
            path1 = path1.merge(&counter_a);
            let mut path2 = counter_b.clone();
            path2 = path2.merge(&counter_a);
            path1 == path2
        }
    }

    #[test]
    fn test_gcounter_state_associativity() {
        assert!(GCounter::<String>::state_associativity());
    }

    #[test]
    fn test_gcounter_state_commutativity() {
        assert!(GCounter::<String>::state_commutativity());
    }

    #[test]
    fn test_gcounter_state_idempotence() {
        assert!(GCounter::<String>::state_idempotence());
    }

    #[test]
    fn test_gcounter_state_monotonicity() {
        assert!(GCounter::<String>::state_monotonicity());
    }

    #[test]
    fn test_gcounter_operation_commutativity() {
        assert!(GCounter::<String>::operation_commutativity());
    }

    #[test]
    fn test_gcounter_operation_delivery_precondition() {
        assert!(GCounter::<String>::operation_delivery_precondition());
    }

    #[test]
    fn test_gcounter_operation_effect_relation() {
        assert!(GCounter::<String>::operation_effect_relation());
    }

    #[test]
    fn test_gcounter_delta_associativity() {
        assert!(GCounter::<String>::delta_associativity());
    }

    #[test]
    fn test_gcounter_delta_commutativity() {
        assert!(GCounter::<String>::delta_commutativity());
    }

    #[test]
    fn test_gcounter_delta_idempotence() {
        assert!(GCounter::<String>::delta_idempotence());
    }

    #[test]
    fn test_gcounter_delta_state_composability() {
        assert!(GCounter::<String>::delta_state_composability());
    }
}
