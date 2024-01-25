use warp::Filter;
use crate::handlers;

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_exercise()
}

fn get_exercise() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("exercise" / String)
        .and(warp::get())
        .and_then(handlers::get_exercise)
}
