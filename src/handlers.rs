use uuid::Uuid;
use crate::models::Exercise;
use std::convert::Infallible;
use std::fs;
use std::{fs::OpenOptions, io::Write};
use warp::http::StatusCode;

pub async fn get_exercises() -> Result<impl warp::Reply, warp::Rejection> {
    let exercise = fs::read_to_string("exercises.json").expect("couldn't read file.");
    let exercise_json: serde_json::Value = serde_json::from_str(exercise.as_str()).expect("couldnt convert to json");
    Ok(warp::reply::json(&exercise_json))
}

pub async fn create_exercise(new_exercise: Exercise) -> Result<impl warp::Reply, Infallible> {
    let file = fs::read_to_string("exercises.json").unwrap();
    let mut new_file = OpenOptions::new().write(true).truncate(true).open("exercises.json").expect("couldnt open file");
    let mut exercises: Vec<Exercise> = serde_json::from_str(&file).unwrap();
    exercises.push(new_exercise);
    let json = serde_json::to_string(&exercises).expect("couldnt create json");
    new_file.write(json.as_bytes()).expect("couldnt write file");
    Ok(warp::reply::with_status(format!("Exercise created"), StatusCode::CREATED))
}

pub async fn delete_exercise(id: u16) -> Result<impl warp::Reply, Infallible> {
    let file = fs::read_to_string("exercises.json").unwrap();
    let mut new_file = OpenOptions::new().write(true).truncate(true).open("exercises.json").expect("couldnt open file");
    let mut exercises: Vec<Exercise> = serde_json::from_str(&file).unwrap();
    let index = exercises.iter().position(|e| e.id == id).unwrap();
    exercises.remove(index);
    let json = serde_json::to_string(&exercises).expect("couldnt create json");
    new_file.write(json.as_bytes()).expect("couldnt write file");
    Ok(warp::reply::with_status(format!("Exercise deleted"), StatusCode::OK))
}

pub async fn update_exercise(new_exercise: Exercise) -> Result<impl warp::Reply, Infallible> {
    let file = fs::read_to_string("exercises.json").unwrap();
    let mut new_file = OpenOptions::new().write(true).truncate(true).open("exercises.json").expect("couldnt open file");
    let mut exercises: Vec<Exercise> = serde_json::from_str(&file).unwrap();
    exercises.push(new_exercise);
    let json = serde_json::to_string(&exercises).expect("couldnt create json");
    new_file.write(json.as_bytes()).expect("couldnt write file");
    Ok(warp::reply::with_status(format!("Exercise created"), StatusCode::CREATED))
}
