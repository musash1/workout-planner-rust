use uuid::Uuid;
use warp::Filter;
use crate::models::Exercise;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Write, BufReader, BufRead, Error};

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

pub async fn create_exercise(body: String) -> Result<impl warp::Reply, warp::Rejection> {
    let file = fs::read_to_string("exercises.json").unwrap();
    let new_file = OpenOptions::new().write(true).truncate(true).open("exercises.json");
    let mut exercises: Vec<Exercise> = serde_json::from_str(file.as_str()).expect("Couldnt create list");
    let new_exercise: Exercise = serde_json::from_str(body.as_str()).expect("Couldnt get object from string");
    exercises.push(new_exercise);


    Ok(warp::reply::json(&exercises))
}
