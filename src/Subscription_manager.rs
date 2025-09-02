use std::{collections::HashMap, sync::Arc};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use redis::Client;
use serde::{Serialize,Deserialize};
#[derive(Serialize)]
pub struct Subscrption_Manager{
    #[serde(skip_serializing,skip_deserializing)]
       pub client:Client,
  pub subscription:HashMap<String,Vec<String>>,
  pub reversesubscription:HashMap<String,Vec<String>>
}

pub static  INSTANCE:Lazy<Arc<Mutex<Subscrption_Manager>>>=Lazy::new(||{Arc::new(Mutex::new(Subscrption_Manager::new()))});
impl Subscrption_Manager{
    pub fn instance()->  Arc<Mutex<Subscrption_Manager>>{
        println!("check3");
        Arc::clone(&INSTANCE)
      }
      pub fn new()->Self{
          let subscription=HashMap::new();
          let reversesubscription=HashMap::new();
          let redis_url = "redis://localhost:6379";
          let client: Client=Client::open(redis_url).unwrap();
          Subscrption_Manager { client: client, subscription: subscription, reversesubscription: reversesubscription }
      }
      pub fn unsubscribe(&mut self,userid:String,channel:String){ 
         if let Some(subscriptions)=self.subscription.get_mut(&userid){
            subscriptions.retain(|sub|sub!=&channel);
         }
         if let Some(users)=self.reversesubscription.get_mut(&channel){
             users.retain(|user|userid!=user.to_string());     
         }
         if(self.reversesubscription.get(&channel).unwrap().len()==0){
              self.reversesubscription.remove(&channel);    
              match self.client.get_connection() {
                Ok(mut client) => {
                    if let Err(e) = client.as_pubsub().unsubscribe(&channel) {
                        eprintln!("Error: Failed to unsubscribe from channel '{}': {}", channel, e);
                    }
                }
                Err(e) => {
                    println!("Error: Failed to get a client connection: {}", e);
                }
            }
         }
      }
      pub fn subscribe(&mut self,userid:String,subscriptionmsg:String){
         let subscribemessage=self.subscription.get(&userid).unwrap();
          if(!subscribemessage.contains(&subscriptionmsg)){
              let user=self.subscription.get(&userid).unwrap();
              self.subscription.entry(userid.clone()).or_insert(Vec::new()).push(subscriptionmsg.to_string());
              self.reversesubscription.entry(subscriptionmsg.clone()).or_insert(Vec::new()).push(userid.to_string());
              if(self.reversesubscription.get(&subscriptionmsg).unwrap().len()==1){
                          
              }

          }else{

          }
    }
    pub fn userleft(&mut self,userid:String){
       let channel=self.subscription.get(&userid).map(|channel|channel.clone()).unwrap();
       for unscubribe in channel{
        self.unsubscribe(userid.clone(), unscubribe);
       }
       self.subscription.remove(&userid);
    }




}
