use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use serde::Serialize;
use std::{net::SocketAddr, time::Duration};
use tokio::time::interval;
use tracing_subscriber;

#[derive(Serialize)]
struct Metric {
    metric: String,
    value: f64,
    timestamp: u64,
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    let mut ticker = interval(Duration::from_secs(1));

    loop {
        tokio::select! {
            _ = ticker.tick() => {
                let m = Metric {
                    metric: "cpu".to_string(),
                    value: rand::random::<f64>() * 100.0,
                    timestamp: chrono::Utc::now().timestamp() as u64,
                };

                let text = serde_json::to_string(&m).unwrap();

                if socket.send(Message::Text(text.into())).await.is_err() {
                    break; // client disconnected
                }
            }

            maybe_msg = socket.recv() => {
                match maybe_msg {
                    Some(Ok(msg)) => match msg {
                        Message::Text(t) => tracing::info!("recv text: {}", t),
                        Message::Ping(p) => { let _ = socket.send(Message::Pong(p)).await; }
                        Message::Close(_) => break,
                        _ => {}
                    },
                    _ => break,
                }
            }
        }
    }

    tracing::info!("client disconnected");
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/ws", get(ws_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}