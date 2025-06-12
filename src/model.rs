use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    pub model: String,
    pub instructions: String,
    pub input: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModelMessage {
    // Id of the message (optional)
    pub id: Option<String>,
    // Assistants role.
    pub role: String,
    // Content of the message.
    pub content: String,
}

pub enum ChatModels {
    Gpt4o,
    Gpt4oMini,
    Gpt41,
    Gpt3o,
    Gpt3oMini,
    Gpto1,
}

impl ChatModels {
    pub fn model_string(&self) -> String {
        match self {
            ChatModels::Gpt4o => "gpt-4o-2024-08-06".to_string(),
            ChatModels::Gpt4oMini => "gpt-4o-mini-2024-07-18".to_string(),
            ChatModels::Gpt41 => "gpt-4.1-2025-04-14".to_string(),
            ChatModels::Gpt3o => "o3-2025-04-16".to_string(),
            ChatModels::Gpt3oMini => "o3-mini-2025-01-31".to_string(),
            ChatModels::Gpto1 => "o1-2025-06-04".to_string(),
        }
    }
}

impl Model {
    pub fn new(model: String, instructions: String, input: String) -> Self {
        Model {
            model,
            instructions,
            input,
        }
    }

    pub fn to_json(&self) -> String {
        match serde_json::to_string(self) {
            Ok(json) => json,
            Err(e) => {
                println!("Error serializing model: {}", e);
                "".to_string()
            }
        }
    }
}

impl ModelMessage {
    // Create new model message with optional id, role, and content.
    pub fn new(id: Option<String>, role: String, content: String) -> Self {
        ModelMessage { id, role, content }
    }

    // Formats it to json.
    pub fn to_json(&self) -> String {
        match serde_json::to_string(self) {
            Ok(json) => json,
            Err(e) => {
                println!("Error serializing model message: {}", e);
                "".to_string()
            }
        }
    }
}
