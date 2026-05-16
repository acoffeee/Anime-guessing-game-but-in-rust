mod api_client;
use api_client::AnilistClient;
use std::io;
use std::io::Error;
use tokio;
use rand::{self, RngExt};
use serde_json::{ json, Value};
fn clean_string(s: &mut String) {
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
   clean_string(&mut username);
   let mut list: Value = json!({"username": "user"});
   if let Ok(x) = std::result::Result::<std::string::String, Error>::Ok(username) {
        let client: AnilistClient::AnilistClient = AnilistClient::AnilistClient::new();
        list = client.request_user_info(&x).await;
   } else {
    println!("damn");
   }
   println!("game starting... \n");
   let rng = rand::rng();
   let entries = &list["data"]["MediaListCollection"]["lists"]["entries"];
   let entry_count = entries.as_array().unwrap().len();
   /*loop {
    let anime = &client.get_anime_info(entries[rng.random_range(0..=entry_count)]);
    println!("{}", anime);
   }*/
   Ok(())
}
