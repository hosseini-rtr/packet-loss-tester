## Packet Loss Test Analysis and Technical Design Document (User to Server)

---

### 1. Introduction 📝

This document provides an analysis, explanation, and technical design for the `packet-loss-analyzer` tool. This tool is specifically designed to offer users a way to measure **packet loss, latency (RTT), and jitter in their internet connection directly to this application's server (Your Server)**. Understanding these metrics is crucial as they directly impact the user's experience with any real-time web application, including this one.

This project utilizes **React JS - TypeScript** for the user interface (UI). The UI will also handle the core logic of sending test messages and calculating metrics. The **Rust-based backend** will primarily function as a high-performance WebSocket echo server.

---

### 2. What is Packet Loss? 🤔

**Packet Loss** occurs when one or more packets (or in our case, WebSocket messages representing packets) of data traveling across a computer network fail to reach their destination. It can be caused by various factors, including:

- **User's Local Network Issues:** Problems with the user's Wi-Fi, modem, router, or local network congestion.
- **Internet Service Provider (ISP) Issues:** Problems within the user's ISP network or peering points.
- **General Internet Congestion:** Congestion on the broader internet paths between the user and Your Server.
- **Your Server's Network or Capacity:** Issues with Your Server's network connectivity, bandwidth, or processing capacity (though the echo logic is designed to be lightweight).
- **Signal Interference (for wireless networks):** Affecting the user's connection.

---

### 3. How the Packet Loss Test Works ⚙️ (User <-> Your Server Model)

The tool measures packet loss, RTT, and jitter on the path between the user's browser and Your Server using WebSockets:

1.  **Test Initiation by User:** The user navigates to the UI and initiates the test, possibly after setting parameters like test duration or number of test messages.
2.  **WebSocket Connection:** The React frontend establishes a WebSocket connection with Your Rust backend server.
3.  **Test Message Transmission (Client-Side):**
    - The **React frontend** begins sending a series of small WebSocket messages to Your Rust Server.
    - Each message contains a unique **sequence number** and a **client-side timestamp** (recorded just before sending).
    - The frequency and total number of messages are controlled by the frontend based on user settings or defaults (e.g., 5 messages per second for 20 seconds).
4.  **Message Echoing (Server-Side):**
    - Your **Rust backend server** receives each WebSocket message from the client.
    - For each message, the server **immediately sends the exact same message (or its payload) back to the originating client** over the same WebSocket connection (acting as an "echo"). The server does not need to interpret the content beyond what's necessary for echoing.
5.  **Response Reception and Metric Calculation (Client-Side):**
    - The **React frontend** listens for these echoed messages from the server.
    - For each echoed message received:
      - It matches the sequence number to an original sent message.
      - It calculates the **Round Trip Time (RTT)** using the stored client-side timestamp from the original message and the current time.
    - The frontend tracks:
      - Number of messages sent.
      - Number of echoed messages received.
      - Individual RTT values.
    - Based on this, the frontend calculates:
      - **Packet Loss Percentage:**
        $$\text{Packet Loss Percentage} = \left( \frac{\text{Messages Sent} - \text{Echoed Messages Received}}{\text{Messages Sent}} \right) \times 100\%$$
      - **Average RTT.**
      - **Jitter** (e.g., standard deviation of RTTs or average difference between consecutive RTTs).
6.  **Displaying Results:**
    - The React frontend displays the calculated packet loss, RTT, and jitter in real-time or as a summary upon test completion.

---

### 4. Key Parameters in Results Analysis 📊

- **Packet Loss Percentage:** Directly reflects the reliability of the connection between the user and Your Server.
  - **0%:** Ideal.
  - **1-2%:** May cause minor glitches in highly interactive applications.
  - **2-5%:** Likely to cause noticeable issues.
  - **Over 5%:** Indicates significant problems with the connection path.
- **Round Trip Time (RTT) or Ping:** The time taken for a message to travel from the user's browser to Your Server and back. Lower values mean a more responsive connection to your application. Measured in milliseconds (ms).
- **Jitter:** Variation in RTT values. High jitter on this path can lead to an inconsistent experience with real-time features of your application. Measured in milliseconds (ms).
- **Number of Test Messages (Sent/Received):** Indicates the sample size for the test. More messages can provide a more accurate average.

---

### 5. Interpreting and Troubleshooting Results 💡

Since the test measures the path **User <-> Your Server**:

- **High Packet Loss/RTT/Jitter:**
  - **Check User's Local Network:** Wi-Fi signal, other devices using bandwidth, modem/router issues. A wired connection might improve results.
  - **User's ISP:** The problem might be with the user's internet service provider.
  - **Internet Path:** Congestion or routing issues on the internet between the user and Your Server.
  - **Your Server Load/Network:** While the echo server is lightweight, extreme load on Your Server or its network uplink could degrade performance. This test can help identify if Your Server is becoming a bottleneck for user connections.
- **Inconsistent Results:** Running the test at different times can help identify if issues are related to peak usage hours.

---

### 🧱 6. Architecture

The system is a client-server model focused on WebSocket communication for the test.

```
+---------------------------------+      +---------------------------------+
|   React Frontend (UI)           | <==> | Rust Backend (WebSocket Echo    |
| (Browser - Sends/Receives Test  |      | Server)                         |
|  Messages, Calculates Metrics)  |      |                                 |
+---------------------------------+      +---------------------------------+
```

- **Frontend (React.js + TypeScript):**
  - Provides the UI for initiating tests and viewing results.
  - Manages the WebSocket connection.
  - Generates and sends test messages with sequence numbers and timestamps.
  - Receives echoed messages.
  - Calculates packet loss, RTT, and jitter.
  - Displays metrics.
- **Backend (Rust):**
  - A lightweight Rust application acting as a WebSocket server.
  - Listens for incoming WebSocket connections.
  - For each connected client, it echoes back any message it receives from that client.
  - Designed for high concurrency and low-latency echoing.

---

### 🧩 7. Components

#### 7.1. Frontend (React.js + TailwindCSS)

- **Framework:** React.js (with Vite).
- **Language:** TypeScript.
- **Styling:** TailwindCSS.
- **Responsibilities:**
  - **UI Rendering:** Test controls (start/stop, parameter inputs like duration/message count), real-time graphs/displays for metrics.
  - **WebSocket Management:** Establishing, maintaining, and closing the WebSocket connection. Handling connection errors.
  - **Test Logic:**
    - Looping to send test messages at a defined frequency (e.g., using `setInterval` or a more precise timer mechanism).
    - Generating unique sequence numbers for each message.
    - Timestamping messages before sending.
    - Storing sent message data (sequence number, timestamp) to match with echoes.
  - **Metrics Calculation:**
    - Counting sent and received (echoed) messages.
    - Calculating RTT for each received echo.
    - Calculating overall packet loss percentage.
    - Calculating average RTT and jitter.
  - **State Management:** Managing test state (idle, running, finished), results, and UI updates.
- **Test Parameters (configured by user or default):**

  | Parameter                    | Type    | Description                                                               |
  | :--------------------------- | :------ | :------------------------------------------------------------------------ |
  | `test_duration_s`            | Integer | Total duration of the test in seconds (e.g., 20)                          |
  | `messages_per_s`             | Integer | Number of test messages to send per second (e.g., 5)                      |
  | `message_payload_size_bytes` | Integer | (Optional) Size of an arbitrary payload in test messages (default: small) |

#### 7.2. Backend (Rust)

- **Language:** Rust.
- **Key Crates/Frameworks (Examples):**
  - **WebSockets/Web Framework:** `tokio-tungstenite` (for pure WebSocket server) or integrated WebSocket support in `actix-web`, `warp`, or `axum`.
  - **Async Runtime:** `tokio`.
  - **Serialization/Deserialization:** `serde` (if messages are JSON, though for simple echo, direct byte handling might be used).
- **Responsibilities:**
  - **WebSocket Server:** Accept and manage WebSocket connections from multiple clients.
  - **Message Echoing:** For every data message received from a client on its WebSocket, send the same data message back to that specific client immediately.
  - **Concurrency:** Handle many clients concurrently and efficiently.
  - **Low Latency:** Process and echo messages with minimal delay.
  - (Optional) Logging of connections and errors.

---

### 🧪 8. Sample Flow

1.  **User Interaction:** User opens the UI and clicks "Start Test."
2.  **WebSocket Connection:** The React frontend establishes a WebSocket connection to the Rust backend server.
3.  **Frontend Test Loop:**
    - The frontend starts a timer (e.g., to run for `test_duration_s` seconds).
    - On a regular interval (defined by `messages_per_s`), the frontend:
      - Creates a test message (e.g., a JSON string or byte array) containing a unique sequence number and the current `client_timestamp_ms`.
      - Sends this message via the WebSocket to the server.
      - Stores the sequence number and timestamp locally.
4.  **Backend Echo:**
    - The Rust backend receives the WebSocket message from a client.
    - It immediately sends the same message back to that client.
5.  **Frontend Receives Echo & Calculates:**
    - The frontend's WebSocket `onmessage` handler receives the echoed message.
    - It extracts the sequence number and original `client_timestamp_ms` from the payload.
    - It marks the corresponding sequence number as received.
    - It calculates RTT: `current_time_ms - client_timestamp_ms`.
    - It updates running totals for RTT, jitter calculations, and received packet count.
    - It updates the UI with the latest calculated metrics (loss percentage, average RTT, jitter).
6.  **Test Completion:**
    - When the test duration expires (or a set number of packets have been sent), the frontend stops sending messages.
    - It may wait a short additional period for any outstanding echoes to arrive.
    - It calculates and displays the final summary of packet loss, average RTT, and jitter.
    - The WebSocket connection might be closed or kept open for further tests.

---

### 🗂️ 9. Repository Structure (Example, adjusted)

```
packet-loss-analyzer/
├── frontend/                  # React.js + TypeScript UI & Test Logic
│   ├── public/
│   ├── src/
│   │   ├── components/        # UI elements
│   │   ├── hooks/             # Custom hooks (e.g., for test logic, WebSocket)
│   │   ├── services/          # WebSocket connection service
│   │   ├── utils/             # Utilities (e.g., timers, calculations)
│   │   ├── App.tsx
│   │   └── main.tsx
│   ├── package.json
│   └── vite.config.ts
├── backend/                   # Rust WebSocket Echo Server
│   ├── src/
│   │   ├── main.rs            # Entry point, WebSocket server setup
│   │   ├── ws_handler.rs      # Logic for handling WebSocket connections and echoing
│   │   └── models.rs          # (Optional) Data structures if using typed messages
│   ├── Cargo.toml
│   └── .env.example
├── docker/
│   ├── Dockerfile.frontend
│   ├── Dockerfile.backend
│   └── docker-compose.yml
├── .gitignore
├── README.md
└── LICENSE
```

---

### 🚀 10. Future Enhancements

- **Visualizations:** More advanced graphs (e.g., RTT distribution histogram, timeline of loss events).
- **Configurable Test Parameters:** Allow user to more finely tune test message size, frequency, total count/duration.
- **Export Results:** Allow users to download test session results (e.g., as JSON or CSV).
- **Historical Data (Client-Side):** Use browser's `localStorage` to store results of past tests for the user.
- **WebRTC DataChannel Test:** Explore using WebRTC DataChannels as an alternative to WebSockets for a potentially different type of P2P-like test (though still user-to-server in this context, WebRTC might handle NATs differently or use UDP). This is a more complex addition.
- **Server Health Indicator:** While the primary test is User->Server, the server could expose a simple health endpoint that the UI can check, separate from the echo test.

---

### ✅ 11. Dependencies (Examples, adjusted)

#### Frontend:

- **React.js, TypeScript, TailwindCSS, Vite**
- **State Management:** Zustand, Redux, or React Context
- **Charting:** Recharts, Chart.js, etc.
- **`isomorphic-ws` or browser's native `WebSocket` API**

#### Backend (Rust):

- **`tokio`:** Async runtime.
- **`tokio-tungstenite`** (or `actix-ws`, `warp::ws`, `axum::extract::ws`): For WebSocket server implementation.
- **`serde`, `serde_json`:** (Optional) If you choose to structure the echo messages as JSON. For simple byte array echo, it might not be strictly needed for the echo payload itself, but can be useful for control messages if any.
- **`tracing` or `log`:** For logging on the server.

#### DevOps (Optional):

- **Docker, Docker Compose**

---

### ⚙️ 12. Example WebSocket Message Exchange & Calculated Data

**A. Message sent from Client (React) to Server (Rust):**
Could be a simple string, or structured JSON. For efficiency, a compact format or raw bytes might be used, but JSON is easier to debug.

_Example using JSON:_

```json
{
  "seq": 101,
  "ts": 1678886400100 // Client's send timestamp (milliseconds)
}
```

**B. Message echoed from Server (Rust) back to Client (React):**
The server sends back the _exact same payload_ it received.

```json
{
  "seq": 101,
  "ts": 1678886400100
}
```

**C. Data calculated and maintained by the Frontend (React):**
The frontend doesn't receive a "summary" from the server. It _builds_ the summary and real-time updates based on the messages it sent and the echoes it received.

_Internal state in frontend during/after test might look like:_

```typescript
// Example internal state or variables in React component
const testResults = {
  messagesSent: 200,
  messagesReceived: 198,
  packetLossPercentage: 1.0, // ( (200-198)/200 ) * 100
  rttValues: [15, 18, 16, 25, 17 /* ... */], // Array of RTTs in ms for each received echo
  averageRttMs: 18.5,
  jitterMs: 3.2, // Calculated from rttValues
  // Potentially individual timestamps of sent/received messages for detailed analysis
};
```

The UI would then display values from this `testResults` object.

---

### 🧪 13. Testing & CI (Continuous Integration)

- **Unit Tests (Rust Backend):**
  - Test WebSocket connection handling.
  - Test the echo logic specifically (ensure messages are returned to the correct client unmodified and promptly).
- **Integration Tests (Rust Backend):**
  - Could involve a test client connecting to the Rust WebSocket server, sending messages, and verifying echoes.
- **Frontend Tests (React + TypeScript):**
  - **Unit Tests (Jest/Vitest + React Testing Library):**
    - Test WebSocket service module (mocking actual WebSocket).
    - Test metric calculation logic (loss, RTT, jitter).
    - Test individual UI components.
  - **E2E Tests (Playwright or Cypress):**
    - Crucial for this setup: Test the full flow of connecting to a (test/dev) backend, sending messages, receiving echoes, and verifying that the UI displays calculated metrics correctly.
- **CI Pipeline (e.g., GitHub Actions):**
  - Linting, testing (backend & frontend), building, optional Docker image creation.
