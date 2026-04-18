use std::sync::OnceLock;

use tokio::signal::unix::{SignalKind, signal};

use crate::{db::Db, prelude::*};

mod api;
mod constants;
mod db;
mod env;
mod prelude;
mod pswd;
mod types;
mod utils;

static DB: OnceLock<Db> = OnceLock::new();

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let db = Db::new().await?;
    DB.get_or_init(|| db);

    let addr = std::env::var(crate::env::LISTEN_ADDR).unwrap_or_else(|_| String::from("0.0.0.0"));
    let listener = tokio::net::TcpListener::bind(addr.as_str()).await?;

    let mut int = signal(SignalKind::interrupt())?;
    let mut hup = signal(SignalKind::hangup())?;
    let mut term = signal(SignalKind::terminate())?;

    tokio::select! {
        _ = axum::serve(listener, api::make_router()) => {},
        _ = int.recv() => {},
        _ = hup.recv() => {},
        _ = term.recv() => {},
    }

    DB.get()
        .expect("Initialized at the start of main")
        .cleanup()
        .await?;

    Ok(())
}
