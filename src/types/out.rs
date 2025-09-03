use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Clone)]
pub struct Tickerdata{
    pub close_price: Option<String>,      
    pub high_price: Option<String>,     
    pub low_price: Option<String>,        
    pub volume: Option<String>,           
    pub quote_volume: Option<String>,     
    pub symbol: Option<String>,           
    pub id: u64,               
    pub event_type: String,
}
#[derive(Serialize,Deserialize,Clone)]
pub struct  Depthupdatetype{
pub bids:Option<Vec<(String,String)>>,
pub ask:Option<Vec<(String,String)>>,
pub id:u64,
pub event_type:String
}
#[derive(Serialize,Deserialize,Clone)]
pub struct Tradedata{
    pub is_buyer_maker: bool,     
    pub price: String,            
    pub quantity: String,         
    pub symbol: String,  
}
#[derive(Serialize,Deserialize,Clone)]
#[serde(tag="type")]
pub enum Outgoingmessage {
    #[serde(rename="ticker")]
    Ticker{
        data:Tickerdata
    },
    #[serde(rename="depth")]
    Depth{
        data:Depthupdatetype
    },
    #[serde(rename="trade")]
    Trade{
        data:Tradedata
    }
}
