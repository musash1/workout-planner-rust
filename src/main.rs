mod exercises;
mod workouts;

use std::collections::HashMap;

use exercises::routes::{get_exercise,delete_exercise, create_exercise, update_exercise};
use workouts::routes::{get_workout, post_workout};
use warp::{filters::method::delete, Filter};

#[tokio::main]
async fn main() {
    let routes = get_exercise() 
        .or(create_exercise())
        .or(delete_exercise())
        .or(update_exercise())
        .or(get_workout())
        .or(post_workout())
        .with(warp::cors().allow_any_origin());

    warp::serve(routes)
        .run(([127, 0, 0,1], 3030))
        .await;
}
