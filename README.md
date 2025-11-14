# Real-time Streaming Dashboard — Plan & Starter Code

A Real-time Streaming Dashboard (Rust backend streaming metrics over WebSockets + React frontend displaying live charts).


## Quick overview

What does this app do?

## The backend 😎
The backend of this application is built in Rust backend (Axum + Tokio) and it exposes:

A WebSocket endpoint called "/ws" that emits streaming metric events (JSON) every 1s (configurable).

A static-file handler to serve the frontend.

## The frontend 📊🌸 
A React frontend that connects to the WebSocket, receives metric events, and renders them in an updating chart.
Link => WIP (@TODO)

## Topics covered

- Rust async programming, WebSocket handling, serialization with serde library, and basic observability.

- Integration of a modern frontend with real-time streaming and reactive UI.

## Tech stack

- Backend: Rust, axum, tokio, serde, tracing (optional).

- Frontend: React (Vite), TypeScript (optional), Chart.js or lightweight visualization.

- Communication: WebSockets (JSON messages).

![alt text](<Captura desde 2025-11-14 20-33-45.png>)

## Roadmap 

- Rust server that sends simulated metrics + React client charts. 

- Authentication (token on WS handshake).

- Persistent ingestion: write metrics to SQLite or Append-only log.

- Backpressure & rate limiting.

- Multiple topics/rooms (e.g., per-host streams) and filtering.

- Observability: tracing, metrics, Prometheus exporter.

- Benchmark & optimize: CPU/latency tests, per-message serialization formats (CBOR vs JSON).