mod models;
mod handlers;
mod routes;

use std::collections::HashMap;

use routes::{delete_routes, post_routes};
use warp::{filters::method::delete, Filter};

#[tokio::main]
async fn main() {
    let get_routes = routes::get_routes();
    let post_route = routes::post_routes();
    let delete_route = routes::delete_routes();

    let routes = get_routes
        .or(post_routes())
        .or(delete_routes())
        .with(warp::cors().allow_any_origin());


    warp::serve(routes)
        .run(([127, 0, 0,1], 3030))
        .await;
}
