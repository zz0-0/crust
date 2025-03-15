use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CounterOperation<K> {
    Increment { value: K },
    Decrement { value: K },
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum GraphOperation<K> {
    AddNode { value: K },
    RemoveNode { value: K },
    AddEdge { from: K, to: K },
    RemoveEdge { from: K, to: K },
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SetOperation<K> {
    Add { value: K },
    Remove { value: K },
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TextOperation<K> {
    Insert { pos: usize, value: K },
    Delete { pos: usize, value: K },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CrdtOperation<K> {
    Counter(CounterOperation<K>),
    Graph(GraphOperation<K>),
    Set(SetOperation<K>),
    Text(TextOperation<K>),
}
