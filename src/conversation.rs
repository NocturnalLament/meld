pub mod convo {
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
    } 
}