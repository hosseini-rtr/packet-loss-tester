# 📡 packet-loss-analyzer

**Open-source, self-hosted tool to test packet loss, latency, and jitter on the connection path between the user and this server.**
Built with **React.js (TypeScript)**, **Rust (WebSocket Echo Server)**, and **Docker**.

Inspired by tools like [packetlosstest.com](https://packetlosstest.com) — focused on understanding the direct connection quality to your web application.

---

## 🚀 Features

- ✅ Real-time packet loss, latency (RTT), and jitter testing for the **User \<-\> Server** path.
- 📈 Live visualization with charts and summary metrics, calculated client-side.
- ⚡ Fast and efficient **Rust-based WebSocket echo server** for minimal server-side overhead.
- 🔌 Simple WebSocket API for test communication.
- 🐳 Easy to deploy with Docker.
- 🛠️ Open source, modular, and extensible.

---

## 🖼️ Architecture

The application consists of a frontend UI that also runs the test logic, and a lightweight Rust backend that acts as a WebSocket echo server.

```
+---------------------------------+      +---------------------------------+
|   React Frontend (UI)           | <==> | Rust Backend (WebSocket Echo    |
| (Browser - Sends/Receives Test  |      | Server)                         |
|  Messages, Calculates Metrics)  |      |                                 |
+---------------------------------+      +---------------------------------+
```

- **Frontend (React.js + TypeScript):** Provides the user interface, initiates tests, sends test messages over WebSockets, receives echoed messages, and calculates all metrics (packet loss, RTT, jitter).
- **Backend (Rust):** A high-performance, lightweight WebSocket server that immediately echoes back any message it receives from a client, allowing the client to measure connection quality.

---

## 📦 Tech Stack

| Layer    | Tech                                                                                  |
| :------- | :------------------------------------------------------------------------------------ |
| Frontend | React.js (with Vite) + TypeScript + TailwindCSS                                       |
| Backend  | Rust (with a WebSocket library like `tokio-tungstenite` or `axum`/`warp`/`actix-web`) |
| Realtime | WebSockets (client-driven test messages, server echo) + JSON (for messages)           |
| DevOps   | Docker + Docker Compose                                                               |

---

## 🔧 Usage

### 1\. Clone the repo

```bash
git clone https://github.com/amir-mersad/packet-loss-analyzer.git # Or your repo URL
cd packet-loss-analyzer
```

### 2\. Configure (if needed)

Ensure backend WebSocket port in `docker-compose.yml` or Rust config matches the URL used by the frontend.

### 3\. Build & Run with Docker

```bash
docker-compose up --build
```

This spins up:

- Frontend (React App) at `http://localhost:3000` (or your configured port)
- Rust WebSocket Echo Server at `ws://localhost:8080/ws` (or your configured backend port and path)

Access the frontend URL in your browser to start testing your connection to the server.

---

## 📡 API Overview (WebSocket Communication)

The primary communication happens over a single WebSocket endpoint (e.g., `ws://your-server-address/ws`).

**Client-to-Server Message (Test Packet):**
The client sends messages to the server to be echoed.
_Example:_

```json
{
  "seq": 123, // Sequence number
  "ts": 1678886400100 // Client's send timestamp (milliseconds)
}
```

**Server-to-Client Message (Echoed Packet):**
The server immediately sends back the exact message it received.
_Example:_

```json
{
  "seq": 123,
  "ts": 1678886400100
}
```

The frontend uses these echoed messages to calculate packet loss (by checking sequence numbers) and RTT (using the original `ts` and current time).

---

## 🖥️ Rust Backend (Echo Server)

The Rust application is not a standalone CLI for general network testing in this architecture. Instead, it's a dedicated WebSocket server whose primary role is to:

1.  Accept WebSocket connections.
2.  Receive messages from connected clients.
3.  Immediately send (echo) the received message back to the _same_ client.

It's designed to be lightweight and fast to ensure it doesn't significantly contribute to the measured latency.

---

## 📂 Repo Structure (Example)

```
packet-loss-analyzer/
├── backend/                  # Rust WebSocket Echo Server
│   ├── src/
│   │   └── main.rs
│   └── Cargo.toml
├── frontend/                 # React + Tailwind UI & Test Logic
│   ├── src/
│   └── package.json
├── docker/                   # Dockerfiles for frontend & backend
│   └── docker-compose.yml
├── README.md
└── LICENSE
```

---

## 🧪 Test Settings & Interpretation

The test measures the quality of your connection _to this server_. Key metrics:

- **Packet Loss %:** Percentage of test messages that were sent by your browser but not echoed back by the server.
- **RTT (ms):** Round Trip Time. Average time for a message to go from your browser to the server and back.
- **Jitter (ms):** Variation in RTT. High jitter means an unstable connection.

(Optional: Link to a more detailed guide if you create one)
➡️ `[See Test Interpretation Guide](docs/test-interpretation.md)`

---

## 💡 Roadmap

- [ ] Export client-side calculated results to CSV/JSON.
- [ ] Enhanced real-time visualizations (e.g., RTT distribution).
- [ ] Store test history locally in the browser (`localStorage`/`IndexedDB`).
- [ ] Option for configurable test message payload size.
- [ ] UI improvements: themes, more responsive design.
- [ ] Explore WebRTC DataChannels as an alternative transport for comparison.

---

## 🤝 Contributing

Contributions are welcome\! Please open issues to report bugs or suggest features, or submit Pull Requests with improvements.

See [`CONTRIBUTING.md`](CONTRIBUTING.md) (you'll need to create this file) for more details.

---

## 📄 License

MIT License. See [`LICENSE`](LICENSE) (ensure this file exists).

---

## 🌐 Maintainers

This project is maintained by [`Amir Mersad`](https://github.com/amir-mersad) and shared with the open-source community.
`
---

## 🙏 Acknowledgements

- Inspired by the concept of [packetlosstest.com](https://packetlosstest.com) for measuring connection quality.
- Built with love using open-source tools like React, Rust, Docker, and many great libraries. ❤️
