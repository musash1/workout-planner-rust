use warp::Filter;
use crate::workouts::handlers;


pub fn get_workouts() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_workout()
}

fn get_workout() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("workout")
        .and(warp::get())
        .and_then(handlers::get_workout)
}

