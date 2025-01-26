use crate::{
    crdt_type::{CmRDT, CvRDT, Delta},
    crdt_validation::{CmRDTValidation, CvRDTValidation, DeltaValidation},
    get_current_timestamp,
    text_operation::TextOperation,
};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug};
use std::{collections::HashSet, hash::Hash};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct MVRegister<K>
where
    K: Eq + Hash,
{
    values: HashMap<Uuid, (K, u128)>,
    tombstones: HashSet<(Uuid, u128)>,
    previous_values: HashMap<Uuid, (K, u128)>,
    previous_tombstones: HashSet<(Uuid, u128)>,
}

#[derive(Clone)]
pub enum Operation<K> {
    Write { value: K, replica_id: Uuid },
}

impl<K> MVRegister<K>
where
    K: Eq + Hash + Serialize + for<'a> Deserialize<'a>,
{
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            tombstones: HashSet::new(),
            previous_values: HashMap::new(),
            previous_tombstones: HashSet::new(),
        }
    }

    pub fn to_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }

    pub fn to_crdt(str: String) -> Result<Self, serde_json::Error> {
        serde_json::from_str(&str)
    }

    pub fn to_delta(str: String) -> Result<Self, serde_json::Error> {
        serde_json::from_str(&str)
    }

    pub fn write(&mut self, replica_id: Uuid, value: K) {
        let timestamp = self.values.get(&replica_id).map(|(_, ts)| *ts);
        if let Some(ts) = timestamp {
            self.values.remove(&replica_id);
            self.tombstones.insert((replica_id.clone(), ts));
        }
        self.values
            .insert(replica_id, (value, get_current_timestamp()));
    }

    pub fn name(&self) -> String {
        "MVRegister".to_string()
    }
}

impl<K> CmRDT for MVRegister<K>
where
    K: Eq + Hash + Clone + Serialize + for<'a> Deserialize<'a>,
{
    type Op = Operation<K>;
    type Value = K;

    fn apply(&mut self, op: &Self::Op) {
        match *op {
            Operation::Write {
                ref value,
                replica_id,
            } => {
                self.write(replica_id, value.clone());
            }
        }
    }

    fn convert_operation(&self, op: TextOperation<K>) -> Vec<Self::Op> {
        match op {
            TextOperation::Insert { position: _, value } => {
                vec![Operation::Write {
                    value,
                    replica_id: Uuid::new_v4(),
                }]
            }
            TextOperation::Delete {
                position: _,
                value: _,
            } => todo!(),
        }
    }
}

impl<K> CvRDT for MVRegister<K>
where
    K: Eq + Hash + Clone,
{
    fn merge(&mut self, other: &Self) {
        self.tombstones.extend(other.tombstones.iter());
        for (replica_id, (value, timestamp)) in &other.values {
            if !self.tombstones.contains(&(*replica_id, *timestamp)) {
                match self.values.get(replica_id) {
                    Some((_, current_timestamp)) if timestamp > current_timestamp => {
                        self.values.insert(*replica_id, (value.clone(), *timestamp));
                    }
                    None => {
                        self.values.insert(*replica_id, (value.clone(), *timestamp));
                    }
                    _ => {}
                }
            }
        }

        self.values.retain(|replica_id, (_, timestamp)| {
            !self.tombstones.contains(&(*replica_id, *timestamp))
        });
    }
}

impl<K> Delta for MVRegister<K>
where
    K: Eq + Hash + Clone + Serialize + for<'a> Deserialize<'a>,
{
    type De = MVRegister<K>;

    fn generate_delta(&self) -> Self::De {
        let mut delta = MVRegister::new();
        for (replica_id, (value, timestamp)) in &self.values {
            match self.previous_values.get(replica_id) {
                Some((_, since_timestamp)) if timestamp > since_timestamp => {
                    delta
                        .values
                        .insert(*replica_id, (value.clone(), *timestamp));
                }
                None => {
                    delta
                        .values
                        .insert(*replica_id, (value.clone(), *timestamp));
                }
                _ => {}
            }
        }
        for tombstone in &self.tombstones {
            if !self.previous_tombstones.contains(tombstone) {
                delta.tombstones.insert(*tombstone);
            }
        }
        delta
    }

    fn apply_delta(&mut self, delta: &Self::De) {
        self.merge(&delta);
    }
}

impl<K> CvRDTValidation<MVRegister<K>> for MVRegister<K>
where
    K: Eq + Hash + Clone + Debug,
{
    fn cvrdt_associativity(a: MVRegister<K>, b: MVRegister<K>, c: MVRegister<K>) -> bool {
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

    fn cvrdt_commutativity(a: MVRegister<K>, b: MVRegister<K>) -> bool {
        let mut a_b = a.clone();
        a_b.merge(&b);
        let mut b_a = b.clone();
        b_a.merge(&a);
        println!("{:?} {:?}", a_b, b_a);
        a_b == b_a
    }

    fn cvrdt_idempotence(a: MVRegister<K>) -> bool {
        let mut a_a = a.clone();
        a_a.merge(&a);
        println!("{:?} {:?}", a_a, a);
        a_a == a
    }
}

impl<K> CmRDTValidation<MVRegister<K>> for MVRegister<K>
where
    K: Eq + Hash + Clone,
    MVRegister<K>: CmRDT + Debug,
{
    fn cmrdt_commutativity(
        a: MVRegister<K>,
        op1: <MVRegister<K> as CmRDT>::Op,
        op2: <MVRegister<K> as CmRDT>::Op,
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

    fn cmrdt_idempotence(a: MVRegister<K>, op1: <MVRegister<K> as CmRDT>::Op) -> bool {
        let mut a1 = a.clone();
        a1.apply(&op1);
        a1.apply(&op1);
        let mut a2 = a.clone();
        a2.apply(&op1);
        println!("{:?} {:?}", a1, a2);
        a1 == a2
    }

    fn cmrdt_sequential_consistency(
        a: MVRegister<K>,
        ops: Vec<<MVRegister<K> as CmRDT>::Op>,
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

impl<K> DeltaValidation<MVRegister<K>> for MVRegister<K>
where
    K: Eq + Hash + Clone + Serialize + for<'a> Deserialize<'a>,
    MVRegister<K>: Delta<De = MVRegister<K>> + Debug,
{
    fn delta_associativity(
        a: MVRegister<K>,
        de1: <MVRegister<K> as Delta>::De,
        de2: <MVRegister<K> as Delta>::De,
        de3: <MVRegister<K> as Delta>::De,
    ) -> bool {
        let mut a1 = a.clone();
        a1.apply_delta(&de1.clone());
        a1.apply_delta(&de2.clone());
        a1.apply_delta(&de3.clone());

        let mut a2 = a.clone();
        let mut combined_delta = MVRegister::new();

        // Merge values from de2
        for (replica_id, (value, timestamp)) in &de2.values {
            combined_delta
                .values
                .insert(*replica_id, (value.clone(), *timestamp));
        }
        combined_delta
            .tombstones
            .extend(de2.tombstones.iter().cloned());

        // Merge values from de3
        for (replica_id, (value, timestamp)) in &de3.values {
            match combined_delta.values.get(replica_id) {
                Some((_, current_timestamp)) if timestamp > current_timestamp => {
                    combined_delta
                        .values
                        .insert(*replica_id, (value.clone(), *timestamp));
                }
                None => {
                    combined_delta
                        .values
                        .insert(*replica_id, (value.clone(), *timestamp));
                }
                _ => {}
            }
        }
        combined_delta
            .tombstones
            .extend(de3.tombstones.iter().cloned());

        a2.apply_delta(&de1);
        a2.apply_delta(&combined_delta);

        println!("{:?} {:?}", a1, a2);
        a1 == a2
    }

    fn delta_commutativity(
        a: MVRegister<K>,
        de1: <MVRegister<K> as Delta>::De,
        de2: <MVRegister<K> as Delta>::De,
    ) -> bool {
        let mut a1 = a.clone();
        a1.apply_delta(&de1.clone());
        a1.apply_delta(&de2.clone());
        let mut a2 = a.clone();
        a2.apply_delta(&de2);
        a2.apply_delta(&de1);
        println!("{:?} {:?}", a1, a2);
        a1 == a2
    }

    fn delta_idempotence(a: MVRegister<K>, de1: <MVRegister<K> as Delta>::De) -> bool {
        let mut a1 = a.clone();
        a1.apply_delta(&de1.clone());
        a1.apply_delta(&de1.clone());
        let mut a2 = a.clone();
        a2.apply_delta(&de1.clone());
        println!("{:?} {:?}", a1, a2);
        a1 == a2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cvrdt_validation() {
        let mut a = MVRegister::<String>::new();
        let mut b = MVRegister::<String>::new();
        let mut c = MVRegister::<String>::new();

        a.write(Uuid::new_v4(), "a".to_string());
        b.write(Uuid::new_v4(), "b".to_string());
        c.write(Uuid::new_v4(), "c".to_string());

        assert!(MVRegister::<String>::cvrdt_associativity(
            a.clone(),
            b.clone(),
            c.clone()
        ));
        assert!(MVRegister::<String>::cvrdt_commutativity(
            a.clone(),
            b.clone()
        ));
        assert!(MVRegister::<String>::cvrdt_idempotence(a.clone()));
    }

    #[test]
    fn test_cmrdt_validation() {
        let mut a = MVRegister::<String>::new();
        a.write(Uuid::new_v4(), "a".to_string());
        let op1 = Operation::Write {
            value: "b".to_string(),
            replica_id: Uuid::new_v4(),
            // Use same timestamp for operation
        };
        let op2 = Operation::Write {
            value: "c".to_string(),
            replica_id: Uuid::new_v4(),
            // Use same timestamp for operation
        };
        assert!(MVRegister::<String>::cmrdt_commutativity(
            a.clone(),
            op1.clone(),
            op2.clone()
        ));
        assert!(MVRegister::<String>::cmrdt_idempotence(
            a.clone(),
            op1.clone()
        ));
        assert!(MVRegister::<String>::cmrdt_sequential_consistency(
            a.clone(),
            vec![op1.clone(), op2.clone()]
        ));
    }

    #[test]
    fn test_delta_validation() {
        let mut a = MVRegister::<String>::new();
        let mut b = MVRegister::<String>::new();

        // Create initial state
        a.write(Uuid::new_v4(), "a".to_string());

        // Generate meaningful deltas
        let de1 = a.clone(); // Use full state as delta

        // Create different state for second delta
        b.write(Uuid::new_v4(), "b".to_string());
        let de2 = b.clone();

        // Create different state for third delta
        let mut c = MVRegister::<String>::new();
        c.write(Uuid::new_v4(), "c".to_string());
        let de3 = c.clone();
        assert!(MVRegister::<String>::delta_associativity(
            a.clone(),
            de1.clone(),
            de2.clone(),
            de3.clone()
        ));
        assert!(MVRegister::<String>::delta_commutativity(
            a.clone(),
            de1.clone(),
            de2.clone()
        ));
        assert!(MVRegister::<String>::delta_idempotence(
            a.clone(),
            de1.clone()
        ));
    }
}
