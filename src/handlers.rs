use uuid::Uuid;
use warp::Filter;
use crate::models::Exercise;

pub async fn get_exercise(name: String) -> Result<impl warp::Reply, warp::Rejection> {
    let exercise = Exercise  {
        id: Uuid::new_v4().to_string(),
        name,
        reps: 12,
        sets: 4
    };
    Ok(warp::reply::json(&exercise))
}
