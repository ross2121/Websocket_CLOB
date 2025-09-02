use std::{net::TcpStream, sync::Arc};

use futures_util::{lock::Mutex, stream::{SplitSink, SplitStream}};
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

pub struct User{
    userid:String,
    sender:Arc<Mutex<SplitSink<WebSocketStream<TcpStream>,Message>>>,
    stream:Option<SplitStream<WebSocketStream<TcpStream>>>,
    subscription:Arc<Mutex<Vec<String>>>
}
impl User {
    pub fn new(userid:String,sender:Arc<Mutex<SplitSink<WebSocketStream<TcpStream>,Message>>>,stream:SplitStream<WebSocketStream<TcpStream>>)->Self{
          let User=User{
            userid,
            sender,
            stream:Some(stream),
            subscription:Arc::new(Mutex::new(Vec::new()))
          };
          User
    }
    pub async fn subscribe(&mut self,subscription_data:String){
        let mut subscription=self.subscription.lock().await;
        subscription.push(subscription_data)
    }
    pub async fn unscubribe(&mut self,subscription_data:String){
        let mut unsubscribe=self.subscription.lock().await;
        unsubscribe.retain(|sub|sub.to_string()!=subscription_data);
    }
}
