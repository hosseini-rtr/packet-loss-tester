use std::error::Error;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};
use tokio::prelude::*; // For tokio 0.1 specific Future, Stream, Sink

fn main() -> Result<(), Box<dyn Error>> {
    let addr_str = "127.0.0.1:8080";
    let addr = addr_str.parse::<SocketAddr>()?;
    let listener = TcpListener::bind(&addr)?;
    println!("Listening on: {}", addr);

    let server = listener.incoming()
        .map_err(|e| eprintln!("Failed to accept connection: {:?}", e))
        .for_each(move |stream| {
            let peer_addr = stream.peer_addr().expect("Failed to get peer address");
            println!("New connection from: {}", peer_addr);
            tokio::spawn(accept_connection(stream).map_err(|e| {
                // tokio::spawn requires Error = ()
                eprintln!("Connection error: {:?}", e);
            }));
            Ok(())
        });

    tokio::run(server);
    Ok(())
}

// Define a type alias for the Sink part of the WebSocket stream for clarity.
type WsSink = tokio::prelude::stream::SplitSink<tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>>;

fn accept_connection(stream: tokio::net::TcpStream) -> impl Future<Item = (), Error = tokio_tungstenite::tungstenite::Error> {
    accept_async(stream)
        .map_err(|e| {
            eprintln!("Error during WebSocket handshake: {}", e);
            e // Propagate the error
        })
        .and_then(|ws_stream| {
            println!("WebSocket connection established.");
            let (writer, reader) = ws_stream.split();

            reader
                .map_err(move |e| { // Ensure writer is moved if used in error mapping
                    eprintln!("Error reading message: {}", e);
                    e // Ensure this error type matches what for_each expects
                })
                .fold(writer, |mut writer_sink, msg| { // Use fold to thread the writer sink
                    println!("Received message: {:?}", msg);
                    let response_fut: Box<dyn Future<Item = WsSink, Error = tokio_tungstenite::tungstenite::Error> + Send> = match msg {
                        Message::Text(text) => {
                            println!("Echoing text: {}", text);
                            let response = Message::Text(text);
                            Box::new(writer_sink.send(response).map_err(|e| {
                                eprintln!("Error sending text response: {}", e);
                                e
                            }))
                        }
                        Message::Binary(bin) => {
                            println!("Echoing binary: {:?}", bin);
                            let response = Message::Binary(bin);
                            Box::new(writer_sink.send(response).map_err(|e| {
                                eprintln!("Error sending binary response: {}", e);
                                e
                            }))
                        }
                        Message::Ping(data) => {
                            println!("Received Ping, sending Pong back.");
                            let response = Message::Pong(data);
                            Box::new(writer_sink.send(response).map_err(|e| {
                                eprintln!("Error sending Pong: {}", e);
                                e
                            }))
                        }
                        Message::Pong(_) => {
                            println!("Received Pong.");
                            // Just return the writer, no send needed
                            Box::new(future::ok(writer_sink))
                        }
                        // No Message::Close variant in this version of tungstenite
                        // The stream will simply end.
                    };
                    response_fut
                })
                .then(|result| { // result is Result<WsSink, Error>
                    match result {
                        Ok(_) => println!("Message loop completed, writer retained."),
                        Err(e) => eprintln!("Error in message loop (fold): {}", e),
                    }
                    println!("WebSocket connection closed for one client.");
                    future::ok(()) // Future must resolve to Item = () for accept_connection
                })
        })
}
