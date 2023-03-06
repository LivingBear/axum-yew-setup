use tokio::fs;
use std::path::PathBuf;

use axum::body::{Body, boxed, BoxBody};
use axum::http::{StatusCode, Response, Request};
use axum::{routing::get, routing::post, Router};
use axum::error_handling::HandleErrorLayer;
use axum::response::IntoResponse;

use clap::Parser;
use tower::{ServiceBuilder, ServiceExt, BoxError};
use tower_http::trace::TraceLayer;
use tokio::time::Duration;

use crate::routes::hi::hi_route;
use crate::routes::email::get_email_list::get_email_list;
use crate::routes::user::{create_user, get_user, update_user, delete_user};
// use crate::routes::auth::login::login;
use crate::routes::auth::register::registration_mirror;
use crate::routes::auth::auth::auth_routes;


// use crate::routes::auth::login;
// use crate::routes::auth::register;
use crate::structs::opt_struct::Opt;
use tower_http::services::ServeDir;

pub async fn get_router() -> Router {
    Router::new()
        .route("/api/hi", get(hi_route))
        .route("/api/get_email_list", get(get_email_list))
        .route("/api/register", post(registration_mirror))
        .route("/api/user", get(get_user).post(create_user))
        .route("/api/user/:id", get(get_user).patch(update_user).delete(delete_user))
        .nest("/auth", auth_routes())
        .layer(
            ServiceBuilder::new()
                // Handle errors from middleware
                .layer(HandleErrorLayer::new(handle_error))
                .layer(TraceLayer::new_for_http())
                .load_shed()
                .concurrency_limit(1024)
                .timeout(Duration::from_secs(10))
                .into_inner(),
        )
        .fallback_service(get(fallback_service))
        .layer(ServiceBuilder::new())
        // Authorize requests using `MyAuth`
}

async fn fallback_service(req: Request<Body>) -> Response<BoxBody> {
    let opt = Opt::parse();
    match ServeDir::new(&opt.static_dir).oneshot(req).await {
        Ok(res) => {
            let status = res.status();
            match status {
                StatusCode::NOT_FOUND => {
                    let index_path = PathBuf::from(&opt.static_dir).join("index.html");
                    let index_content = match fs::read_to_string(index_path).await {
                        Err(_) => {
                            return Response::builder()
                                .status(StatusCode::NOT_FOUND)
                                .body(boxed(Body::from("index file not found")))
                                .unwrap()
                        }
                        Ok(index_content) => index_content,
                    };

                    Response::builder()
                        .status(StatusCode::OK)
                        .body(boxed(Body::from(index_content)))
                        .unwrap()
                }
                _ => res.map(boxed),
            }
        }
        Err(err) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(boxed(Body::from(format!("error: {}", err))))
            .expect("error response"),
    }

}

async fn handle_error(error: BoxError) -> impl IntoResponse {
    if error.is::<tower::timeout::error::Elapsed>() {
        return (StatusCode::REQUEST_TIMEOUT, String::from("request timed out"));
    }

    if error.is::<tower::load_shed::error::Overloaded>() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            String::from("service is overloaded, try again later"),
        );
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        String::from(format!("Unhandled internal error: {}", error)),
    )
}