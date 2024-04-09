use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use crate::exercises::models::Exercise;


#[derive(Debug, Deserialize, Serialize, Clone, ToSchema, IntoParams)]
pub struct Workout {
    pub id: u16,
    pub name: String,
    pub split_type: String,
    pub exercises: Vec<Exercise>
}

impl Workout {
    pub fn new(id: u16, name: String, split_type: String, exercises: Vec<Exercise>) -> Self {
        Self {
            id,
            name,
            split_type,
            exercises,
        }
    }
}
