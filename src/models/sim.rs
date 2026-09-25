use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SimProfile {
    pub slot_id: u8,
    pub iccid: String,
    pub operator_code: String,
    pub is_active: bool,
    pub signal_strength_dbm: i16,
}