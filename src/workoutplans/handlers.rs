use std::{convert::Infallible, fs::{self, OpenOptions}, io::Write};
use warp::http::StatusCode;

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

#[utoipa::path(
        post,
        path = "/workoutplan",
        request_body = WorkoutPlan,
        responses((status = 200, description = "created workout successfully", body = [WorkoutPlan]))
    )]
pub async fn create_workoutplan(new_workoutplan: WorkoutPlan) -> Result<impl warp::Reply, Infallible> {
    let file = fs::read_to_string("workoutplans.json").unwrap();
    let mut new_file = OpenOptions::new().write(true).truncate(true).open("workoutplans.json").expect("couldnt open file");
    let mut workoutplans: Vec<WorkoutPlan> = serde_json::from_str(&file).unwrap();

    if !workoutplans.iter().any(|w| w.id == new_workoutplan.id) {
        workoutplans.push(new_workoutplan);
    } else {
        let json = serde_json::to_string(&workoutplans).expect("couldnt create json");
        new_file.write(json.as_bytes()).expect("couldnt write file");
        return Ok(warp::reply::with_status(format!("Workoutplan exists already"), StatusCode::BAD_REQUEST));
    }

    let json = serde_json::to_string(&workoutplans).expect("couldnt create json");
    new_file.write(json.as_bytes()).expect("couldnt write file");
    Ok(warp::reply::with_status(format!("Workout created"), StatusCode::CREATED))
}
