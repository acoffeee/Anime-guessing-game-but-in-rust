use super::rate_limiter::rate_limiter::Limiter;
use json::JsonValue;
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
    pub async fn request_user_info (&self, user_name: &str) -> serde_json::Value  {
        let query = json! (
            {
                "query": Queries::USERLISTINFOQUERY,
                "variables": {
                    "userName": &user_name
                }
            }
        );
        println!("Username: {}", &query);
        let results: String = self.client
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
        let content = serde_json::from_str(&results.as_str()).unwrap();
        content
    }
}
