pub mod http;

use axum::Router;
use http::movies;
use std::collections::HashMap;
use tokio::net::TcpListener;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    // 401 Unauthorized
    #[error("authorization required")]
    Unauthorized,

    // 403 Forbidden
    #[error("user may not perform this action")]
    Forbidden,

    // 404 Not Found
    #[error("resource not found")]
    NotFound,

    #[error(transparent)]
    SerializationError(#[from] serde_json::Error),
    // // 422 Unprocessable Entity
    // #[error("invalid request body")]
    // UnprocessableEntity {
    //     errors: HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>,
    // },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone)]
pub(crate) struct ApiContext {
    pub storage: HashMap<String, String>, // TODO: wrap in threadsafe
}

pub async fn serve() {
    let api_context = ApiContext {
        storage: HashMap::new(),
    };

    let app = api_router(api_context);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app)
        .await
        .expect("failed to run HTTP server");
}

fn api_router(api_context: ApiContext) -> Router {
    Router::new()
        .merge(movies::router())
        // .layer(TraceLayer::new_for_http())
        .with_state(api_context)
}
