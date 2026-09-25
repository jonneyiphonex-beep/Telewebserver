use crate::state::TelecomEngine;
use std::sync::atomic::Ordering;

pub struct SimManager;

impl SimManager {
    /// التبديل بين الشرائح بداخل الذاكرة بلمح البصر (Nano-second level atomic store)
    pub fn switch_active_slot(engine: &TelecomEngine, slot_id: u8) {
        engine.active_sim_slot.store(slot_id, Ordering::SeqCst);
    }

    pub fn get_active_slot(engine: &TelecomEngine) -> u8 {
        engine.active_sim_slot.load(Ordering::Relaxed)
    }
}