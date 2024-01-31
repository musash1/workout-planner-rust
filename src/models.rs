use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Exercise {
    pub id: String,
    pub name: String,
    pub reps: u16,
    pub sets: u16,
}

impl Exercise {
    pub fn new(name: String, reps: u16, sets: u16) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            reps,
            sets,
        }
    }
}
