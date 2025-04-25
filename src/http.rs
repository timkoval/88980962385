pub mod movies;

#[derive(Clone)]
pub(crate) struct ApiContext {
    pub config: Arc<Config>,
    pub db: PgPool,
}
