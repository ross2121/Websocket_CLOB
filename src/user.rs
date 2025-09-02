use std::{ sync::Arc};
use tokio::net::TcpStream;
use futures_util::{SinkExt, StreamExt};
use futures_util::{lock::Mutex, stream::{SplitSink, SplitStream}};
use tokio_tungstenite::tungstenite::client;
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use crate::types::out::Outgoingmessage;
use crate::types::IncomingMessage;
use crate::usermanager::Usermanager;
use crate::Subscription_manager::Subscrption_Manager;

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
    pub async fn emit(&mut self,message:Outgoingmessage)->Result<(),Box<dyn std::error::Error>>{
      let  parsedmessage:String=serde_json::to_string(&message).unwrap();
      let mut send=self.sender.lock().await;
      send.send(Message::Text(parsedmessage)).await;
      Ok(())
    }
    pub fn addlistner(&mut self){
       let id=self.userid.clone();
       let mut stream=self.stream.take().unwrap();
    
       tokio::spawn(async move{
        while  let Some(message)=stream.next().await {
          match  message {
              Ok(Message::Text(text))=>{
                if let Ok(parsedmessage)=serde_json::from_str::<IncomingMessage>(&text){
                    let mgr = Subscrption_Manager::instance();      // keep Arc alive
              let mut client = mgr.lock().unwrap();    
                    match parsedmessage{
                          IncomingMessage::Subscribe(msg)=>{
                            for s in msg.params{
                                let res=client.subscribe(id.clone(), s.to_string());
                            }
                          }
                          IncomingMessage::Unsubscribe(msg)=>{
                            for s in msg.params{
                                let res=client.subscribe(id.clone(), s.to_string());
                            }
                          }
                    }
                }
              }
              Ok(_) => {} 
              Err(e) => {
                eprintln!("Error processing message for user {}: {}", id, e);
                break;
            }
          }
        }
         let user_manager=Usermanager::instance();
         let mut guard=user_manager.lock().await;
         guard.user.remove(&id);
         let sub_manager=Subscrption_Manager::instance();
         let mut guard_manager=sub_manager.lock().unwrap();
          guard_manager.userleft(id.clone());
       });
    }
}
