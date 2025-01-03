use anyhow::Context;
use axum::{
    routing::{get, patch, post},
    Router,
};
use config::AppConfig;
use error::AppError;
use handlers::*;
use models::*;
use sqlx::PgPool;
use std::{fmt, ops::Deref, sync::Arc};
use utils::{DecodingKey, EncodingKey};

pub mod config;
mod error;
mod handlers;
mod models;
mod utils;

#[derive(Debug, Clone)]
pub(crate) struct AppState {
    pub(crate) inner: Arc<AppStateInner>,
}

#[allow(unused)]
#[derive(Clone)]
struct AppStateInner {
    pub(crate) config: AppConfig,
    pub(crate) dk: DecodingKey,
    pub(crate) ek: EncodingKey,
    pub(crate) pool: PgPool,
}

pub async fn get_router(config: AppConfig) -> Result<Router, AppError> {
    let state = AppState::try_new(config).await?;
    let api = Router::new()
        .route("/signin", post(signin_handler))
        .route("/signup", post(signup_handler))
        .route("/chat", get(list_chat_handler).post(create_chat_handler))
        .route(
            "/chat/:id",
            patch(update_chat_handler)
                .delete(delete_chat_handler)
                .post(send_message_handler),
        )
        .route("/chat/:id/messages", get(list_message_handler))
        .with_state(state.clone());
    let app = Router::new()
        .route("/", get(index_handler))
        .nest("/api", api)
        .with_state(state);
    Ok(app)
}

impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AppState {
    pub async fn try_new(config: AppConfig) -> Result<Self, AppError> {
        let dk = DecodingKey::load(&config.auth.pk).context("load ek failed")?;
        let ek = EncodingKey::load(&config.auth.sk).context("load dk failed")?;
        let pool = PgPool::connect(&config.server.db_url)
            .await
            .context("connect to db failed")?;
        Ok(Self {
            inner: Arc::new(AppStateInner {
                config,
                dk,
                ek,
                pool,
            }),
        })
    }
}

impl fmt::Debug for AppStateInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppStateInner")
            .field("config", &self.config)
            .finish()
    }
}

#[cfg(test)]
impl AppState {
    pub async fn new_for_test(
        config: AppConfig,
    ) -> Result<(sqlx_db_tester::TestPg, Self), AppError> {
        use sqlx_db_tester::TestPg;
        let dk = DecodingKey::load(&config.auth.pk).context("load pk failed")?;
        let ek = EncodingKey::load(&config.auth.sk).context("load sk failed")?;

        let server_url = match config.server.db_url.rsplit_once('/') {
            Some((prefix, _)) => &prefix.to_string(),
            None => &config.server.db_url,
        };

        let tdb = TestPg::new(server_url.clone(), std::path::Path::new("../migrations"));
        let pool = tdb.get_pool().await;

        let state = Self {
            inner: Arc::new(AppStateInner {
                config,
                dk,
                ek,
                pool,
            }),
        };
        Ok((tdb, state))
    }
}
