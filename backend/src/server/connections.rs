use crate::model::{WsCommand, WsRequest};
use futures::stream::SplitSink;
use futures::SinkExt;
use log::{error, info};
use std::collections::HashMap;
use tokio::sync::mpsc;
use warp::ws::{Message, WebSocket};

pub struct Connections {
    connections: HashMap<String, SplitSink<WebSocket, Message>>,
    msg_receiver: mpsc::Receiver<WsCommand>,
}

impl Connections {
    pub fn new() -> (Self, mpsc::Sender<WsCommand>) {
        let (msg_sender, msg_receiver): (mpsc::Sender<WsCommand>, mpsc::Receiver<WsCommand>) =
            mpsc::channel(128);

        let connections = Connections {
            connections: HashMap::new(),
            msg_receiver,
        };

        (connections, msg_sender)
    }

    pub async fn start_listening(&mut self) {
        info!("Started listening for connections");

        self.process_message_requests().await;
    }

    async fn process_message_requests(&mut self) {
        while let Some((peer_id, request)) = self.msg_receiver.recv().await {
            info!("Received message request");

            match request {
                WsRequest::SendMessage { msg } => {
                    if self
                        .connections
                        .get_mut(&peer_id)
                        .unwrap()
                        .send(Message::text(serde_json::to_string(&msg).unwrap()))
                        .await
                        .is_err()
                    {
                        error!("Sending message to {} has failed", &peer_id);
                    }
                }
                WsRequest::AddConnection { sender } => {
                    self.connections.insert(peer_id, sender);
                }
            }
        }
    }
}
