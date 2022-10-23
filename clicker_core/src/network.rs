use serde::{Deserialize, Serialize};

pub const PROTOCOL_ID: u64 = 7;

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMessages {
    ClickEvent
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessages {
    UpdateState { gold: u64 },
}