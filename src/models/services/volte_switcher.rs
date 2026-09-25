use crate::state::TelecomEngine;
use std::sync::atomic::Ordering;

pub struct VolteSwitcher;

impl VolteSwitcher {
    /// تبديل مسار البيانات بسرعة فائقة أثناء مكالمات VoLTE
    pub fn set_volte_state(engine: &TelecomEngine, is_active: bool) -> &'static str {
        engine.volte_call_active.store(is_active, Ordering::SeqCst);

        if is_active {
            "VoLTE Call Active: Switched packet data to IMS Dedicated Bearer"
        } else {
            "VoLTE Call Ended: Restored default LTE Data Bearer"
        }
    }
}