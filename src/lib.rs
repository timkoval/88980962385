pub mod http;

#[derive(Clone)]
pub(crate) struct ApiContext {
    pub storage: HashMap<String, Movie>,
}

pub async fn serve() {
    let api_context = ApiContext {
        config: Arc::new(config),
        db,
    };

    let app = api_router(api_context);

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .context("failed to run HTTP server")
}

fn api_router(api_context: ApiContext) -> Router {
    Router::new()
        .merge(users::router())
        .merge(profiles::router())
        // .layer(TraceLayer::new_for_http())
        .with_state(api_context)
}
