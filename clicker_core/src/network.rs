use serde::{Deserialize, Serialize};

pub const PROTOCOL_ID: u64 = 7;

#[derive(Debug, Serialize, Deserialize)]
pub enum Messages {
    ClickEvent
}