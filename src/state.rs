use std::sync::atomic::{AtomicBool, AtomicU8};

#[derive(Debug)]
pub struct TelecomEngine {
    pub active_sim_slot: AtomicU8,
    pub volte_call_active: AtomicBool,
}

impl TelecomEngine {
    pub fn new() -> Self {
        Self {
            active_sim_slot: AtomicU8::new(1),
            volte_call_active: AtomicBool::new(false),
        }
    }
}