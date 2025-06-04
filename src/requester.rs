use reqwest::{Client, header::{HeaderMap, HeaderValue, HeaderName, AUTHORIZATION, CONTENT_TYPE}};
use crate::conversation::convo::Conversation;
use crate::model_response::response::Response;
use crate::model::ModelMessage;
use crate::model_response;
use crate::conversation;

pub struct Requester {
    pub client: Client,
    pub url: String,
    pub api_key: String,

}

impl Requester {
    pub fn new(client: Client, url: String, api_key: String, headers: HeaderMap) -> Self {
        Requester { client, url, api_key }
    }

    pub async fn send_request(&self, conversation: &conversation::convo::Conversation) -> Result<model_response::response::Response, reqwest::Error> {
        // let response = self.client.post(self.url.clone())
        //     .headers(self.headers.clone())
        //     .body(conversation.to_json())
        //     .send()
        //     .await?;
    //}
    //println!("Sending request to: {}", conversation.to_json());
    let response = self.client.post(self.url.clone())
    .header("Authorization", format!("Bearer {}", self.api_key))
    .header("Content-Type", "application/json")
    .body(conversation.to_json())
    .send()
    .await?;
    let body = response.text().await.expect("Failed to get response body");
    //println!("Body: {:?}", body);
    let response: Response = serde_json::from_str(&body).expect("stuff");
    Ok(response)
    }
}

pub fn requester_factory(api_key: String, url: String) -> Requester {
    let client = Client::new();
    let key = api_key.clone();
    let headers = generate_headers(api_key);
    Requester::new(client, url, key, headers)
}

pub fn generate_headers(api_key: String) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap());
    headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json").unwrap());
    headers
}