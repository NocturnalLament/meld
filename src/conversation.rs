pub mod convo {
    use tokio::fs::File;
    use tokio::io::AsyncWriteExt;
    use serde::{Deserialize, Serialize};
    use serde_json;
    use crate::model::ModelMessage;

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
            let yaml = serde_yaml::to_string(&self.messages).expect("Failed to serialize messages");
            let file_path = format!("{}.yaml", file_name);
            let mut file = File::create(file_path).await.expect("Failed to create file");
            file.write_all(yaml.as_bytes()).await.expect("Failed to write to file");
        }

        pub fn messages_saveworthy(&self) -> bool {
            self.messages.len() >= 10
        }

        pub fn reset_messages(&mut self) {
            self.messages.clear();
        }
    } 
}