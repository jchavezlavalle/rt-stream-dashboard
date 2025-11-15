# Real-time Streaming Dashboard

A Real-time Streaming Dashboard (Rust backend streaming CPU metrics over WebSockets + React frontend displaying live charts).

## Quick overview

What does this app do?

## The backend 😎
The backend of this application is built in Rust backend (Axum + Tokio) and it exposes:

A WebSocket endpoint called "/ws" that emits streaming metric events (JSON) every 1s.

A static-file handler to serve the frontend.

## The frontend 📊🌸 
A React frontend that connects to the WebSocket, receives the CPU metric events, and renders them in an updating chart.
Find the repo [here](https://github.com/jchavezlavalle/rt-dashboard-frontend).

## Topics covered

- Rust async programming, WebSocket handling, serialization with serde library, and basic observability.

- Integration of a modern frontend with real-time streaming and reactive UI.

## Tech stack

- Backend: Rust, axum, tokio, serde, tracing (optional).

- Frontend: React, Vite, TypeScript.

- Communication: WebSockets (JSON messages).

![alt text](<./src/assets/Captura desde 2025-11-14 20-33-45.png>)

## Roadmap 

- Rust server that sends simulated metrics + React client charts. 

- Authentication (token on WS handshake).

- Persistent ingestion: write metrics to SQLite or Append-only log.

- Backpressure & rate limiting.

- Multiple topics/rooms (e.g., per-host streams) and filtering.

- Observability: tracing, metrics, Prometheus exporter.

- Benchmark & optimize: CPU/latency tests, per-message serialization formats (CBOR vs JSON).

## Run the app

## Backend

Run the following script:

```
cargo run
```
To send the requests open another terminal, you would need wscat 🐱 installed, 
if you don't have it you can install it like this:

```
npm i wscat
```

After this just run on another terminal:

```
wscat -c ws://127.0.0.1:3000/ws
```

You would see something pretty cool like this:

![logs from wscat](<./src/assets/Captura desde 2025-11-14 21-22-40.png>)

## Frontend 

Find it here -> [https://github.com/jchavezlavalle/rt-dashboard-frontend](https://github.com/jchavezlavalle/rt-dashboard-frontend)

This is a sneak-peak of what the frontend looks like:
![Kawaii UI](<./src/assets/Captura desde 2025-11-15 18-23-52.png'>)