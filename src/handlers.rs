use uuid::Uuid;
use warp::Filter;
use crate::models::Exercise;
use std::fs;

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
    let exercise = fs::read("exercises.json").expect("couldn't read file.");
    Ok(warp::reply::json(&exercise))
}
