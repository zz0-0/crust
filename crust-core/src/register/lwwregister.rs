use crate::{
    crdt_type::{CmRDT, CvRDT},
    crdt_validation::{CmRDTValidation, CvRDTValidation},
    get_current_timestamp,
    text_operation::TextOperation,
};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct LWWRegister<K> {
    value: Option<K>,
    timestamp: u128,
    replica_id: Uuid,
}

#[derive(Clone)]
pub enum Operation<K> {
    Set(K, u128, Uuid),
}

impl<K> LWWRegister<K>
where
    K: Clone + Serialize + for<'a> Deserialize<'a>,
{
    pub fn new() -> Self {
        Self {
            value: None,
            timestamp: get_current_timestamp(),
            replica_id: Uuid::new_v4(),
        }
    }

    pub fn to_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }

    pub fn to_crdt(str: String) -> Result<Self, serde_json::Error> {
        serde_json::from_str(&str)
    }

    pub fn update(&mut self, value: K) {
        self.value = value.into();
        self.timestamp = get_current_timestamp();
    }
}

impl<K> CmRDT for LWWRegister<K>
where
    K: Clone,
{
    type Op = Operation<K>;
    type Value = K;

    fn apply(&mut self, op: &Self::Op) {
        match *op {
            Operation::Set(ref value, timestamp, ref replica_id) => {
                self.merge(&LWWRegister {
                    value: value.clone().into(),
                    timestamp,
                    replica_id: *replica_id,
                });
            }
        }
    }

    fn convert_operation(&self, op: TextOperation<K>) -> Vec<Self::Op> {
        match op {
            TextOperation::Insert { position: _, value } => vec![Operation::Set(
                value,
                get_current_timestamp(),
                Uuid::new_v4(),
            )],
            TextOperation::Delete {
                position: _,
                value: _,
            } => todo!(),
        }
    }
}

impl<K> CvRDT for LWWRegister<K>
where
    K: Clone,
{
    fn merge(&mut self, other: &Self) {
        if self.timestamp < other.timestamp
            || (other.timestamp == self.timestamp && other.replica_id > self.replica_id)
        {
            self.value = other.value.clone();
            self.timestamp = other.timestamp;
            self.replica_id = other.replica_id
        }
    }
}

impl<K> CvRDTValidation<LWWRegister<K>> for LWWRegister<K>
where
    K: Clone + PartialEq + Debug,
{
    fn cvrdt_associativity(a: LWWRegister<K>, b: LWWRegister<K>, c: LWWRegister<K>) -> bool {
        let mut ab_c = a.clone();
        ab_c.merge(&b);
        let mut bc = b.clone();
        bc.merge(&c);
        ab_c.merge(&c);
        let mut a_bc = a.clone();
        a_bc.merge(&bc);
        println!("{:?} {:?}", ab_c, a_bc);
        ab_c == a_bc
    }

    fn cvrdt_commutativity(a: LWWRegister<K>, b: LWWRegister<K>) -> bool {
        let mut a_b = a.clone();
        a_b.merge(&b);
        let mut b_a = b.clone();
        b_a.merge(&a);
        println!("{:?} {:?}", a_b, b_a);
        a_b == b_a
    }

    fn cvrdt_idempotence(a: LWWRegister<K>) -> bool {
        let mut a_a = a.clone();
        a_a.merge(&a);
        println!("{:?} {:?}", a_a, a);
        a_a == a
    }
}

impl<K> CmRDTValidation<LWWRegister<K>> for LWWRegister<K>
where
    K: Clone + PartialEq + Debug,
{
    fn cmrdt_commutativity(
        a: LWWRegister<K>,
        op1: <LWWRegister<K> as CmRDT>::Op,
        op2: <LWWRegister<K> as CmRDT>::Op,
    ) -> bool {
        let mut a1 = a.clone();
        a1.apply(&op1);
        a1.apply(&op2);
        let mut a2 = a.clone();
        a2.apply(&op2);
        a2.apply(&op1);
        println!("{:?} {:?}", a1, a2);
        a1 == a2
    }

    fn cmrdt_idempotence(a: LWWRegister<K>, op1: <LWWRegister<K> as CmRDT>::Op) -> bool {
        let mut a1 = a.clone();
        a1.apply(&op1);
        a1.apply(&op1);
        let mut a2 = a.clone();
        a2.apply(&op1);
        println!("{:?} {:?}", a1, a2);
        a1 == a2
    }

    fn cmrdt_sequential_consistency(
        a: LWWRegister<K>,
        ops: Vec<<LWWRegister<K> as CmRDT>::Op>,
    ) -> bool {
        let mut a1 = a.clone();
        for op in &ops {
            a1.apply(op);
        }

        let mut rng = rand::thread_rng();
        let mut ops_permuted = ops.clone();
        for _ in 0..5 {
            ops_permuted.shuffle(&mut rng);
            let mut a2 = a.clone();
            for op in &ops_permuted {
                a2.apply(op);
            }
            if a1 != a2 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cvrdt_validation() {
        let a = LWWRegister {
            value: "a".to_string().into(),
            timestamp: 1,
            replica_id: Uuid::new_v4(),
        };
        let b = LWWRegister {
            value: "b".to_string().into(),
            timestamp: 2,
            replica_id: Uuid::new_v4(),
        };
        let c = LWWRegister {
            value: "c".to_string().into(),
            timestamp: 3,
            replica_id: Uuid::new_v4(),
        };
        assert!(LWWRegister::<String>::cvrdt_associativity(
            a.clone(),
            b.clone(),
            c.clone()
        ));
        assert!(LWWRegister::<String>::cvrdt_commutativity(
            a.clone(),
            b.clone()
        ));
        assert!(LWWRegister::<String>::cvrdt_idempotence(a.clone()));
    }

    #[test]
    fn test_cmrdt_validation() {
        let a = LWWRegister::<String> {
            value: "a".to_string().into(),
            timestamp: 1,
            replica_id: Uuid::new_v4(),
        };
        let op1 = Operation::Set("b".to_string(), 2, Uuid::new_v4());
        let op2 = Operation::Set("c".to_string(), 3, Uuid::new_v4());
        assert!(LWWRegister::<String>::cmrdt_commutativity(
            a.clone(),
            op1.clone(),
            op2.clone()
        ));
        assert!(LWWRegister::<String>::cmrdt_idempotence(
            a.clone(),
            op1.clone()
        ));
        assert!(LWWRegister::<String>::cmrdt_sequential_consistency(
            a.clone(),
            vec![op1.clone(), op2.clone()]
        ));
    }
}
