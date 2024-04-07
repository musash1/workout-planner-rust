use crate::workouts::models::Workout;
use std::convert::Infallible;
use std::fs;
use std::{fs::OpenOptions, io::Write};
use warp::http::StatusCode;

pub async fn get_workout() -> Result<impl warp::Reply, warp::Rejection> {
    let workout = fs::read_to_string("workouts.json").expect("couldn't read file.");
    let workout_json: serde_json::Value = serde_json::from_str(workout.as_str()).expect("couldnt convert to json");
    Ok(warp::reply::json(&workout_json))
}

pub async fn create_workout(new_workout: Workout) -> Result<impl warp::Reply, warp::Rejection> {
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
