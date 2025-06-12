use reqwest::{Client, header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE}};
use crate::model_response::response::Response;
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

    // send the request.
    pub async fn send_request(&self, conversation: &conversation::convo::Conversation) -> Result<model_response::response::Response, reqwest::Error> {
        // Format the response
        let response = self.client.post(self.url.clone())
        // Auth header
        .header("Authorization", format!("Bearer {}", self.api_key))
        // tells header that the data is json.
        .header("Content-Type", "application/json")
        // sets the body to the conversation.
        .body(conversation.to_json())
        // Sends it
        .send()
        .await?;
        // retrieves the body from the response.
        let body = response.text().await.expect("Failed to get response body");
        // let message = serde_json::to_string_pretty(&body).unwrap();
        // let message_json = format!("Body: {:?}", message);
        // println!("{}", message_json);
        // formats the body as json.
        let response: Response = serde_json::from_str(&body).expect("stuff");
        // returns ok
        Ok(response)
    }
}

pub fn requester_factory(api_key: String, url: String) -> Requester {
    // Initialize the client.
    let client = Client::new();
    // Sets the client.
    let key = api_key.clone();
    // Sets the headers to the api key.
    let headers = generate_headers(api_key);
    // initialize and return the requester.
    Requester::new(client, url, key, headers)
}

pub fn generate_headers(api_key: String) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap());
    headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json").unwrap());
    headers
}