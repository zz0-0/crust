use crate::{
    counter::{self, gcounter::GCounter, pncounter::PNCounter},
    graph::{self, awgraph::AWGraph, ggraph::GGraph, orgraph::ORGraph, tpgraph::TPGraph},
    register::{self, lwwregister::LWWRegister, mvregister::MVRegister},
    sequence::{self, logoot::Logoot, lseq::LSeq, rga::RGA},
    set::{self, awset::AWSet, gset::GSet, orset::ORSet, rwset::RWSet, tpset::TPSet},
    text_operation::TextOperation,
    tree::{self, merkle_dag_tree::MerkleDAGTree},
};
use serde::Serialize;
use std::hash::Hash;

pub trait CmRDT {
    type Op;
    type Value;

    fn apply(&mut self, op: Self::Op);
    fn convert_operation(&self, op: TextOperation<Self::Value>) -> Vec<Self::Op>;
}
pub trait CvRDT {
    type Value;

    fn merge(&mut self, other: &Self);
    fn convert_state(&self, op: TextOperation<Self::Value>);
}
pub trait Delta {
    type Value;

    fn generate_delta(&self, since: &Self) -> Self;
    fn apply_delta(&mut self, other: &Self);
    fn convert_delta(&self, op: TextOperation<Self::Value>);
}

#[derive(Clone)]
pub enum DataType<K>
where
    K: Eq + Hash + Clone + Ord,
{
    Gcounter(GCounter<K>),
    PNcounter(PNCounter<K>),
    AWGraph(AWGraph<K>),
    GGraph(GGraph<K>),
    ORGraph(ORGraph<K>),
    TPGraph(TPGraph<K>),
    // CMMap(CMMap<K, V>),
    // LWWMap(LWWMap<K, V>),
    // ORMap(ORMap<K, V>),
    // RMap(RMap<K, V>),
    LWWRegister(LWWRegister<K>),
    MVRegister(MVRegister<K>),
    Logoot(Logoot<K>),
    LSeq(LSeq<K>),
    RGA(RGA<K>),
    AWSet(AWSet<K>),
    GSet(GSet<K>),
    ORSet(ORSet<K>),
    RWSet(RWSet<K>),
    TPSet(TPSet<K>),
    MerkleDAGTree(MerkleDAGTree<K>),
}

pub enum Operation<K> {
    GCounter(counter::gcounter::Operation<K>),
    PNCounter(counter::pncounter::Operation<K>),
    AWGraph(graph::awgraph::Operation<K>),
    GGraph(graph::ggraph::Operation<K>),
    ORGraph(graph::orgraph::Operation<K>),
    TPGraph(graph::tpgraph::Operation<K>),
    // CMMap(map::cmmap::Operation<K, V>),
    // LWWMap(map::lwwmap::Operation<K, V>),
    // ORMap(map::ormap::Operation<K, V>),
    // RMap(map::rmap::Operation<K, V>),
    LWWRegister(register::lwwregister::Operation<K>),
    MVRegister(register::mvregister::Operation<K>),
    Logoot(sequence::logoot::Operation<K>),
    LSeq(sequence::lseq::Operation<K>),
    RGA(sequence::rga::Operation<K>),
    AWSet(set::awset::Operation<K>),
    GSet(set::gset::Operation<K>),
    ORSet(set::orset::Operation<K>),
    RWSet(set::rwset::Operation<K>),
    TPSet(set::tpset::Operation<K>),
    MerkleDAGTree(tree::merkle_dag_tree::Operation<K>),
}

impl<K> DataType<K>
where
    K: Eq + Hash + Clone + Ord + Serialize,
    DataType<K>: Clone,
{
    // pub fn get_or_create(crdt_type: String) -> Self {}

    pub fn new(crdt_type: String) -> Self {
        match crdt_type.as_str() {
            "gcounter" => DataType::Gcounter(GCounter::<K>::new()),
            "pncounter" => DataType::PNcounter(PNCounter::<K>::new()),
            "awgraph" => DataType::AWGraph(AWGraph::<K>::new()),
            "ggraph" => DataType::GGraph(GGraph::<K>::new()),
            "orgraph" => DataType::ORGraph(ORGraph::<K>::new()),
            "tpgraph" => DataType::TPGraph(TPGraph::<K>::new()),
            // "cmmap" => DataType::CMMap(CMMap::<K, V>::new()),
            // "lwwmap" => DataType::LWWMap(LWWMap::<K, V>::new()),
            // "ormap" => DataType::ORMap(ORMap::<K, V>::new()),
            // "rmap" => DataType::RMap(RMap::<K, V>::new()),
            "lwwregister" => DataType::LWWRegister(LWWRegister::<K>::new()),
            "mvregister" => DataType::MVRegister(MVRegister::<K>::new()),
            "logoot" => DataType::Logoot(Logoot::<K>::new()),
            "lseq" => DataType::LSeq(LSeq::<K>::new()),
            "rga" => DataType::RGA(RGA::<K>::new()),
            "awset" => DataType::AWSet(AWSet::<K>::new()),
            "gset" => DataType::GSet(GSet::<K>::new()),
            "orset" => DataType::ORSet(ORSet::<K>::new()),
            "rwset" => DataType::RWSet(RWSet::<K>::new()),
            "tpset" => DataType::TPSet(TPSet::<K>::new()),
            "merkledagtree" => DataType::MerkleDAGTree(MerkleDAGTree::<K>::new()),
            _ => panic!("Invalid CRDT type"),
        }
    }

    pub fn convert_operation(&self, text_operation: TextOperation<K>) -> Vec<Operation<K>> {
        match self {
            DataType::Gcounter(crdt) => crdt
                .convert_operation(text_operation)
                .into_iter()
                .map(|op| Operation::GCounter(op))
                .collect(),
            DataType::PNcounter(crdt) => crdt
                .convert_operation(text_operation)
                .into_iter()
                .map(|op| Operation::PNCounter(op))
                .collect(),
            DataType::AWGraph(crdt) => crdt
                .convert_operation(text_operation)
                .into_iter()
                .map(|op| Operation::AWGraph(op))
                .collect(),
            DataType::GGraph(crdt) => crdt
                .convert_operation(text_operation)
                .into_iter()
                .map(|op| Operation::GGraph(op))
                .collect(),
            DataType::ORGraph(crdt) => crdt
                .convert_operation(text_operation)
                .into_iter()
                .map(|op| Operation::ORGraph(op))
                .collect(),
            DataType::TPGraph(crdt) => crdt
                .convert_operation(text_operation)
                .into_iter()
                .map(|op| Operation::TPGraph(op))
                .collect(),
            // DataType::CMMap(crdt) => crdt.convert_operation(text_operation),
            // DataType::LWWMap(crdt) => crdt.convert_operation(text_operation),
            // DataType::ORMap(crdt) => crdt.convert_operation(text_operation),
            // DataType::RMap(crdt) => crdt.convert_operation(text_operation),
            DataType::LWWRegister(crdt) => crdt
                .convert_operation(text_operation)
                .into_iter()
                .map(|op| Operation::LWWRegister(op))
                .collect(),
            DataType::MVRegister(crdt) => crdt
                .convert_operation(text_operation)
                .into_iter()
                .map(|op| Operation::MVRegister(op))
                .collect(),
            DataType::Logoot(crdt) => crdt
                .convert_operation(text_operation)
                .into_iter()
                .map(|op| Operation::Logoot(op))
                .collect(),
            DataType::LSeq(crdt) => crdt
                .convert_operation(text_operation)
                .into_iter()
                .map(|op| Operation::LSeq(op))
                .collect(),
            DataType::RGA(crdt) => crdt
                .convert_operation(text_operation)
                .into_iter()
                .map(|op| Operation::RGA(op))
                .collect(),
            DataType::AWSet(crdt) => crdt
                .convert_operation(text_operation)
                .into_iter()
                .map(|op| Operation::AWSet(op))
                .collect(),
            DataType::GSet(crdt) => crdt
                .convert_operation(text_operation)
                .into_iter()
                .map(|op| Operation::GSet(op))
                .collect(),
            DataType::ORSet(crdt) => crdt
                .convert_operation(text_operation)
                .into_iter()
                .map(|op| Operation::ORSet(op))
                .collect(),
            DataType::RWSet(crdt) => crdt
                .convert_operation(text_operation)
                .into_iter()
                .map(|op| Operation::RWSet(op))
                .collect(),
            DataType::TPSet(crdt) => crdt
                .convert_operation(text_operation)
                .into_iter()
                .map(|op| Operation::TPSet(op))
                .collect(),
            DataType::MerkleDAGTree(crdt) => crdt
                .convert_operation(text_operation)
                .into_iter()
                .map(|op| Operation::MerkleDAGTree(op))
                .collect(),
        }
    }

    pub fn apply_operation(&mut self, op: Operation<K>) {
        match (self, op) {
            (DataType::Gcounter(crdt), Operation::GCounter(op)) => crdt.apply(op),
            (DataType::PNcounter(crdt), Operation::PNCounter(op)) => crdt.apply(op),
            (DataType::AWGraph(crdt), Operation::AWGraph(op)) => crdt.apply(op),
            (DataType::GGraph(crdt), Operation::GGraph(op)) => crdt.apply(op),
            (DataType::ORGraph(crdt), Operation::ORGraph(op)) => crdt.apply(op),
            (DataType::TPGraph(crdt), Operation::TPGraph(op)) => crdt.apply(op),
            // (DataType::CMMap(crdt), Operation::CMMap(op)) => crdt.apply(op),
            // (DataType::LWWMap(crdt), Operation::LWWMap(op)) => crdt.apply(op),
            // (DataType::ORMap(crdt), Operation::ORMap(op)) => crdt.apply(op),
            // (DataType::RMap(crdt), Operation::RMap(op)) => crdt.apply(op),
            (DataType::LWWRegister(crdt), Operation::LWWRegister(op)) => crdt.apply(op),
            (DataType::MVRegister(crdt), Operation::MVRegister(op)) => crdt.apply(op),
            (DataType::Logoot(crdt), Operation::Logoot(op)) => crdt.apply(op),
            (DataType::LSeq(crdt), Operation::LSeq(op)) => crdt.apply(op),
            (DataType::RGA(crdt), Operation::RGA(op)) => crdt.apply(op),
            (DataType::AWSet(crdt), Operation::AWSet(op)) => crdt.apply(op),
            (DataType::GSet(crdt), Operation::GSet(op)) => crdt.apply(op),
            (DataType::ORSet(crdt), Operation::ORSet(op)) => crdt.apply(op),
            (DataType::RWSet(crdt), Operation::RWSet(op)) => crdt.apply(op),
            (DataType::TPSet(crdt), Operation::TPSet(op)) => crdt.apply(op),
            (DataType::MerkleDAGTree(crdt), Operation::MerkleDAGTree(op)) => crdt.apply(op),
            _ => panic!("Invalid operation for CRDT type"),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            DataType::Gcounter(crdt) => crdt.to_string(),
            DataType::PNcounter(crdt) => crdt.to_string(),
            DataType::AWGraph(crdt) => crdt.to_string(),
            DataType::GGraph(crdt) => crdt.to_string(),
            DataType::ORGraph(crdt) => crdt.to_string(),
            DataType::TPGraph(crdt) => crdt.to_string(),
            // DataType::CMMap(crdt) => crdt.to_string(),
            // DataType::LWWMap(crdt) => crdt.to_string(),
            // DataType::ORMap(crdt) => crdt.to_string(),
            // DataType::RMap(crdt) => crdt.to_string(),
            DataType::LWWRegister(crdt) => crdt.to_string(),
            DataType::MVRegister(crdt) => crdt.to_string(),
            DataType::Logoot(crdt) => crdt.to_string(),
            DataType::LSeq(crdt) => crdt.to_string(),
            DataType::RGA(crdt) => crdt.to_string(),
            DataType::AWSet(crdt) => crdt.to_string(),
            DataType::GSet(crdt) => crdt.to_string(),
            DataType::ORSet(crdt) => crdt.to_string(),
            DataType::RWSet(crdt) => crdt.to_string(),
            DataType::TPSet(crdt) => crdt.to_string(),
            DataType::MerkleDAGTree(crdt) => crdt.to_string(),
        }
    }
}
