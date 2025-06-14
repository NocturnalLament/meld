pub mod file_logic {
    use std::path::Path;
    use tokio::fs;
    use std::env;
    use serde_yaml;
    use crate::conversation::convo;
    use crate::model::ModelMessage;
    use crate::model::ChatModels;

    pub fn check_for_conversation_file(file_name: &String) -> bool {
        let mut file_path_base_env = env::current_dir().expect("Failed to get current directory");
        file_path_base_env.push(file_name);
        let file_path = file_path_base_env.to_str().expect("Failed to convert path to string");
        let file_exists = Path::new(file_path).exists();
        file_exists
    }

    pub async fn get_conversation_files(save_file_name: &String) -> Vec<String> {
        let mut files: Vec<String> = Vec::new();
        let file_path_base_env = env::current_dir().expect("Failed to get current directory");
        let mut all_files = fs::read_dir(file_path_base_env).await.expect("Failed to read directory");
        while let Some(file) = all_files.next_entry().await.expect("Failed to get file") {
            let path = file.path();
            let file_name = path.file_name().expect("Failed to get file name").to_str().expect("Failed to convert file name to string");
            let file_name_without_extension = file_name.split(".").next().expect("Failed to get file name without extension");
            files.push(file_name_without_extension.to_string());
        }
        files
    }

    pub async fn get_highest_conversation_number(path_vec: &Vec<String>) -> u32 {
        let mut highest_number = 0;
        for file in path_vec {
            let file_name = file.split("-").next().expect("Failed to get file name");
            let file_number = file_name.parse::<u32>().expect("Failed to parse file number");
            if file_number > highest_number {
                highest_number = file_number;
            }
        }
        highest_number
    }

    pub async fn get_saved_conversations() -> Vec<String> {
        let mut files: Vec<String> = Vec::new();
        let file_path_base_env = env::current_dir().expect("Failed to get current directory");
        let mut all_files = fs::read_dir(file_path_base_env).await.expect("Failed to read directory");
        while let Some(file) = all_files.next_entry().await.expect("Failed to get file") {
            let path = file.path();
            let file_name = path.file_name().expect("Failed to get file name").to_str().expect("Failed to convert file name to string");
            let file_name_without_extension = file_name.split(".").next().expect("Failed to get file name without extension");
            files.push(file_name_without_extension.to_string());
        }
        files
    }

    pub async fn load_conversation(file_name: &String) -> convo::Conversation {
        let mut file_path_base_env = env::current_dir().expect("Failed to get current directory");
        file_path_base_env.push(file_name);
        let file_path = file_path_base_env.to_str().expect("Failed to convert path to string");
        let file = fs::read_to_string(file_path).await.expect("Failed to read file");
         // Parse the messages array first
        let messages: Vec<ModelMessage> = serde_yaml::from_str(&file).expect("Failed to parse messages");
        
        // Create a new conversation with the loaded messages
        let conversation = convo::Conversation::new(
            ChatModels::Gpt4oMini.model_string(),
            messages
        );
        conversation
    }

    pub fn display_conversation_list(conversation_list: &Vec<String>) {
        for conversation in conversation_list {
            println!("{}", conversation);
        }
    }

}
