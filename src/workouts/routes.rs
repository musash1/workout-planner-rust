use warp::Filter;
use crate::workouts::handlers;
use super::models;

fn json_body() -> impl Filter<Extract = (models::Workout,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn get_workout() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("workout")
        .and(warp::get())
        .and_then(handlers::get_workout)
}

pub fn post_workout() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("workout")
    .and(warp::post())
    .and(json_body())
    .and_then(handlers::create_workout)
}
