use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use futures_util::stream::StreamExt;
use futures_util::sink::SinkExt;
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");
    println!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream.peer_addr().expect("connected streams should have a peer address");
        println!("Peer address: {}", peer);
        tokio::spawn(accept_connection(peer, stream));
    }
}

async fn accept_connection(peer: SocketAddr, stream: TcpStream) {
    let mut ws_stream = accept_async(stream).await.expect("Failed to accept websocket connection");
    println!("New WebSocket connection: {}", peer);

    while let Some(msg) = ws_stream.next().await {
        let msg = msg.expect("Failed to get message");
        if msg.is_text() || msg.is_binary() {
            println!("Received a message from {}: {}", peer, msg.to_text().unwrap());
            ws_stream.send(Message::text("Hello from server!")).await.unwrap();
        }
    }
}
