mod models;
mod handlers;
mod routes;

use routes::routes;
use routes::post_routes;
use warp::filters::method::post;
use warp::Filter;

#[tokio::main]
async fn main() {
    let routes = routes::routes();
    let post_routes = routes::post_routes();

    warp::serve(routes)
        .run(([127, 0, 0,1], 3030))
        .await;

    warp::serve(post_routes)
    .run(([127, 0, 0,1], 3030))
    .await;
}
