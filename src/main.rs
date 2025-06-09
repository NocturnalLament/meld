//Just to get rid of the warnings for now
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use dotenv::dotenv;

use model::ModelMessage;
mod model_response;
mod conversation;
mod roles;
mod requester;
mod model;
mod configurator;
mod file_logic;

async fn initialize_config(config_file_name: String) -> configurator::configurator::Config {
    let config_file_name = config_file_name;
    let mut config = configurator::configurator::default_config();
    if !configurator::configurator::file_exists(&config_file_name) {
        let config = configurator::configurator::default_config();
        config.save_config(&config_file_name).await;
        println!("Config file created");
    } else {
        config = configurator::configurator::load_config(config_file_name).await;
        println!("Config file loaded");
    }
    config.initialize_prompt();
    config
}

async fn handle_conversation(response: &Result<model_response::response::Response, reqwest::Error>, config: &configurator::configurator::Config, requester: &requester::Requester, conversation: &mut conversation::convo::Conversation) {
    match response {
        Ok(response) => {
            //println!("Response: {:?}", response);
            //println!("Response: {:?}", response.output[0].content[0].text);
            let content = response.output[0].content[0].text.clone();
            let id = response.output[0].id.clone();
            conversation.add_message(ModelMessage::new( Some(id.clone()), "assistant".to_string(), content.clone()));
            println!("Content: {:?}", content);
            println!("len: {:?}", conversation.messages.len());
            if conversation.messages_saveworthy() {
                // conversation.save_messages("conversation".to_string()).await;
                // println!("Messages saved");
                // //conversation.reset_messages();
                // let last_message = conversation.messages.last().unwrap().clone();
                // conversation.reset_messages();
                // conversation.add_message(ModelMessage { id: None, role: "system".to_string(), content: "You are a ditzy valley girl secretary that is obsessed with all things adorable and frequently gets distracted".to_string() });
                // conversation.add_message(last_message.clone());
                if conversation::convo::Conversation::file_exists("conversation".to_string()) {
                    //conversation.append_messages("conversation".to_string()).await;
                    conversation.append_messages("conversation".to_string()).await;
                    println!("Messages appended");
                    //conversation.reset_messages();
                    let last_message = conversation.messages.last().unwrap().clone();
                    conversation.reset_messages();
                    conversation.add_message(ModelMessage { id: None, role: "system".to_string(), content: "You are a ditzy valley girl secretary that is obsessed with all things adorable and frequently gets distracted".to_string() });
                    conversation.add_message(last_message);
                } else {
                    conversation.save_messages("conversation".to_string()).await;
                    println!("Messages saved");
                    let last_message = conversation.messages.last().unwrap().clone();
                    conversation.reset_messages();
                    conversation.add_message(ModelMessage { id: None, role: "system".to_string(), content: "You are a ditzy valley girl secretary that is obsessed with all things adorable and frequently gets distracted".to_string() });
                    conversation.add_message(last_message.clone());
                }
            }
            //println!("ID: {:?}", &id);
            //conversation.add_message(ModelMessage::new(,"assistant".to_string(), content.clone()));
            //let test_message = serde_json::from_str::<ModelMessage>(&content).expect("Failed to deserialize message");
            //conversation.add_message(test_message);
            //println!("Test message: {:?}", test_message.id);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}

#[tokio::main]
async fn main() {
    let config = initialize_config("config".to_string()).await;
    dotenv().ok();
    let mut running = true;
    let api_key = &config.env_key;
    let url = "https://api.openai.com/v1/responses";
    let requester = requester::requester_factory(api_key.clone(), url.to_string());
    let mut conversation = conversation::convo::Conversation::new(model::ChatModels::Gpt4oMini.model_string(), Vec::new());
    conversation.add_message(ModelMessage { id: None, role:"system".to_string(), content: config.model_prompt.clone() });
    while running {
        //conversation.reset_messages();
        if conversation.messages_saveworthy() {
            if conversation::convo::Conversation::file_exists("conversation".to_string()) {
                conversation.append_messages("conversation".to_string()).await;
            } else {
                conversation.save_messages("conversation".to_string()).await;
            }
            conversation.reset_messages();
        }
        println!("Enter a message: ");
        let mut message = String::new();
        
        std::io::stdin().read_line(&mut message).expect("Failed to read line");
        let message = message.trim();

        
        if message == "exit" {
            running = false;
            continue;
        } else if message == "save-conversation" {
            println!("Enter a file name: ");
            let mut file_name = String::new();
            std::io::stdin().read_line(&mut file_name).expect("Failed to read line");
            let file_name = file_name.trim();
            let file_base = "saved_conversations".to_string();
            let exists = file_logic::file_logic::check_for_conversation_file(&file_base);
            if !exists {
                let file_name = format!("{}-{}", file_name, "saved");
                conversation.save_messages(file_name.to_string()).await;
            }
            continue;
        } else if message == "load-conversation" {
            let conversation_list = file_logic::file_logic::get_saved_conversations().await;
            file_logic::file_logic::display_conversation_list(&conversation_list);
            println!("Enter a conversation name: ");
            let mut conversation_name = String::new();
            std::io::stdin().read_line(&mut conversation_name).expect("Failed to read line");
            let conversation_name = conversation_name.trim().to_string();
            conversation = file_logic::file_logic::load_conversation(&conversation_name).await;
            //conversation.display_conversation();
            continue;
        }
        
        println!("Asking...");
        conversation.add_message(ModelMessage::new( None, "user".to_string(), message.to_string()));
        let response = requester.send_request(&conversation).await;
        handle_conversation(&response, &config, &requester, &mut conversation).await;
    }
}
