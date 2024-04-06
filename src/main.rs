mod exercises;
mod workouts;

use std::collections::HashMap;

use exercises::routes::{get_exercises,delete_exercises, post_exercises, update_exercises};
use workouts::routes::get_workouts;
use warp::{filters::method::delete, Filter};

#[tokio::main]
async fn main() {
    let routes = get_exercises() 
        .or(post_exercises())
        .or(delete_exercises())
        .or(update_exercises())
        .or(get_workouts())
        .with(warp::cors().allow_any_origin());

    warp::serve(routes)
        .run(([127, 0, 0,1], 3030))
        .await;
}
