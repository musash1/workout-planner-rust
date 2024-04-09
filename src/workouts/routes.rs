use warp::Filter;
use crate::workouts::{handlers, models::Workout};

fn json_body() -> impl Filter<Extract = (Workout,), Error = warp::Rejection> + Clone {
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

pub fn delete_workout() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone { 
    warp::path!("workout" / u16)
        .and(warp::delete())
        .and_then(handlers::delete_workout)
}

pub fn update_workout() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone { 
    warp::path!("workout")
        .and(warp::put())
        .and(json_body())
        .and_then(handlers::update_workout)
}
