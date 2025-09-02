mod types;
mod Subscription_manager;
mod user;
use types::IncomingMessage;

fn main() {
    println!("WebSocket Server Starting...");
    
    // Example usage of the types
    let subscribe_msg = IncomingMessage::Subscribe(types::SubscribeMessage {
        method: "SUBSCRIBE".to_string(),
        params: vec!["btcusdt@ticker".to_string()],
        id: 1,
    });
    
    println!("Subscribe message: {:?}", subscribe_msg);
}