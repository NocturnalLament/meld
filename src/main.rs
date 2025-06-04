use dotenv::dotenv;
use std::env;
use serde_json;
use model_response::response::Response;
use roles::roles::Role;
use model::ModelMessage;
mod model_response;
mod conversation;
mod roles;
mod requester;
mod model;
#[tokio::main]
async fn main() {
    dotenv().ok();
    let mut running = true;
    let api_key = env::var("OPEN_API").expect("API_KEY must be set");
    let url = "https://api.openai.com/v1/responses";
    let requester = requester::requester_factory(api_key.clone(), url.to_string());
    let mut conversation = conversation::convo::Conversation::new(model::ChatModels::Gpt4oMini.model_string(), Vec::new());
    conversation.add_message(ModelMessage { role:"system".to_string(), content: "You are a ditzy valley girl secretary that is obsessed with all things adorable and frequently gets distracted".to_string() });
    while running {
        println!("Enter a message: ");
        let mut message = String::new();
        
        std::io::stdin().read_line(&mut message).expect("Failed to read line");
        let message = message.trim();

        
        if message == "exit" {
            running = false;
            continue;
        }
        
        println!("Asking...");
        conversation.add_message(ModelMessage::new("user".to_string(), message.to_string()));
        let response = requester.send_request(&conversation).await;
        match response {
            Ok(response) => {
                //println!("Response: {:?}", response);
                //println!("Response: {:?}", response.output[0].content[0].text);
                let content = response.output[0].content[0].text.clone();
                println!("Content: {:?}", content);
                conversation.add_message(ModelMessage::new("assistant".to_string(), content.clone()));
                
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }   
    }
}
