pub mod convo {
    use tokio::fs::File;
    use tokio::io::{AsyncWriteExt};
    use serde::{Deserialize, Serialize};
    use serde_json;
    use crate::model::ModelMessage;
    use std::path::Path;
    use tokio::fs::OpenOptions;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Conversation {
        pub model: String,
        pub messages: Vec<ModelMessage>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub temperature: Option<f32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub max_tokens: Option<u32>,

    }

    impl Conversation {
        // Sets the model type and a vector of model messages.
        pub fn new(model: String, messages: Vec<ModelMessage>) -> Self {
            Conversation { model,   messages, temperature: None, max_tokens: None }
        }

        pub fn to_json(&self) -> String {
            #[derive(Serialize)]
            struct ConversationJson {
                model: String,
                messages: Vec<ModelMessageJson>,
                temperature: Option<f32>,
                max_tokens: Option<u32>,
            }
            #[derive(Serialize)]
            struct ModelMessageJson {
                role: String,
                content: String,
            }
            let messages = self.messages.iter().map(|message| ModelMessageJson { role: message.role.clone(), content: message.content.clone() }).collect::<Vec<ModelMessageJson>>();
            let conversation = ConversationJson { model: self.model.clone(), messages, temperature: self.temperature, max_tokens: self.max_tokens };
            match serde_json::to_string(&conversation) {
                Ok(json) => json,
                Err(e) => {
                    println!("Error serializing conversation: {}", e);
                    "".to_string()
                }
            }
        }

        pub fn add_message(&mut self, message: ModelMessage) {
            self.messages.push(message);
        }

        pub async fn save_messages(&self, file_name: String) {
            let filtered_messages = self.messages.iter().filter(|message| message.role != "system").collect::<Vec<&ModelMessage>>();
            let yaml = serde_yaml::to_string(&filtered_messages).expect("Failed to serialize messages");
            let file_path = format!("{}.yaml", file_name);
            let mut file = File::create(file_path).await.expect("Failed to create file");
            file.write_all(yaml.as_bytes()).await.expect("Failed to write to file");
        }
       
        pub async fn append_messages(&mut self, file_name: String) {
            let filtered_messages = self.messages.iter().filter(|message| message.role != "system").collect::<Vec<&ModelMessage>>();
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(format!("{}.yaml", file_name))
                .await
                .expect("Failed to open file");
            file.write_all(serde_yaml::to_string(&filtered_messages).expect("Failed to serialize messages").as_bytes())
                .await
                .expect("Failed to write to file");
        }

        pub fn messages_saveworthy(&self, limit: &u32) -> bool {
            self.messages.len() >= 10
        }

        pub fn reset_messages(&mut self) {
            self.messages.clear();
            self.messages.push(ModelMessage { id: None, role: "system".to_string(), content: "You are a ditzy valley girl secretary that is obsessed with all things adorable and frequently gets distracted".to_string() });
        }

        // Get the proper file path.
        pub fn file_exists(file_name: String) -> bool {
            // determine if the file does not contain '.yaml'
            if !file_name.contains(".yaml") {
                // format the path to contain .yaml if not
                let file_path = format!("{}.yaml", file_name);
                // return the result of if the file path exists.
                return Path::new(&file_path).exists();
            } else {
                // return the result of if the file path exists.
                return Path::new(&file_name).exists();
            } 
        }
         
    }
    
}