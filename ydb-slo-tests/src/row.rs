use std::time::SystemTime;

pub type RowID = u64;

#[derive(Debug, Clone)]
pub struct Row {
    pub hash: Option<u64>,
    pub id: Option<RowID>,
    pub payload_str: Option<String>,
    pub payload_double: Option<f64>,
    pub payload_timestamp: Option<SystemTime>,
    pub payload_hash: Option<u64>,
}

impl Row {
    pub fn new(
        hash: Option<u64>,
        id: Option<RowID>,
        payload_str: Option<String>,
        payload_double: Option<f64>,
        payload_timestamp: Option<SystemTime>,
        payload_hash: Option<u64>,
    ) -> Self {
        Self {
            hash,
            id,
            payload_str,
            payload_double,
            payload_timestamp,
            payload_hash,
        }
    }
}
