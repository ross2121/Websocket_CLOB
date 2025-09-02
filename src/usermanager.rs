use std::{collections::HashMap, sync::Arc};

use futures_util::lock::Mutex;
use futures_util::StreamExt;
use tokio::net::TcpStream;
use once_cell::sync::Lazy;
use tokio_tungstenite::tungstenite::WebSocket;
use tokio_tungstenite::WebSocketStream;
use crate::user::User;
use rand::Rng;


pub struct Usermanager{
    pub user:HashMap<String,Arc<Mutex<User>>>
}

pub static INSTANCE:Lazy<Arc<Mutex<Usermanager>>>=Lazy::new(||{Arc::new(Mutex::new(Usermanager::new()))});
impl Usermanager{
    pub fn new()->Self{

        Usermanager { user: HashMap::new() }
    }
    pub fn instance()->Arc<Mutex<Usermanager>>{
        Arc::clone(&INSTANCE)
    }
  pub fn add_user(&mut self,ws:WebSocketStream<TcpStream>){
        let (sink,stream)=ws.split();
          let id=self.randomId();
          let user_sender=Arc::new(Mutex::new(sink));
          let user=User::new(id.clone(), user_sender, stream);
          self.user.entry(id).insert_entry(Arc::new(Mutex::new(user)));
  }
  pub fn get_user(&mut self,userid:String)->Option<&Arc<Mutex<User>>>{
    self.user.get(&userid)
  }
  pub fn randomId(&self)->String{
    let mut rng = rand::thread_rng();
    format!("{:x}{:x}", rng.gen::<u64>(), rng.gen::<u64>())
  }
}