mod models;
mod handlers;
mod routes;

use routes::routes;
use warp::Filter;

#[tokio::main]
async fn main() {
    let routes = routes::routes();

    warp::serve(routes)
        .run(([127, 0, 0,1], 3030))
        .await;
}
