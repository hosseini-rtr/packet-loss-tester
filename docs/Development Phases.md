# 📆 Development Phases (Updated for User-to-Server Test Model)

| Phase | Title                                             | Duration   | Key Deliverables                                                                 |
| :---- | :------------------------------------------------ | :--------- | :------------------------------------------------------------------------------- |
| 1     | UI Scaffold & Rust WebSocket Echo Server Setup    | 1 Week     | Basic project structures, initial UI, functional Rust Echo Server.                 |
| 2     | Frontend Test Logic Implementation (MVP)          | 1.5 Weeks  | Frontend sends test messages, receives echoes, calculates basic loss & RTT.        |
| 3     | Live Frontend Display & Full Metrics              | 1.5 Weeks  | Real-time charts for loss, RTT, Jitter; polished UI for test execution.          |
| 4     | Client-Side Result Storage & History (Optional)   | 1 Week     | Store test results in browser, allow viewing past sessions.                      |
| 5     | Dockerization & Deployment Prep                   | 1 Week     | Containerize frontend & backend, `docker-compose` setup.                         |
| 6     | Documentation & Open Source Release               | 0.5 Week   | Final `README.md`, `LICENSE`, `CONTRIBUTING.md`, public GitHub repo.               |
| **Total** |                                               | **~6.5 Weeks** | *Focused part-time estimate. Full-time could be ~3-4 weeks.* |

---

### **Phase 1: UI Scaffold & Rust WebSocket Echo Server Setup**

**Duration**: 1 week
**Goal**: Set up basic project structures for frontend and backend, and establish initial WebSocket communication with a simple echo from the Rust server.

#### Tasks:

* Scaffold React app (Vite + TypeScript).
* Set up TailwindCSS and a basic component structure.
* Scaffold Rust backend project (e.g., using `cargo new`).
* Implement a basic Rust WebSocket server (e.g., using `tokio-tungstenite` or `actix-web`/`warp` with WebSocket support) that echoes any message it receives.
* Define WebSocket message structure (e.g., simple JSON for test messages like `{seq: number, ts: number}`).
* Basic UI with a "Start Test" button, placeholder for results, and ability to connect to the Rust WebSocket server.
* Frontend sends a test message, Rust server echoes it, frontend logs received echo.

✅ *Outcome*: Frontend can connect to the Rust WebSocket server, send a message, and receive an echo. Basic project structures are in place.

---

### **Phase 2: Frontend Test Logic Implementation (MVP)**

**Duration**: 1.5 weeks
**Goal**: Implement the core client-side logic for sending a sequence of test messages, receiving echoes, and calculating basic packet loss and RTT.

#### Tasks:

* In React frontend:
    * Implement logic to send a configurable number of test messages at a set interval (e.g., 10 messages/sec).
    * Each message should contain a unique sequence number and a client-side timestamp.
    * Store sent message timestamps and sequence numbers.
    * On receiving echoed messages, match them by sequence number.
    * Calculate RTT for each received echo.
    * Calculate overall packet loss percentage (messages sent vs. echoes received).
    * Calculate average RTT.
    * Display basic results (loss %, avg RTT) in the UI upon test completion.
* Handle test duration (e.g., stop sending after X seconds or Y messages).

✅ *Outcome*: Frontend can execute a full test sequence against the Rust echo server and display calculated packet loss and average RTT.

---

### **Phase 3: Live Frontend Display & Full Metrics**

**Duration**: 1.5 weeks
**Goal**: Visualize test results in real-time on the frontend, calculate jitter, and polish the user interface for the test execution phase.

#### Tasks:

* Implement live-updating charts in React (e.g., using Recharts or similar) for:
    * RTT per message/over time.
    * Packet loss percentage as it updates.
* Calculate and display Jitter (e.g., standard deviation of RTTs or inter-packet delay variation).
* Display current sent/received packet counts live.
* Improve UI for test parameters (duration, frequency, etc.).
* Style loading states, error states (e.g., WebSocket disconnection), and no-data states.
* Implement Start/Stop test functionality robustly.

✅ *Outcome*: Smooth, real-time UX during test runs with comprehensive metrics (loss, RTT, jitter) displayed dynamically.

---

### **Phase 4: Client-Side Result Storage & History (Optional)**

**Duration**: 1 week
**Goal**: Add functionality to store test results locally in the user's browser and allow them to view a history of past test sessions.

#### Tasks:

* Design a data structure for storing test session summaries (parameters, overall metrics, timestamp of test).
* Use browser `localStorage` or `IndexedDB` to persist test session data.
* Implement a UI section to display a list of past test sessions.
* Allow users to click on a past session to view its detailed results.
* Provide functionality to clear history.

✅ *Outcome*: Users can review their past test session results performed in their current browser.

---

### **Phase 5: Dockerization & Deployment Prep**

**Duration**: 1 week
**Goal**: Containerize the frontend and Rust backend applications and set up `docker-compose` for easy local deployment and development.

#### Tasks:

* Write `Dockerfile` for the React frontend (multi-stage build: build static assets, then serve with a lightweight server like Nginx or a simple Node server).
* Write `Dockerfile` for the Rust WebSocket echo server (compile Rust binary in a builder stage, then copy to a minimal runtime image).
* Create `docker-compose.yml` to orchestrate both frontend and backend services.
* Configure environment variables (e.g., WebSocket server URL for frontend, server port for Rust backend).
* Ensure the entire stack can be run locally using `docker-compose up`.
* Test different build modes (dev vs. prod if applicable).

✅ *Outcome*: The entire application is containerized and runnable with a single `docker-compose up` command.

---

### **Phase 6: Documentation & Open Source Release**

**Duration**: 0.5 week
**Goal**: Finalize project documentation, add necessary files for open-sourcing, and publish the repository.

#### Tasks:

* Write/Update `README.md` with:
    * Project overview and purpose (User <-> Server test).
    * Updated architecture diagram.
    * Instructions for local setup (manual and Docker).
    * How to use the tool.
* Add a `LICENSE` file (e.g., MIT or Apache 2.0).
* Create `CONTRIBUTING.md` guidelines and issue templates if expecting contributions.
* Perform final code cleanup, ensure all linters/formatters pass.
* Ensure CI pipeline (if any) is green.
* Publish the repository on GitHub (or your chosen platform).

✅ *Outcome*: A polished, public, self-hostable open-source repository for the packet loss testing tool.

