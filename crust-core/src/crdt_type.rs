use std::{collections::HashMap, hash::Hash};

use serde::{Deserialize, Serialize};

use crate::{
    counter::{self, gcounter::GCounter, pncounter::PNCounter},
    graph::{self, awgraph::AWGraph, ggraph::GGraph, orgraph::ORGraph, tpgraph::TPGraph},
    register::{self, lwwregister::LWWRegister, mvregister::MVRegister},
    // sequence::{self,logoot::Logoot, lseq::LSeq, rga::RGA},
    set::{self, awset::AWSet, gset::GSet, orset::ORSet, rwset::RWSet, tpset::TPSet},
    // text_operation::TextOperation,
    // tree::merkle_dag_tree::MerkleDAGTree,
};

use crate::text_operation::TextOperation;

pub trait CmRDT {
    type Op: Clone;
    type Value: Clone;

    fn apply(&mut self, op: &Self::Op);
    fn convert_operation(&self, op: TextOperation<Self::Value>) -> Vec<Self::Op>;
}
pub trait CvRDT {
    fn merge(&mut self, other: &Self);
}
pub trait Delta {
    type De: Clone;

    fn generate_delta(&self, since: &Self) -> Self::De;
    fn merge_delta(&mut self, delta: Self::De);
}

#[derive(Clone)]
pub enum DataType<K>
where
    K: Eq + Clone + Hash + Ord,
{
    GCounter(GCounter<K>),
    PNCounter(PNCounter<K>),
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
    // Logoot(Logoot<K>),
    // LSeq(LSeq<K>),
    // RGA(RGA<K>),
    AWSet(AWSet<K>),
    GSet(GSet<K>),
    ORSet(ORSet<K>),
    RWSet(RWSet<K>),
    TPSet(TPSet<K>),
    // MerkleDAGTree(MerkleDAGTree<K>),
}

pub enum DataTypeDelta<K>
where
    K: Eq + Clone + Hash + Ord,
{
    GCounter(HashMap<K, u64>),
    PNCounter((HashMap<K, u64>, HashMap<K, u64>)),
    AWGraph(AWGraph<K>),
    GGraph(GGraph<K>),
    ORGraph(ORGraph<K>),
    TPGraph(TPGraph<K>),
    // CMMap(HashMap<K, V>),
    // LWWMap(HashMap<K, V>),
    // ORMap(HashMap<K, V>),
    // RMap(HashMap<K, V>),
    // LWWRegister(LWWRegister<K>),
    MVRegister(MVRegister<K>),
    // Logoot(HashMap<K, (K, u128)>),
    // LSeq(HashMap<K, (K, u128)>),
    // RGA(HashMap<K, (K, u128)>),
    AWSet(AWSet<K>),
    GSet(GSet<K>),
    ORSet(ORSet<K>),
    RWSet(RWSet<K>),
    TPSet(TPSet<K>),
    // MerkleDAGTree(HashMap<K, (K, u128)>),
    None,
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
    // Logoot(sequence::logoot::Operation<K>),
    // LSeq(sequence::lseq::Operation<K>),
    // RGA(sequence::rga::Operation<K>),
    AWSet(set::awset::Operation<K>),
    GSet(set::gset::Operation<K>),
    ORSet(set::orset::Operation<K>),
    RWSet(set::rwset::Operation<K>),
    TPSet(set::tpset::Operation<K>),
    // MerkleDAGTree(tree::merkle_dag_tree::Operation<K>),
}

impl<K> DataType<K>
where
    K: Eq + Hash + Clone + Ord + Serialize + for<'a> Deserialize<'a>,
    DataType<K>: Clone,
{
    pub fn new(crdt_type: String) -> Self {
        match crdt_type.as_str() {
            "gcounter" => DataType::GCounter(GCounter::<K>::new()),
            "pncounter" => DataType::PNCounter(PNCounter::<K>::new()),
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
            // "logoot" => DataType::Logoot(Logoot::<K>::new()),
            // "lseq" => DataType::LSeq(LSeq::<K>::new()),
            // "rga" => DataType::RGA(RGA::<K>::new()),
            "awset" => DataType::AWSet(AWSet::<K>::new()),
            "gset" => DataType::GSet(GSet::<K>::new()),
            "orset" => DataType::ORSet(ORSet::<K>::new()),
            "rwset" => DataType::RWSet(RWSet::<K>::new()),
            "tpset" => DataType::TPSet(TPSet::<K>::new()),
            // "merkledagtree" => DataType::MerkleDAGTree(MerkleDAGTree::<K>::new()),
            _ => panic!("Invalid CRDT type"),
        }
    }

    pub fn convert_operation(&self, text_operation: TextOperation<K>) -> Vec<Operation<K>> {
        match self {
            DataType::GCounter(crdt) => crdt
                .convert_operation(text_operation)
                .into_iter()
                .map(|op| Operation::GCounter(op))
                .collect(),
            DataType::PNCounter(crdt) => crdt
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
            // DataType::Logoot(crdt) => crdt
            //     .convert_operation(text_operation)
            //     .into_iter()
            //     .map(|op| Operation::Logoot(op))
            //     .collect(),
            // DataType::LSeq(crdt) => crdt
            //     .convert_operation(text_operation)
            //     .into_iter()
            //     .map(|op| Operation::LSeq(op))
            //     .collect(),
            // DataType::RGA(crdt) => crdt
            //     .convert_operation(text_operation)
            //     .into_iter()
            //     .map(|op| Operation::RGA(op))
            //     .collect(),
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
            // DataType::MerkleDAGTree(crdt) => crdt
            //     .convert_operation(text_operation)
            //     .into_iter()
            //     .map(|op| Operation::MerkleDAGTree(op))
            //     .collect(),
        }
    }

    pub fn apply_operation(&mut self, op: Operation<K>) {
        match (self, &op) {
            (DataType::GCounter(crdt), Operation::GCounter(op)) => crdt.apply(op),
            (DataType::PNCounter(crdt), Operation::PNCounter(op)) => crdt.apply(op),
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
            // (DataType::Logoot(crdt), Operation::Logoot(op)) => crdt.apply(op),
            // (DataType::LSeq(crdt), Operation::LSeq(op)) => crdt.apply(op),
            // (DataType::RGA(crdt), Operation::RGA(op)) => crdt.apply(op),
            (DataType::AWSet(crdt), Operation::AWSet(op)) => crdt.apply(op),
            (DataType::GSet(crdt), Operation::GSet(op)) => crdt.apply(op),
            (DataType::ORSet(crdt), Operation::ORSet(op)) => crdt.apply(op),
            (DataType::RWSet(crdt), Operation::RWSet(op)) => crdt.apply(op),
            (DataType::TPSet(crdt), Operation::TPSet(op)) => crdt.apply(op),
            // (DataType::MerkleDAGTree(crdt), Operation::MerkleDAGTree(op)) => crdt.apply(op),
            _ => panic!("Invalid operation for CRDT type"),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            DataType::GCounter(crdt) => crdt.to_string().unwrap(),
            DataType::PNCounter(crdt) => crdt.to_string().unwrap(),
            DataType::AWGraph(crdt) => crdt.to_string().unwrap(),
            DataType::GGraph(crdt) => crdt.to_string().unwrap(),
            DataType::ORGraph(crdt) => crdt.to_string().unwrap(),
            DataType::TPGraph(crdt) => crdt.to_string().unwrap(),
            // DataType::CMMap(crdt) => crdt.to_string(),
            // DataType::LWWMap(crdt) => crdt.to_string(),
            // DataType::ORMap(crdt) => crdt.to_string(),
            // DataType::RMap(crdt) => crdt.to_string(),
            DataType::LWWRegister(crdt) => crdt.to_string().unwrap(),
            DataType::MVRegister(crdt) => crdt.to_string().unwrap(),
            // DataType::Logoot(crdt) => crdt.to_string(),
            // DataType::LSeq(crdt) => crdt.to_string(),
            // DataType::RGA(crdt) => crdt.to_string(),
            DataType::AWSet(crdt) => crdt.to_string().unwrap(),
            DataType::GSet(crdt) => crdt.to_string().unwrap(),
            DataType::ORSet(crdt) => crdt.to_string().unwrap(),
            DataType::RWSet(crdt) => crdt.to_string().unwrap(),
            DataType::TPSet(crdt) => crdt.to_string().unwrap(),
            // DataType::MerkleDAGTree(crdt) => crdt.to_string(),
        }
    }

    pub fn to_crdt(&mut self, str: String) {
        match self {
            DataType::GCounter(crdt) => *crdt = GCounter::to_crdt(str).unwrap(),
            DataType::PNCounter(crdt) => *crdt = PNCounter::to_crdt(str).unwrap(),
            DataType::AWGraph(crdt) => *crdt = AWGraph::to_crdt(str).unwrap(),
            DataType::GGraph(crdt) => *crdt = GGraph::to_crdt(str).unwrap(),
            DataType::ORGraph(crdt) => *crdt = ORGraph::to_crdt(str).unwrap(),
            DataType::TPGraph(crdt) => *crdt = TPGraph::to_crdt(str).unwrap(),
            // DataType::CMMap(crdt) => *crdt = CMMap::to_crdt(str),
            // DataType::LWWMap(crdt) => *crdt = LWWMap::to_crdt(str),
            // DataType::ORMap(crdt) => *crdt = ORMap::to_crdt(str),
            // DataType::RMap(crdt) => *crdt = RMap::to_crdt(str),
            DataType::LWWRegister(crdt) => *crdt = LWWRegister::to_crdt(str).unwrap(),
            DataType::MVRegister(crdt) => *crdt = MVRegister::to_crdt(str).unwrap(),
            // DataType::Logoot(crdt) => *crdt = Logoot::to_crdt(str).unwrap(),
            // DataType::LSeq(crdt) => *crdt = LSeq::to_crdt(str).unwrap(),
            // DataType::RGA(crdt) => *crdt = RGA::to_crdt(str).unwrap(),
            DataType::AWSet(crdt) => *crdt = AWSet::to_crdt(str).unwrap(),
            DataType::GSet(crdt) => *crdt = GSet::to_crdt(str).unwrap(),
            DataType::ORSet(crdt) => *crdt = ORSet::to_crdt(str).unwrap(),
            DataType::RWSet(crdt) => *crdt = RWSet::to_crdt(str).unwrap(),
            DataType::TPSet(crdt) => *crdt = TPSet::to_crdt(str).unwrap(),
            // DataType::MerkleDAGTree(crdt) => *crdt = MerkleDAGTree::to_crdt(str),
        }
    }

    pub fn merge(&mut self, other: Self) {
        match (self, other) {
            (DataType::GCounter(crdt), DataType::GCounter(other_crdt)) => crdt.merge(&other_crdt),
            (DataType::PNCounter(crdt), DataType::PNCounter(other_crdt)) => crdt.merge(&other_crdt),
            (DataType::AWGraph(crdt), DataType::AWGraph(other_crdt)) => crdt.merge(&other_crdt),
            (DataType::GGraph(crdt), DataType::GGraph(other_crdt)) => crdt.merge(&other_crdt),
            (DataType::ORGraph(crdt), DataType::ORGraph(other_crdt)) => crdt.merge(&other_crdt),
            (DataType::TPGraph(crdt), DataType::TPGraph(other_crdt)) => crdt.merge(&other_crdt),
            // (DataType::CMMap(crdt), DataType::CMMap(other_crdt)) => crdt.merge(&other_crdt),
            // (DataType::LWWMap(crdt), DataType::LWWMap(other_crdt)) => crdt.merge(&other_crdt),
            // (DataType::ORMap(crdt), DataType::ORMap(other_crdt)) => crdt.merge(&other_crdt),
            // (DataType::RMap(crdt), DataType::RMap(other_crdt)) => crdt.merge(&other_crdt),
            (DataType::LWWRegister(crdt), DataType::LWWRegister(other_crdt)) => {
                crdt.merge(&other_crdt)
            }
            (DataType::MVRegister(crdt), DataType::MVRegister(other_crdt)) => {
                crdt.merge(&other_crdt)
            }
            // (DataType::Logoot(crdt), DataType::Logoot(other_crdt)) => crdt.merge(&other_crdt),
            // (DataType::LSeq(crdt), DataType::LSeq(other_crdt)) => crdt.merge(&other_crdt),
            // (DataType::RGA(crdt), DataType::RGA(other_crdt)) => crdt.merge(&other_crdt),
            (DataType::AWSet(crdt), DataType::AWSet(other_crdt)) => crdt.merge(&other_crdt),
            (DataType::GSet(crdt), DataType::GSet(other_crdt)) => crdt.merge(&other_crdt),
            (DataType::ORSet(crdt), DataType::ORSet(other_crdt)) => crdt.merge(&other_crdt),
            (DataType::RWSet(crdt), DataType::RWSet(other_crdt)) => crdt.merge(&other_crdt),
            (DataType::TPSet(crdt), DataType::TPSet(other_crdt)) => crdt.merge(&other_crdt),
            // (DataType::MerkleDAGTree(crdt), DataType::MerkleDAGTree(other_crdt)) => {
            //     crdt.merge(&other_crdt)
            // }
            _ => panic!("Cannot merge different CRDT types"),
        }
    }

    pub fn to_delta(&mut self, str: String) -> DataTypeDelta<K> {
        match self {
            DataType::GCounter(_crdt) => DataTypeDelta::GCounter(GCounter::to_delta(str).unwrap()),
            DataType::PNCounter(_crdt) => {
                DataTypeDelta::PNCounter(PNCounter::to_delta(str).unwrap())
            }
            DataType::AWGraph(_crdt) => DataTypeDelta::AWGraph(AWGraph::to_delta(str).unwrap()),
            DataType::GGraph(_crdt) => DataTypeDelta::GGraph(GGraph::to_delta(str).unwrap()),
            DataType::ORGraph(_crdt) => DataTypeDelta::ORGraph(ORGraph::to_delta(str).unwrap()),
            DataType::TPGraph(_crdt) => DataTypeDelta::TPGraph(TPGraph::to_delta(str).unwrap()),
            // DataType::CMMap(crdt) =>  CMMap::to_delta(str),
            // DataType::LWWMap(crdt) =>  LWWMap::to_delta(str),
            // DataType::ORMap(crdt) =>  ORMap::to_delta(str),
            // DataType::RMap(crdt) =>  RMap::to_delta(str),
            DataType::LWWRegister(_crdt) => DataTypeDelta::None,
            DataType::MVRegister(_crdt) => {
                DataTypeDelta::MVRegister(MVRegister::to_delta(str).unwrap())
            }
            // DataType::Logoot(crdt) =>  Logoot::to_delta(str).unwrap(),
            // DataType::LSeq(crdt) =>  LSeq::to_delta(str).unwrap(),
            // DataType::RGA(crdt) =>  RGA::to_delta(str).unwrap(),
            DataType::AWSet(_crdt) => DataTypeDelta::AWSet(AWSet::to_delta(str).unwrap()),
            DataType::GSet(_crdt) => DataTypeDelta::GSet(GSet::to_delta(str).unwrap()),
            DataType::ORSet(_crdt) => DataTypeDelta::ORSet(ORSet::to_delta(str).unwrap()),
            DataType::RWSet(_crdt) => DataTypeDelta::RWSet(RWSet::to_delta(str).unwrap()),
            DataType::TPSet(_crdt) => DataTypeDelta::TPSet(TPSet::to_delta(str).unwrap()),
            // DataType::MerkleDAGTree(crdt) =>  MerkleDAGTree::to_delta(str),
        }
    }

    pub fn merge_delta(&mut self, delta: DataTypeDelta<K>) {
        match (self, delta) {
            (DataType::GCounter(crdt), DataTypeDelta::GCounter(delta)) => crdt.merge_delta(delta),
            (DataType::PNCounter(crdt), DataTypeDelta::PNCounter(delta)) => crdt.merge_delta(delta),
            (DataType::AWGraph(crdt), DataTypeDelta::AWGraph(delta)) => crdt.merge_delta(delta),
            (DataType::GGraph(crdt), DataTypeDelta::GGraph(delta)) => crdt.merge_delta(delta),
            (DataType::ORGraph(crdt), DataTypeDelta::ORGraph(delta)) => crdt.merge_delta(delta),
            (DataType::TPGraph(crdt), DataTypeDelta::TPGraph(delta)) => crdt.merge_delta(delta),
            // (DataType::CMMap(crdt), DataTypeDelta::CMMap(delta)) => crdt.merge_delta(delta),
            // (DataType::LWWMap(crdt), DataTypeDelta::LWWMap(delta)) => crdt.merge_delta(delta),
            // (DataType::ORMap(crdt), DataTypeDelta::ORMap(delta)) => crdt.merge_delta(delta),
            // (DataType::RMap(crdt), DataTypeDelta::RMap(delta)) => crdt.merge_delta(delta),
            // (DataType::LWWRegister(crdt), DataTypeDelta::LWWRegister(delta)) => crdt.merge_delta(delta),
            (DataType::MVRegister(crdt), DataTypeDelta::MVRegister(delta)) => {
                crdt.merge_delta(delta)
            }
            // (DataType::Logoot(crdt), DataTypeDelta::(delta)) => crdt.merge_delta(delta),
            // (DataType::LSeq(crdt), DataTypeDelta::(delta)) => crdt.merge_delta(delta),
            // (DataType::RGA(crdt), DataTypeDelta::(delta)) => crdt.merge_delta(delta),
            (DataType::AWSet(crdt), DataTypeDelta::AWSet(delta)) => crdt.merge_delta(delta),
            (DataType::GSet(crdt), DataTypeDelta::GSet(delta)) => crdt.merge_delta(delta),
            (DataType::ORSet(crdt), DataTypeDelta::ORSet(delta)) => crdt.merge_delta(delta),
            (DataType::RWSet(crdt), DataTypeDelta::RWSet(delta)) => crdt.merge_delta(delta),
            (DataType::TPSet(crdt), DataTypeDelta::TPSet(delta)) => crdt.merge_delta(delta),
            // (DataType::MerkleDAGTree(crdt), DataTypeDelta::) => crdt.merge_delta(delta),
            _ => panic!("Cannot merge delta with different CRDT types"),
        }
    }
}
