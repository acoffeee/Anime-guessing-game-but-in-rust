use super::rate_limiter::rate_limiter::Limiter;
use reqwest::Client;
use serde_json::json;
use std::collections::HashMap;
use crate::api_client::Queries;
const URL: &str = "https://graphql.anilist.co";
pub struct AnilistClient {
    limit: Limiter,
    client: Client
}
impl AnilistClient {
    pub fn new() -> AnilistClient{
        AnilistClient {
            limit: Limiter::new(),
            client: reqwest::Client::new()
                }
    }
    pub async fn request_user_info (&self, user_name: String)  {
        let query = json! (
            {
                "query": Queries::USERLISTGUESSINGQUERY,
                "variables": {
                    "userName": &user_name
                }
            }
        );
        let contents: String = self.client
        .post(URL)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(query.to_string())
        .send()
        .await
        .unwrap()
        .text()
        .await
        .expect("JSON");
        println!("Done");
        println!("this is the query {}", Queries::USERLISTGUESSINGQUERY);
        println!("cnts {}", contents);
    }
}
