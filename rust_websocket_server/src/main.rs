use futures_util::{SinkExt, StreamExt};
use std::error::Error;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};
#[warn(unused_imports)]
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let peer_addr = stream.peer_addr()?;
        println!("New connection from: {}", peer_addr);
        tokio::spawn(accept_connection(stream));
    }
    Ok(())
}

async fn accept_connection(stream: TcpStream) {
    match accept_async(stream).await {
        Ok(ws_stream) => {
            println!("WebSocket connection established.");
            let (mut writer, mut reader) = ws_stream.split();

            while let Some(msg_result) = reader.next().await {
                match msg_result {
                    Ok(Message::Text(text)) => {
                        println!("Received text: {}", text);

                        let response = format!("Server received: {}", text);
                        if writer.send(Message::Text(response)).await.is_err() {
                            eprintln!("Error sending message back to client.");
                            break;
                        }
                    }
                    Ok(Message::Binary(bin)) => {
                        println!("Received binary: {:?}", bin);
                        if writer.send(Message::Binary(bin)).await.is_err() {
                            eprintln!("Error sending binary message back.");
                            break;
                        }
                    }
                    Ok(Message::Close(_)) => {
                        println!("Client sent close message. Closing connection.");
                        break;
                    }
                    Ok(Message::Ping(data)) => {
                        // WebSockets have Ping/Pong to keep connections alive or check latency.
                        // We should respond to a Ping with a Pong.
                        // TODO: Handle Ping for latency check.
                        println!("Received Ping, sending Pong back.");
                        if writer.send(Message::Pong(data)).await.is_err() {
                            eprintln!("Error sending Pong.");
                            break;
                        }
                    }
                    Ok(Message::Pong(_)) => {
                        // We received a Pong, usually in response to our Ping.
                        // For an echo server, we don't need to do much.
                        // TODO: Handle Pong for latency check.
                        println!("Received Pong.");
                    }
                    Ok(Message::Frame(_)) => {
                        println!("Received raw WebSocket frame.");
                    }
                    Err(e) => {
                        eprintln!("Error reading message from client: {}", e);
                        break;
                    }
                }
            }
            println!("WebSocket connection closed for one client.");
        }
        Err(e) => {
            eprintln!("Error during WebSocket handshake: {}", e);
        }
    }
}
