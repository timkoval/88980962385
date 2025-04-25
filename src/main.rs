use axum::http::StatusCode;
use axum::Json;

fn main() {
    // Create Axum server with the following endpoints:
    // 1. GET /movie/{id} - This should return back a movie given the id
    // 2. POST /movie - this should save move in a DB (HashMap<String, Movie>). This movie will be sent
    // via a JSON payload.

    // As a bonus: implement a caching layer so we don't need to make expensive "DB" lookups, etc.
}
