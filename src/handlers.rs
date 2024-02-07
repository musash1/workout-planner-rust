use uuid::Uuid;
use warp::Filter;
use warp::filters::body;
use crate::models::Exercise;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Write, BufReader, BufRead, Error};
use warp::hyper::body::Body;

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

pub fn create_exercise() -> Result<impl warp::Reply, warp::Rejection> {
    let body_string = json_body();
    let file = fs::read_to_string("exercises.json").unwrap();
    let mut new_file = OpenOptions::new().write(true).truncate(true).open("exercises.json").expect("couldnt open file");
    let mut exercises: Vec<Exercise> = serde_json::from_str(file.as_str()).expect("Couldnt create list");
    let new_exercise: Exercise = serde_json::from_str(body_string).expect("Couldnt get object from string");
    exercises.push(new_exercise);
    let json = serde_json::to_string(&exercises).expect("couldnt create json");
    new_file.write(json.as_bytes()).expect("couldnt write file");
    Ok(warp::reply::json(&json))
}

fn json_body() -> impl Filter<Extract = (Exercise,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
