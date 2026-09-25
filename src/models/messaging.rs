use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessagePayload {
    pub from: String,
    pub to: String,
    pub content: String,
    pub is_rcs: bool,
}