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
        #[serde(rename = "input")]
        pub messages: Vec<ModelMessage>,
    }

    impl Conversation {
        pub fn new(model: String, messages: Vec<ModelMessage>) -> Self {
            Conversation { model,   messages }
        }

        pub fn to_json(&self) -> String {
            match serde_json::to_string(self) {
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
        /*  pub async fn load_messages(file_name: String) -> Vec<ModelMessage> {
            let file_path = format!("{}.yaml", file_name);
            let mut file = File::open(file_path).await.expect("Failed to open file");
            let mut yaml = String::new();
            file.read_to_string(&mut yaml).await.expect("Failed to read file");
            let messages: Vec<ModelMessage> = serde_yaml::from_str(&yaml).expect("Failed to deserialize messages");
            messages
        } */
        pub async fn append_messages(&mut self, file_name: String) {
            
            // let yaml = serde_yaml::to_string(&existing).expect("Failed to serialize messages");
            // let file_path = format!("{}.yaml", file_name);
            // let mut file = File::open(file_path).await.expect("Failed to create file");
            // file.write_all(yaml.as_bytes()).await.expect("Failed to write to file");
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

        pub fn messages_saveworthy(&self) -> bool {
            self.messages.len() >= 10
        }

        pub fn reset_messages(&mut self) {
            self.messages.clear();
            self.messages.push(ModelMessage { id: None, role: "system".to_string(), content: "You are a ditzy valley girl secretary that is obsessed with all things adorable and frequently gets distracted".to_string() });
        }

        pub fn file_exists(file_name: String) -> bool {
            let file_path = format!("{}.yaml", file_name);
            Path::new(&file_path).exists()
        }
         
    }
    
}