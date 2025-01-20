use std::net::SocketAddr;

use axum::{routing::*, Json};
use clap::Parser;
use tokio::net::TcpListener;

#[derive(clap::Parser)]
struct Args {
    #[clap(long, env)]
    host: SocketAddr,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let Args { host } = Args::parse();

    env_logger::init();

    let router = axum::Router::new()
        .route("/", get(index));

    let listener = match TcpListener::bind(host).await {
        Ok(l) => l,
        Err(err) => {
            log::error!("{err}");
            return;
        }
    };

    if let Err(err) = axum::serve(listener, router).await {
        log::error!("{err}");
        return;
    }
}

#[derive(serde::Serialize)]
struct Message {
    message: String,
}

async fn index() -> Json<Message> {
    Json(Message {
        message: "Hello, world!".into(),
    })
}
