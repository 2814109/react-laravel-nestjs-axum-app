use crate::infrastructure::repository::task;
use hyper::{header,Method};
use tower_http::cors::{CorsLayer, AllowOrigin};

use axum::{routing::{get,post}, Router};

// use axum::{
//     extract::Path,
// };

pub fn app() -> Router {
    let router: Router = Router::new()
        .route("/task/new", post(
            {
            move |body| task::create_one_task(body)
        }))
         .route("/task/logical_delete", post(
            {
            move |body| task::logical_delete_for_task(body)
        }))
          .route("/task/update", post(
            {
            move |body| task::update_task(body)
        }))
        .route("/task/physical_delete", post(
            {
            move |body| task::physical_delete(body)
        }))
        .route("/task/:id",get(task::get_task_by_id))
        .route("/task", post(|| async { task::create_one("post test").await }))
        .route("/tasks", get(|| async { task::get_all_task().await }))
      .layer(
     CorsLayer::new()
         .allow_origin(AllowOrigin::exact("http://localhost:5173".parse().unwrap()))
        .allow_headers(vec![
        header::ACCEPT,
        header::ACCEPT_LANGUAGE,
        header::AUTHORIZATION,
        header::CONTENT_LANGUAGE,
        header::CONTENT_TYPE,
    ])
         .allow_methods(vec![Method::GET,Method::POST]),);

    return router;
}
