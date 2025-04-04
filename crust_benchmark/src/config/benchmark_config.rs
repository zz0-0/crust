use crust_core::{
    r#type::CrdtType,
    sync::{SyncMode, SyncType},
};

#[derive(Clone)]
pub struct BenchmarkConfig {
    pub crdt_type: CrdtType<String>,
    pub sync_type: SyncType,
    pub sync_mode: SyncMode,
    pub replica_count: usize,
    pub command_count: usize,
}

impl BenchmarkConfig {
    pub fn new(
        crdt_type: CrdtType<String>,
        sync_type: SyncType,
        sync_mode: SyncMode,
        replica_count: usize,
        command_count: usize,
    ) -> Self {
        Self {
            crdt_type,
            sync_type,
            sync_mode,
            replica_count,
            command_count,
        }
    }
}
