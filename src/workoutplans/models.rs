use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use crate::workouts::models::Workout;

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema, IntoParams)]
pub struct WorkoutPlan {
    pub id: u16,
    pub name: String,
    pub workouts: Vec<Workout>
}

impl WorkoutPlan {
    pub fn new(id: u16, name: String, workouts: Vec<Workout>) -> Self {
        Self {
            id,
            name,
            workouts,
        }
    }
}
