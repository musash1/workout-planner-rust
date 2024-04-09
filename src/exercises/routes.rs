use warp::Filter;
use crate::exercises::handlers;
use crate::exercises::models::Exercise;

fn json_body() -> impl Filter<Extract = (Exercise,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn get_exercise() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("exercise")
        .and(warp::get())
        .and(warp::path::end())
        .and_then(handlers::get_exercises)
}

pub fn create_exercise() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("exercise")
        .and(warp::post())
        .and(warp::path::end())
        .and(json_body())
        .and_then(handlers::create_exercise)
}

pub fn delete_exercise() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("exercise" / u16)
        .and(warp::delete())
        .and(warp::path::end())
        .and_then(handlers::delete_exercise)
}

pub fn update_exercise() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("exercise")
        .and(warp::put())
        .and(warp::path::end())
        .and(json_body())
        .and_then(handlers::update_exercise)
}
