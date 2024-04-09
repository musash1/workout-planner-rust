use crate::workouts::models::Workout;
use std::convert::Infallible;
use std::fs;
use std::{fs::OpenOptions, io::Write};
use warp::http::StatusCode;

#[utoipa::path(
        get,
        path = "/workout",
        responses(
            (status = 200, description = "List workouts successfully", body = [Workout])
        )
    )]
pub async fn get_workout() -> Result<impl warp::Reply, Infallible> {
    let workout = fs::read_to_string("workouts.json").expect("couldn't read file.");
    let workout_json: serde_json::Value = serde_json::from_str(workout.as_str()).expect("couldnt convert to json");
    Ok(warp::reply::json(&workout_json))
}

#[utoipa::path(
        post,
        path = "/workout",
        request_body = Workout,
        responses(
            (status = 200, description = "created workout successfully", body = [Workout])
        )
    )]
pub async fn create_workout(new_workout: Workout) -> Result<impl warp::Reply, Infallible> {
    let file = fs::read_to_string("workouts.json").unwrap();
    let mut new_file = OpenOptions::new().write(true).truncate(true).open("workouts.json").expect("couldnt open file");
    let mut workouts: Vec<Workout> = serde_json::from_str(&file).unwrap();

    if !workouts.iter().any(|w| w.id == new_workout.id) {
        workouts.push(new_workout);
    } else {
        let json = serde_json::to_string(&workouts).expect("couldnt create json");
        new_file.write(json.as_bytes()).expect("couldnt write file");
        return Ok(warp::reply::with_status(format!("Workout exists already"), StatusCode::BAD_REQUEST));
    }

    let json = serde_json::to_string(&workouts).expect("couldnt create json");
    new_file.write(json.as_bytes()).expect("couldnt write file");
    Ok(warp::reply::with_status(format!("Workout created"), StatusCode::CREATED))
}

#[utoipa::path(
        delete,
        path = "/workout/{id}",
        responses(
            (status = 200, description = "Delete successful"),
            (status = 404, description = "Workout not found to delete"),
        ),
        params(
            ("id" = u16, Path, description = "Workouts's unique id")
        )
    )]
pub async fn delete_workout(id: u16) -> Result<impl warp::Reply, Infallible> {
    let file = fs::read_to_string("workouts.json").unwrap();
    let mut new_file = OpenOptions::new().write(true).truncate(true).open("workouts.json").expect("couldnt open file");
    let mut workouts: Vec<Workout> = serde_json::from_str(&file).unwrap();
    let index = workouts.iter().position(|w| w.id == id).unwrap_or(0);
    if index == 0 {
        return Ok(warp::reply::with_status(format!("Workout not found"), StatusCode::NOT_FOUND));
    }
    workouts.remove(index);
    let json = serde_json::to_string(&workouts).expect("couldnt create json");
    new_file.write(json.as_bytes()).expect("couldnt write file");
    Ok(warp::reply::with_status(format!("Workout deleted"), StatusCode::OK))
}

#[utoipa::path(
        put,
        path = "/workout",
        request_body = Workout,
        responses(
            (status = 200, description = "workout updated", body = [Workout])
        )
    )]
pub async fn update_workout(new_workout: Workout) -> Result<impl warp::Reply, Infallible> {
    let file = fs::read_to_string("workouts.json").unwrap();
    let mut new_file = OpenOptions::new().write(true).truncate(true).open("workouts.json").expect("couldnt open file");
    let mut workouts: Vec<Workout> = serde_json::from_str(&file).unwrap();
    let index = workouts.iter().position(|e| e.id == new_workout.id).unwrap();

    if !new_workout.name.is_empty() {
        workouts[index].name = new_workout.name;
    }

    if !new_workout.split_type.is_empty() {
        workouts[index].split_type = new_workout.split_type;
    }

    if new_workout.exercises.len() != 0 {
        workouts[index].exercises = new_workout.exercises;
    }

    let json = serde_json::to_string(&workouts).expect("couldnt create json");
    new_file.write(json.as_bytes()).expect("couldnt write file");
    Ok(warp::reply::with_status(format!("Workout updated"), StatusCode::OK))
}
