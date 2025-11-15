# Real-time Streaming Dashboard

A Real-time Streaming Dashboard built with Rust, which streams CPU metrics over (WebSockets)[https://developer.mozilla.org/en-US/docs/Web/API/WebSockets_API] + [React](https://github.com/facebook/react) frontend displaying live charts.

# 🧠 What is rt-stream-dashboard?

rt-stream-dashboard sends CPU/memory metrics through websockets using Rust. It uses ([Axum](https://github.com/tokio-rs/axum) + [Tokio](https://github.com/tokio-rs/tokio)) and it exposes:

A WebSocket endpoint called "/ws" that emits streaming metric events ( in JSON format ) every 1s.

A static-file handler to serve the frontend.

# ✨ Highlights 
- **💬 Async programming:** WebSocket handling.
- **⚡ Fast:** Made in Rust :crab: for performance.
- **👀 Basic observability:** Allows to manage and examine the state of logs, metrics and traces over time.
- **🗒️ Machine-parsable serialization:** Serialization with the (serde)[https://github.com/serde-rs/serde] library in JSON format, allowing a simpler data exchange.
- **🎨 Interactive UI:** Integration of a modern frontend with real-time streaming and reactive UI. React frontend that connects to the WebSocket, receives the CPU metric events, and renders them in an updating chart. See the repo [here](https://github.com/jchavezlavalle/rt-dashboard-frontend).

# Tech stack
- 😎 Backend: Rust, axum, tokio, serde, tracing (optional).
- 📊🌸 Frontend: React, Vite, TypeScript.
- 🦜💬 Communication: WebSockets (JSON messages).

![alt text](<Captura desde 2025-11-14 20-33-45.png>)

# Roadmap 

- Rust server that sends simulated metrics + React client charts.
- Authentication (token on WS handshake).
- Persistent ingestion: write metrics to SQLite or Append-only log.
- Backpressure & rate limiting.
- Multiple topics/rooms (e.g., per-host streams) and filtering.
- Observability: tracing, metrics, Prometheus exporter.
- Benchmark & optimize: CPU/latency tests.

# Usage

## Running the Application

```
cargo run
```
## Send requests using wscat 🐱

Make sure to have it installed first, otherwise run the following command:
```
npm i wscat
```

After this run on another terminal wscat:

```
wscat -c ws://127.0.0.1:3000/ws
```

You would see something pretty cool like this:

![alt text](<Captura desde 2025-11-14 21-22-40.png>)

# 📄 License
This project is licensed under the MIT License - see the LICENSE file for details.

# 🤝 Contributing
Contributions are welcome! Please feel free to submit a Pull Request.

# 📧 Contact
Created by jchavezlavalle
