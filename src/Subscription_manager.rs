use std::{collections::HashMap, sync::{ Arc}};
use futures_util::StreamExt;
use tokio::sync::{OnceCell,Mutex};
use redis::{Client, aio::PubSub};
use serde::{Serialize,Deserialize};

use crate::{types::out::Outgoingmessage, user::User, usermanager::Usermanager};
#[derive(Serialize)]
pub struct Subscrption_Manager{
    #[serde(skip_serializing,skip_deserializing)]
       pub client:PubSub,
  pub subscription:HashMap<String,Vec<String>>,
  pub reversesubscription:HashMap<String,Vec<String>>
}

pub static  INSTANCE: OnceCell<Arc<Mutex<Subscrption_Manager>>> = OnceCell::const_new();
impl Subscrption_Manager{
    pub async  fn instance()->  Arc<Mutex<Subscrption_Manager>>{
        INSTANCE.get_or_init(|| async{
            let manager=Subscrption_Manager::new().await;
            let manager_arc=Arc::new(Mutex::new(manager));
            tokio::spawn(Self::redis_listner(manager_arc.clone()));
            manager_arc
        }).await.clone()
      }
      pub async  fn new()->Self{
          let subscription=HashMap::new();
          let reversesubscription=HashMap::new();
          let redis_url = "redis://localhost:6379";
          let client: Client=Client::open(redis_url).unwrap();
          let pubsub=client
          .get_async_connection()
          .await
          .expect("Failed to get Redis connection");
        let pubsub = pubsub.into_pubsub();
          Subscrption_Manager { client: pubsub, subscription: subscription, reversesubscription: reversesubscription }
      }
      pub async fn unsubscribe(&mut self,userid:String,channel:String){ 
         if let Some(subscriptions)=self.subscription.get_mut(&userid){
            subscriptions.retain(|sub|sub!=&channel);
         }
         if let Some(users)=self.reversesubscription.get_mut(&channel){
             users.retain(|user|userid!=user.to_string());     
             if self.reversesubscription.get(&channel).unwrap().len() == 0 {
                self.reversesubscription.remove(&channel);
                // Use async PubSub methods
                if let Err(e) = self.client.unsubscribe(&channel).await {
                    eprintln!("Error: Failed to unsubscribe from channel '{}': {}", channel, e);
                }
            }
      }
    }
      pub async  fn subscribe(&mut self,userid:String,subscriptionmsg:String){
         let subscribemessage=self.subscription.get(&userid).unwrap();
          if(!subscribemessage.contains(&subscriptionmsg)){
              let user=self.subscription.get(&userid).unwrap();
              self.subscription.entry(userid.clone()).or_insert(Vec::new()).push(subscriptionmsg.to_string());
              self.reversesubscription.entry(subscriptionmsg.clone()).or_insert(Vec::new()).push(userid.to_string());
              if self.reversesubscription.get(&subscriptionmsg).unwrap().len() == 1 {
                
                if let Err(e) = self.client.subscribe(&subscriptionmsg).await {
                    eprintln!("Failed to subscribe to channel: {}", e);
                }
              }
        
    }}
    pub fn userleft(&mut self,userid:String){
       let channel=self.subscription.get(&userid).map(|channel|channel.clone()).unwrap();
       for unscubribe in channel{
        self.unsubscribe(userid.clone(), unscubribe);
       }
       self.subscription.remove(&userid);
    }
 pub   async   fn rediscallbackhandler(&mut self,message:String,channel:String) {
        match  serde_json::from_str::<Outgoingmessage>(&message) {
              Ok(parsedmessage)=>{
                if let Some(subs)=self.reversesubscription.get(&channel){
                    for sub in subs{
                        let user_manager=Usermanager::instance();
                        let mut manger_guard=user_manager.lock().await;
                        if let Some(user_arc)=manger_guard.get_user(sub.to_string()){
                             let mut user_grd=user_arc.lock().await;
                             if let Err(e) = user_grd.emit(parsedmessage.clone()).await {
                                eprintln!("Failed to emit message to user : {}", e);
                            }

                             
                        }
                    }
                }
              }
              Err(e) => {
                eprintln!(
                    "Error parsing OutgoingMessage from Redis on channel {}: {}. Message",
                    channel, e
                );
            }
        }


         

    }
    async  fn redis_listner(instance_Arc:Arc<Mutex<Self>>){
        loop {
            let msg_result={
                let mut grd=instance_Arc.lock().await;
            let mut message=grd.client.on_message();
            message.next().await
            };
            match msg_result {
                Some(redis_msg) => {
                    let channel_str: &str = redis_msg.get_channel_name();
                    let channel: String = channel_str.to_string();
                    let payload_result: redis::RedisResult<String> = redis_msg.get_payload();
                    let payload: String = match payload_result {
                        Ok(p) => p,
                        Err(e) => {
                            eprintln!("Error getting payload from Redis message on channel '{}': {}. Skipping.", channel, e);
                            continue;
                        }
                    };

                    let mut guard = instance_Arc.lock().await;
                      guard.rediscallbackhandler(payload, channel);
                }
                None => {
                    eprintln!("Redis PubSub stream ended. Attempting to re-establish stream in the next iteration.");
                }
            }
        }
    }




}
