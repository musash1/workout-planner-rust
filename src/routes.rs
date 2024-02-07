use warp::Filter;
use crate::handlers;
use std::collections::HashMap;


pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_exercise()
}

fn get_exercise() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!()
        .and(warp::get())
        .and_then(handlers::get_exercises)
}

fn create_exercise() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let route = warp::body::content_length_limit(1024 * 32)
        .and(warp::body::json())
        .map(|simple_map: HashMap<String, String>| {
        "Got a JSON body"
    });
    warp::path!()
        .and(warp::post())
        .and_then(handlers::create_exercise)
}

