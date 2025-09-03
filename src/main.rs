mod types;
mod Subscription_manager;
mod user;
mod usermanager;

use futures_util::stream;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;


use types::IncomingMessage;

use crate::usermanager::Usermanager;
#[tokio::main]
async  fn main() {
   let listner=TcpListener::bind("127.0.0.9001").await.unwrap();
   while let Ok((Stream,_)) =  listner.accept().await{
    tokio::spawn(handle_connection(Stream));
   }
    
 
}
async fn handle_connection(stream:TcpStream){
    match accept_async(stream).await {
        Ok(ws_Stream)=>{
            let user_manage=Usermanager::instance();
            let mut manager_grd=user_manage.lock();
            manager_grd.await.add_user(ws_Stream);
        }
        Err(e)=>{
            println!("{}",e);
        }
    }
}