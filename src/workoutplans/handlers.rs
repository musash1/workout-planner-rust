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

#[utoipa::path(
        delete,
        path = "/workoutplan/{id}",
        responses(
            (status = 200, description = "Delete successful"),
            (status = 404, description = "Workoutplan not found to delete"),
        ),
        params(("id" = u16, Path, description = "Workouts's unique id"))
    )]
pub async fn delete_workoutplan(id: u16) -> Result<impl warp::Reply, Infallible> {
    let file = fs::read_to_string("workoutplans.json").unwrap();
    let mut new_file = OpenOptions::new().write(true).truncate(true).open("workoutplans.json").expect("couldnt open file");
    let mut workoutplans: Vec<WorkoutPlan> = serde_json::from_str(&file).unwrap();
    let index = workoutplans.iter().position(|w| w.id == id).unwrap_or(0);
    if index == 0 {
        return Ok(warp::reply::with_status(format!("Workoutplan not found"), StatusCode::NOT_FOUND));
    }
    workoutplans.remove(index);
    let json = serde_json::to_string(&workoutplans).expect("couldnt create json");
    new_file.write(json.as_bytes()).expect("couldnt write file");
    Ok(warp::reply::with_status(format!("Workoutplan deleted"), StatusCode::OK))
}

#[utoipa::path(
        put,
        path = "/workoutplan",
        request_body = WorkoutPlan,
        responses((status = 200, description = "workoutplan updated", body = [Workout]))
    )]
pub async fn update_workoutplan(new_workoutplan: WorkoutPlan) -> Result<impl warp::Reply, Infallible> {
    let file = fs::read_to_string("workoutplans.json").unwrap();
    let mut new_file = OpenOptions::new().write(true).truncate(true).open("workoutplans.json").expect("couldnt open file");
    let mut workoutplans: Vec<WorkoutPlan> = serde_json::from_str(&file).unwrap();
    let index = workoutplans.iter().position(|e| e.id == new_workoutplan.id).unwrap();

    if !new_workoutplan.name.is_empty() {
        workoutplans[index].name = new_workoutplan.name;
    }

    if new_workoutplan.workouts.len() != 0 {
        workoutplans[index].workouts = new_workoutplan.workouts;
    }

    let json = serde_json::to_string(&workoutplans).expect("couldnt create json");
    new_file.write(json.as_bytes()).expect("couldnt write file");
    Ok(warp::reply::with_status(format!("Workout updated"), StatusCode::OK))
}
