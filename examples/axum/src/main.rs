mod database;
mod error;
mod state;
mod user;

use anyhow::Result;
use tokio::net::TcpListener;
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable};

use crate::state::AppState;

use self::user::routes::UserRoutes;

#[tokio::main]
async fn main() -> Result<()> {
    let router = OpenApiRouter::new().merge(UserRoutes::router());

    let (router, openapi) = OpenApiRouter::new().nest("/api", router).split_for_parts();

    let router = router
        .merge(Scalar::with_url("/api/reference", openapi))
        .with_state(AppState::default());

    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, router).await?;

    Ok(())
}
