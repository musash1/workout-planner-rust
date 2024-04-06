use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Exercise {
    pub id: u16,
    pub name: String,
    pub reps: u16,
    pub sets: u16,
}

impl Exercise {
    pub fn new(id: u16, name: String, reps: u16, sets: u16) -> Self {
        Self {
            id,
            name,
            reps,
            sets,
        }
    }
}
