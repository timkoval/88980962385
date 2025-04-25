use std::collections::HashMap;

use axum::http::StatusCode;

#[derive(Serialize, Deserialize)]
struct Movie {
    id: String,
    name: String,
    year: u16,
    was_good: bool,
}

pub(crate) fn router() -> Router<ApiContext> {
    Router::new()
        .route("/movie/:id", get(get_user))
        .route("/movie", post(create_user))
}

async fn create_movie(
    ctx: State<ApiContext>,
    Json(req): Json<Movie>,
) -> Result<(StatusCode, Json<Movie>)> {
    let
    (StatusCode::CREATED, Json(user))
}

async fn get_user(
    ctx: State<ApiContext>,
    Path(id): Path<u64>,
) -> Result<(StatusCode, Json<Movie>)> {
    (StatusCode::OK, Json(user))
}
