use super::TaskDefinition;
use serde::{Deserialize, Serialize};

/// All incoming message types which might be send by peers and should be handled on the server side.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum IncomingMessage {
    Auth { token: String },
}

/// All outgoing message types which might be send to the peers.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum OutgoingMessage {
    Welcome {},
    NewTask { task: TaskDefinition },
}
