## Real-time Streaming Dashboard — Plan & Starter Code

A Real-time Streaming Dashboard (Rust backend streaming metrics over WebSockets + React frontend displaying live charts).


## Quick overview

What does this app do?

## The backend :cool:
The backend of this application is built in Rust backend (Axum + Tokio) and it exposes:

A WebSocket endpoint called "/ws" that emits streaming metric events (JSON) every 1s (configurable).

A static-file handler to serve the frontend.

## The frontend :flower:
A React frontend that connects to the WebSocket, receives metric events, and renders them in an updating chart.

## Topics covered

- Rust async programming, WebSocket handling, serialization with serde library, and basic observability.

- Integration of a modern frontend with real-time streaming and reactive UI.

## Tech stack

Backend: Rust, axum, tokio, serde, tracing (optional).

Frontend: React (Vite), TypeScript (optional), Chart.js or lightweight visualization.

Communication: WebSockets (JSON messages).

Minimal architecture
+----------------+        WebSocket         +----------------+
|                | <----------------------> |                |
|   Frontend     |                         |   Rust Server  |
|  (React)       |                         |  (axum + tokio) |
+----------------+                         +----------------+
        |                                          |
        | HTTP static files (optional)             | Simulated metrics or real sources
        v                                          v
   Browser (charts)                          Persists/streams/collects
Message schema (JSON)
{
  "metric": "cpu",        // string: metric name
  "value": 42.3,           // number
  "timestamp": 1690000000  // unix seconds (or ISO string)
}

You can extend to batch arrays of metrics per message if needed.

### Roadmap 

Rust server that sends simulated metrics + React client charts. 

Authentication (token on WS handshake).

Persistent ingestion: write metrics to SQLite or Append-only log.

Backpressure & rate limiting.

Multiple topics/rooms (e.g., per-host streams) and filtering.

Observability: tracing, metrics, Prometheus exporter.

Benchmark & optimize: CPU/latency tests, per-message serialization formats (CBOR vs JSON).