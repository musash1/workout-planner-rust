use warp::Filter;
use crate::workoutplans::{handlers, models::WorkoutPlan};

fn json_body() -> impl Filter<Extract = (WorkoutPlan,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn get_workoutplan() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("workoutplan")
        .and(warp::get())
        .and_then(handlers::get_workoutplan)
}

pub fn post_workoutplan() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("workoutplan")
        .and(warp::post())
        .and(json_body())
        .and_then(handlers::create_workoutplan)
}

pub fn delete_workoutplan() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone { 
    warp::path!("workoutplan" / u16)
        .and(warp::delete())
        .and_then(handlers::delete_workoutplan)
}
