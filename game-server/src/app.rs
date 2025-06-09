use std::time::Duration;

use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use bevy_internal::{
    app::{App, Startup},
    core::FrameCountPlugin,
    diagnostic::{
        DiagnosticsPlugin, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
    },
    log::{tracing_subscriber::Layer, Level, LogPlugin},
    prelude::TransformPlugin,
};

pub fn start_bevy_app() {
    let mut app = App::new();

    // app.insert_resource(ServerResource::default());
    //
    // app.add_plugins(TransformPlugin)
    //     .add_plugins(MinimalPlugins)
    //     .add_plugins(FrameCountPlugin)
    //     // Add DiagnosticPlugin
    //     .add_plugins((
    //         DiagnosticsPlugin,
    //         FrameTimeDiagnosticsPlugin,
    //         SystemInformationDiagnosticsPlugin,
    //         LogDiagnosticsPlugin {
    //             wait_duration: Duration::from_secs(10),
    //             ..Default::default()
    //         },
    //     ))
    //     .add_plugins(LogPlugin {
    //         filter: "wgpu=error,bevy_ecs=trace".to_string(),
    //         level: Level::INFO,
    //         custom_layer: |_| {
    //             let layer = crate::tracing::ExecutionTiming {};
    //             Some(layer.boxed())
    //         },
    //     })
    //     .add_plugins(NetworkingPlugin)
    //     .add_plugins(GameStatePlugin)
    //     .add_plugins(LobbyPlugin)
    //     .add_plugins(MetricPlugin);

    // app.add_systems(Startup, ready);

    app.run();
}

pub async fn start_http_server() {
    let app = Router::new().route("/ping", get(|| async { "pong" }));
    // .route("/metrics", get(metrics));

    let metric_host = std::env::var("METRIC_HOST").unwrap_or("0.0.0.0".to_owned());
    let metric_port = std::env::var("METRIC_PORT").unwrap_or("9669".to_owned());
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", metric_host, metric_port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

// pub async fn metrics() -> impl IntoResponse {
//     let text = METRICS.metrics();
//     (StatusCode::OK, text)
// }
