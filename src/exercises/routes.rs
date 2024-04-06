use warp::Filter;
use crate::exercises::handlers;
use std::collections::HashMap;
use crate::exercises::models;


pub fn get_exercises() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_exercise()
}

pub fn post_exercises() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    create_exercise()
}

pub fn delete_exercises() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    delete_exercise()
}

pub fn update_exercises() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    update_exercise()
}

fn json_body() -> impl Filter<Extract = (models::Exercise,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn get_exercise() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("exercise")
        .and(warp::get())
        .and_then(handlers::get_exercises)
}

fn create_exercise() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("exercise")
        .and(warp::post())
        .and(json_body())
        .and_then(handlers::create_exercise)
}

fn delete_exercise() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("exercise" / u16)
        .and(warp::delete())
        .and_then(handlers::delete_exercise)
}

fn update_exercise() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("exercise")
        .and(warp::put())
        .and(json_body())
        .and_then(handlers::update_exercise)
}
