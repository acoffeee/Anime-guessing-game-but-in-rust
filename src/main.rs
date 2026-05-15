mod api_client;
use api_client::AnilistClient;
use std::io;
use std::io::Error;
use tokio;
async fn runner(username: String, client: &AnilistClient::AnilistClient) {
    client.request_user_info(username).await;
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
   println!("hi");
   let mut username = String::new();
   println!("Whats your AL username?");
   io::stdin().read_line(&mut username);
   if let Ok(x) = std::result::Result::<std::string::String, Error>::Ok(username) {
        let client: AnilistClient::AnilistClient = AnilistClient::AnilistClient::new();
        runner(x, &client).await;
   } else {
    println!("damn");
   }
   Ok(())
}
