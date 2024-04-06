use crate::exercises::models::Exercise;
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

    if !exercises.iter().any(|e| e.id == new_exercise.id) {
        exercises.push(new_exercise);
    } else {
        return Ok(warp::reply::with_status(format!("Exercise exists already"), StatusCode::BAD_REQUEST));
    }

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
    let index = exercises.iter().position(|e| e.id == new_exercise.id).unwrap();

    if !new_exercise.name.is_empty() {
        exercises[index].name = new_exercise.name;
    }

    if new_exercise.sets != 0 {
        exercises[index].sets = new_exercise.sets;
    }

    if new_exercise.reps != 0 {
        exercises[index].reps = new_exercise.reps;
    }

    let json = serde_json::to_string(&exercises).expect("couldnt create json");
    new_file.write(json.as_bytes()).expect("couldnt write file");
    Ok(warp::reply::with_status(format!("Exercise updated"), StatusCode::OK))
}
