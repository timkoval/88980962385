use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::{ApiContext, Error, Result};

#[derive(Serialize, Deserialize)]
struct Movie {
    id: String,
    name: String,
    year: u16,
    was_good: bool,
}

pub(crate) fn router() -> Router<ApiContext> {
    Router::new()
        .route("/movie/:id", get(get_movie))
        .route("/movie", post(create_movie))
}

async fn create_movie(
    mut ctx: State<ApiContext>,
    Json(req): Json<Movie>,
) -> Result<(StatusCode, Json<Movie>)> {
    let movie_id = req.id.clone();
    let movie_str = serde_json::to_string(&req)?;
    ctx.storage.insert(movie_id, movie_str);
    Ok((StatusCode::CREATED, Json(req)))
}

async fn get_movie(
    ctx: State<ApiContext>,
    Path(id): Path<String>,
) -> Result<(StatusCode, Json<Movie>)> {
    if let Some(movie) = ctx.storage.get(&id) {
        let movie = serde_json::from_str(movie)?;
        Ok((StatusCode::OK, Json(movie)))
    } else {
        Err(Error::NotFound) // might be better returning empty object
    }
}
