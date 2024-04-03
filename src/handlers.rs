use uuid::Uuid;
use crate::models::Exercise;
use std::convert::Infallible;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use warp::http::StatusCode;

pub async fn get_exercise(name: String) -> Result<impl warp::Reply, warp::Rejection> {
    let exercise = Exercise  {
        id: Uuid::new_v4().to_string(),
        name,
        reps: 12,
        sets: 4
    };
    Ok(warp::reply::json(&exercise))
}

pub async fn get_exercises() -> Result<impl warp::Reply, warp::Rejection> {
    let exercise = fs::read_to_string("exercises.json").expect("couldn't read file.");
    let exercise_json: serde_json::Value = serde_json::from_str(exercise.as_str()).expect("couldnt convert to json");
    Ok(warp::reply::json(&exercise_json))
}

pub fn create_exercise(new_exercise: Exercise) -> Result<impl warp::Reply, Infallible> {
    let file = fs::read_to_string("exercises.json").unwrap();
    let mut new_file = OpenOptions::new().write(true).truncate(true).open("exercises.json").expect("couldnt open file");
    let mut exercises: Vec<Exercise> = serde_json::from_str(file.as_str()).expect("Couldnt create list");
    exercises.push(new_exercise);
    let json = serde_json::to_string(&exercises).expect("couldnt create json");
    new_file.write(json.as_bytes()).expect("couldnt write file");
    Ok(warp::reply::with_status(format!("Exercise created"), StatusCode::CREATED))
}


