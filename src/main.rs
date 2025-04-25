use axum::http::StatusCode;
use axum::Json;

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
    // // 422 Unprocessable Entity
    // #[error("invalid request body")]
    // UnprocessableEntity {
    //     errors: HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>,
    // },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

fn main() {
    // Create Axum server with the following endpoints:
    // 1. GET /movie/{id} - This should return back a movie given the id
    // 2. POST /movie - this should save move in a DB (HashMap<String, Movie>). This movie will be sent
    // via a JSON payload.

    // As a bonus: implement a caching layer so we don't need to make expensive "DB" lookups, etc.
}
