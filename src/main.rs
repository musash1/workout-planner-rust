mod exercises;
mod workouts;

use std::sync::Arc;

use exercises::routes::{get_exercise, delete_exercise, create_exercise, update_exercise};
use exercises::models::Exercise;
use workouts::models::Workout;
use warp::filters::path::{FullPath, Tail};
use warp::reject::Rejection;
use warp::reply::Reply;
use warp::http::Uri;
use warp::hyper::{Response, StatusCode};
use workouts::routes::{get_workout, post_workout, delete_workout, update_workout};
use warp::Filter;
use utoipa::OpenApi;
use utoipa_swagger_ui::Config;

#[tokio::main]
async fn main() {

    let config = Arc::new(Config::from("/api-doc.json"));

    #[derive(OpenApi)]
    #[openapi(
        paths(exercises::handlers::get_exercises, exercises::handlers::create_exercise, 
              exercises::handlers::delete_exercise, exercises::handlers::update_exercise,
              workouts::handlers::get_workout, workouts::handlers::create_workout,
              workouts::handlers::delete_workout, workouts::handlers::update_workout),
        components(
            schemas(Exercise, Workout)
        ),
        tags(
            (name = "Workout", description = "Workouts management API")
        )
    )]
    struct ApiDoc;

    let api_doc = warp::path("api-doc.json")
        .and(warp::get())
        .map(|| warp::reply::json(&ApiDoc::openapi()));

    let swagger_ui = warp::path("swagger-ui")
        .and(warp::get())
        .and(warp::path::full())
        .and(warp::path::tail())
        .and(warp::any().map(move || config.clone()))
        .and_then(serve_swagger);

    let routes = get_exercise() 
        .or(create_exercise())
        .or(delete_exercise())
        .or(update_exercise())
        .or(get_workout())
        .or(post_workout())
        .or(delete_workout())
        .or(update_workout())
        .with(warp::cors().allow_any_origin());

    warp::serve(api_doc.or(swagger_ui).or(routes))
        .run(([127, 0, 0,1], 3030))
        .await;
}

async fn serve_swagger(
    full_path: FullPath,
    tail: Tail,
    config: Arc<Config<'static>>,
) -> Result<Box<dyn Reply + 'static>, Rejection> {
    if full_path.as_str() == "/swagger-ui" {
        return Ok(Box::new(warp::redirect::found(Uri::from_static(
            "/swagger-ui/",
        ))));
    }

    let path = tail.as_str();
    match utoipa_swagger_ui::serve(path, config) {
        Ok(file) => {
            if let Some(file) = file {
                Ok(Box::new(
                    Response::builder()
                        .header("Content-Type", file.content_type)
                        .body(file.bytes),
                ))
            } else {
                Ok(Box::new(StatusCode::NOT_FOUND))
            }
        }
        Err(error) => Ok(Box::new(
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(error.to_string()),
        )),
    }
}
