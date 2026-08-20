//! WebSocket utilities for OCPP

use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, error, warn};

use super::errors::*;

/// WebSocket connection wrapper
pub struct WebSocketConnection {
    sender: Arc<Mutex<tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>>,
    receiver: Arc<Mutex<tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>>,
}

impl WebSocketConnection {
    /// Connect to a WebSocket endpoint
    pub async fn connect(url: &str) -> OcppResult<Self> {
        let request = url
            .into_client_request()
            .map_err(|e| OcppError::WebSocket(e.to_string()))?;

        let (stream, _) = connect_async(request)
            .await
            .map_err(|e| OcppError::WebSocket(e.to_string()))?;

        // Split stream for concurrent send/receive
        let (sender, receiver) = stream.split();
        let sender = Arc::new(Mutex::new(sender));
        let receiver = Arc::new(Mutex::new(receiver));

        Ok(Self { sender, receiver })
    }

    /// Send a message
    pub async fn send(&self, message: Message) -> OcppResult<()> {
        let mut sender = self.sender.lock().await;
        sender
            .send(message)
            .await
            .map_err(|e| OcppError::WebSocket(e.to_string()))?;
        Ok(())
    }

    /// Receive a message
    pub async fn receive(&self) -> OcppResult<Message> {
        let mut receiver = self.receiver.lock().await;
        let message = receiver
            .next()
            .await
            .ok_or_else(|| OcppError::WebSocket("Connection closed".to_string()))?
            .map_err(|e| OcppError::WebSocket(e.to_string()))?;
        Ok(message)
    }

    /// Close the connection
    pub async fn close(&self) -> OcppResult<()> {
        let mut sender = self.sender.lock().await;
        sender
            .close()
            .await
            .map_err(|e| OcppError::WebSocket(e.to_string()))?;
        Ok(())
    }

    /// Check if connection is closed
    pub async fn is_closed(&self) -> bool {
        // Try to send a ping to check connection
        let mut sender = self.sender.lock().await;
        sender
            .send(Message::Ping(vec![]))
            .await
            .is_err()
    }
}