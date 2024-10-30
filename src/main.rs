#![forbid(unsafe_code)]
#![deny(rust_2018_idioms)]

use std::{convert::Infallible, env, net::Ipv4Addr, path::PathBuf, time::Duration};

use axum::{
    http::{header::CACHE_CONTROL, HeaderValue, Response},
    Router,
};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    cors::{AllowHeaders, AllowOrigin, CorsLayer},
    services::{ServeDir, ServeFile},
    timeout::TimeoutLayer,
};

#[tokio::main]
async fn main() {
    let root_dir = env::var("DIR").unwrap_or(".".to_owned());

    let serve_dir = ServeDir::new(root_dir.clone()).append_index_html_on_directories(false);

    let router = if env::var("SPA").is_ok() {
        Router::new().nest_service(
            "/",
            serve_dir.fallback(ServeFile::new(PathBuf::from(root_dir).join("index.html"))),
        )
    } else {
        Router::new().nest_service("/", serve_dir)
    }
    .layer(
        ServiceBuilder::new()
            .layer(TimeoutLayer::new(Duration::from_secs(10)))
            .and_then(|mut res: Response<_>| async {
                if res.status().is_success() {
                    res.headers_mut().insert(
                        CACHE_CONTROL,
                        HeaderValue::from_static("max-age=600, public"),
                    );
                }

                Ok::<_, Infallible>(res)
            })
            .layer(
                CorsLayer::new()
                    .allow_origin(AllowOrigin::any())
                    .allow_headers(AllowHeaders::any()),
            )
            .layer(
                CompressionLayer::new()
                    .br(env::var("BROTLI").is_ok())
                    .gzip(env::var("GZIP").is_ok())
                    .no_deflate()
                    .no_zstd(),
            ),
    );

    let listener = TcpListener::bind((
        env::var("HOST")
            .map(|val| val.parse().expect("Invalid HOST env variable"))
            .unwrap_or(Ipv4Addr::UNSPECIFIED),
        env::var("PORT")
            .map(|val| val.parse().expect("Invalid PORT env variable"))
            .unwrap_or(8080),
    ))
    .await
    .expect("Unable to create listener");

    axum::serve(listener, router)
        .with_graceful_shutdown(wait_until_shutdown())
        .await
        .expect("Unable to start server");
}

#[cfg(unix)]
async fn wait_until_shutdown() {
    use tokio::signal::unix::{signal, SignalKind};

    let mut sigterm = signal(SignalKind::terminate()).expect("Failed to install SIGTERM handler");
    let mut sigint = signal(SignalKind::interrupt()).expect("Failed to install SIGINT handler");

    tokio::select! {
        _ = sigterm.recv() => {},
        _ = sigint.recv() => {},
    }
}

#[cfg(not(unix))]
async fn wait_until_shutdown() {
    use tokio::signal::windows;

    let mut ctrl_c = windows::ctrl_c().expect("Failed to install Ctrl+C handler");
    let mut ctrl_break = windows::ctrl_break().expect("Failed to install Ctrl+Break handler");

    tokio::select! {
        _ = ctrl_c.recv() => {},
        _ = ctrl_break.recv() => {},
    }
}
