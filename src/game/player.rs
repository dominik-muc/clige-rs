use std::borrow::BorrowMut;

use super::Level;

pub struct Player {
    health: u32,
    score: u32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            health: 100,
            score: 0,
        }
    }
    pub fn get_health(&self) -> u32 {
        self.health
    }
}
