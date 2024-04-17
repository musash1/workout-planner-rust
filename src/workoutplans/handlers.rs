use std::{convert::Infallible, fs};
use crate::workoutplans::models::WorkoutPlan;

#[utoipa::path(
        get,
        path = "/workoutplan",
        responses((status = 200, description = "List workoutplans successfully", body = [WorkoutPlan]))
    )]
pub async fn get_workoutplan() -> Result<impl warp::Reply, Infallible> {
    let workoutplan = fs::read_to_string("workoutplans.json").expect("couldn't read file.");
    let workoutplan_json: serde_json::Value = serde_json::from_str(workoutplan.as_str()).expect("couldnt convert to json");
    Ok(warp::reply::json(&workoutplan_json))
}
