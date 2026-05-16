mod api_client;
use api_client::AnilistClient;
use std::io;
use std::io::Error;
use tokio;
use rand;
use serde_json::{ json, Value};
async fn runner(username: &str, client: &AnilistClient::AnilistClient) -> serde_json::Value {
    client.request_user_info(&username).await
}

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
   println!("hi");
   let mut username = String::new();
   println!("Whats your AL username?");
   io::stdin().read_line(&mut username)?;
   trim_newline(&mut username);
   let mut list: Value = json!({"username": "user"});
   if let Ok(x) = std::result::Result::<std::string::String, Error>::Ok(username) {
        let client: AnilistClient::AnilistClient = AnilistClient::AnilistClient::new();
        list = runner(&x as &str, &client).await;
   } else {
    println!("damn");
   }
   println!("game starting... \n");
   let mut rng = rand::rng();
    println!("{}", list);
   Ok(())
}
