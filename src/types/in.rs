use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SubscribeMessage {
    pub method: String,
    pub params: Vec<String>,
    pub id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnsubscribeMessage {
    pub method: String,
    pub params: Vec<String>,
    pub id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "method")]
pub enum IncomingMessage {
    #[serde(rename = "SUBSCRIBE")]
    Subscribe(SubscribeMessage),
    #[serde(rename = "UNSUBSCRIBE")]
    Unsubscribe(UnsubscribeMessage),
}