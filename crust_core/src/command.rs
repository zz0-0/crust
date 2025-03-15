use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CounterInnerCommand<K> {
    Increment { value: K },
    Decrement { value: K },
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum GraphInnerCommand<K> {
    AddNode { value: K },
    RemoveNode { value: K },
    AddEdge { from: K, to: K },
    RemoveEdge { from: K, to: K },
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SetInnerCommand<K> {
    Add { value: K },
    Remove { value: K },
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TextInnerCommand<K> {
    Insert { pos: usize, value: K },
    Delete { pos: usize, value: K },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CrdtInnerCommand<K> {
    Counter(CounterInnerCommand<K>),
    Graph(GraphInnerCommand<K>),
    Set(SetInnerCommand<K>),
    Text(TextInnerCommand<K>),
}
