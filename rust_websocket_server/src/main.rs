use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{error::Error, sync::Arc};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};
use tracing::{error, info, warn};

// Define the test message structure
#[derive(Debug, Serialize, Deserialize)]
struct TestMessage {
    seq: u64, // Sequence number for tracking messages
    ts: u64,  // Client's send timestamp in milliseconds
}

// Structure to track connection statistics
#[derive(Debug, Default)]
struct ConnectionStats {
    messages_received: u64,
    last_sequence: u64,
    missed_sequences: Vec<u64>,
    total_missed: u64,
}

impl ConnectionStats {
    fn new() -> Self {
        Self::default()
    }

    fn update(&mut self, seq: u64) {
        self.messages_received += 1;

        // Check for missed sequences
        if self.messages_received > 1 {
            let expected_seq = self.last_sequence + 1;
            if seq > expected_seq {
                // We missed some sequences
                for missed_seq in expected_seq..seq {
                    self.missed_sequences.push(missed_seq);
                    self.total_missed += 1;
                }
            }
        }

        self.last_sequence = seq;
    }

    fn get_packet_loss_percentage(&self) -> f64 {
        if self.messages_received + self.total_missed == 0 {
            return 0.0;
        }
        (self.total_missed as f64 / (self.messages_received + self.total_missed) as f64) * 100.0
    }

    fn get_stats_message(&self) -> String {
        format!(
            "Packet Loss Stats:\n\
             Total Messages Received: {}\n\
             Total Messages Missed: {}\n\
             Packet Loss Percentage: {:.2}%\n\
             Last Sequence Number: {}",
            self.messages_received,
            self.total_missed,
            self.get_packet_loss_percentage(),
            self.last_sequence
        )
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Bind to the specified address
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await?;
    info!("WebSocket server listening on: {}", addr);

    // Create a shared state for all connections
    let connections = Arc::new(Mutex::new(HashMap::new()));
    let connection_counter = Arc::new(Mutex::new(0u64));

    // Accept incoming connections
    while let Ok((stream, addr)) = listener.accept().await {
        let conn_id = {
            let mut counter = connection_counter.lock().await;
            *counter += 1;
            *counter
        };

        info!("New WebSocket connection from: {} (ID: {})", addr, conn_id);

        // Clone the connections handle for this connection
        let connections = Arc::clone(&connections);

        // Insert new connection stats
        {
            let mut conns = connections.lock().await;
            conns.insert(conn_id, ConnectionStats::new());
        }

        tokio::spawn(handle_connection(stream, conn_id, connections));
    }

    Ok(())
}

async fn handle_connection(
    stream: TcpStream,
    conn_id: u64,
    connections: Arc<Mutex<HashMap<u64, ConnectionStats>>>,
) {
    match accept_async(stream).await {
        Ok(ws_stream) => {
            info!("WebSocket connection established for ID: {}", conn_id);
            let (mut write, mut read) = ws_stream.split();

            // Process incoming messages
            while let Some(message_result) = read.next().await {
                match message_result {
                    Ok(message) => {
                        match message {
                            Message::Text(text) => {
                                // Try to parse the message as a TestMessage
                                match serde_json::from_str::<TestMessage>(&text) {
                                    Ok(test_msg) => {
                                        // Update statistics
                                        {
                                            let mut conns = connections.lock().await;
                                            if let Some(stats) = conns.get_mut(&conn_id) {
                                                stats.update(test_msg.seq);

                                                // Log current stats every 100 messages
                                                if stats.messages_received % 100 == 0 {
                                                    info!(
                                                        "Connection {}: {}",
                                                        conn_id,
                                                        stats.get_stats_message()
                                                    );
                                                }
                                            }
                                        }

                                        // Echo back the original message
                                        if let Err(e) = write.send(Message::Text(text)).await {
                                            error!("Failed to echo message: {}", e);
                                            break;
                                        }
                                    }
                                    Err(e) => {
                                        warn!("Failed to parse message as TestMessage: {}", e);
                                        // Echo back invalid messages too
                                        if let Err(e) = write.send(Message::Text(text)).await {
                                            error!("Failed to echo invalid message: {}", e);
                                            break;
                                        }
                                    }
                                }
                            }
                            Message::Binary(bin) => {
                                // Echo back binary messages immediately
                                if let Err(e) = write.send(Message::Binary(bin)).await {
                                    error!("Failed to echo binary message: {}", e);
                                    break;
                                }
                            }
                            Message::Ping(data) => {
                                if let Err(e) = write.send(Message::Pong(data)).await {
                                    error!("Failed to send Pong: {}", e);
                                    break;
                                }
                            }
                            Message::Pong(_) => {
                                info!("Received Pong from connection {}", conn_id);
                            }
                            Message::Close(frame) => {
                                info!(
                                    "Received close frame from connection {}: {:?}",
                                    conn_id, frame
                                );

                                // Log final statistics
                                {
                                    let conns = connections.lock().await;
                                    if let Some(stats) = conns.get(&conn_id) {
                                        info!(
                                            "Final stats for connection {}: {}",
                                            conn_id,
                                            stats.get_stats_message()
                                        );
                                    }
                                }

                                if let Err(e) = write.send(Message::Close(frame)).await {
                                    error!("Error sending close frame: {}", e);
                                }
                                break;
                            }
                            Message::Frame(_) => {
                                warn!("Received raw frame from connection {}, ignoring", conn_id);
                            }
                        }
                    }
                    Err(e) => {
                        error!(
                            "Error processing message from connection {}: {}",
                            conn_id, e
                        );
                        break;
                    }
                }
            }

            // Clean up connection
            {
                let mut conns = connections.lock().await;
                if let Some(stats) = conns.remove(&conn_id) {
                    info!(
                        "Final stats for connection {}: {}",
                        conn_id,
                        stats.get_stats_message()
                    );
                }
            }
            info!("WebSocket connection {} closed", conn_id);
        }
        Err(e) => {
            error!(
                "Error during WebSocket handshake for connection {}: {}",
                conn_id, e
            );
        }
    }
}
