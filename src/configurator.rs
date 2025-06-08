pub mod configurator {
    use serde::{Deserialize, Serialize};
    use serde_yaml;
    use tokio::fs::File;
    use tokio::io::{AsyncWriteExt, AsyncReadExt};
    use std::path::Path;
    use tokio::fs::OpenOptions;
    use std::env;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Config {
        pub model: String,
        pub model_name: String,
        pub model_prompt: String,
        pub message_retention_limit: u32,
        pub message_retention_maximum: u32,
        pub conversation_file_name: String,
        pub conversation_file_path: String,
        pub config_file_name: String,
        pub config_file_path: String,
        pub env_key: String,
    }

    impl Config {
        pub fn new(model: String, model_name: String, model_prompt: String, message_retention_limit: u32, message_retention_maximum: u32, conversation_file_name: String, conversation_file_path: String, config_file_name: String, config_file_path: String, env_key: String) -> Self {
            Config { model, model_name, model_prompt, message_retention_limit, message_retention_maximum, conversation_file_name, conversation_file_path, config_file_name, config_file_path, env_key }
        }

        pub fn to_yaml(&self) -> String {
            serde_yaml::to_string(self).expect("Failed to serialize config")
        }

        pub async fn save_config(&self, file_name: &String) {
            if !Path::new(&file_name).exists() {
                let file_path = format!("{}.yaml", file_name);
                let mut file = File::create(file_path).await.expect("Failed to create file");
                file.write_all(self.to_yaml().as_bytes()).await.expect("Failed to write to file");
            } else {
                let mut file = OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(format!("{}.yaml", file_name))
                    .await
                    .expect("Failed to open file");
                file.write_all(self.to_yaml().as_bytes()).await.expect("Failed to write to file");
            }
        }


        
    }

    pub fn default_config() -> Config {
        let mut config_file_path_base_env = env::current_dir().expect("Failed to get current directory");
        
        
        let config_file_name = "config.yaml".to_string();
        config_file_path_base_env.push(&config_file_name);
        
        Config {
            model: "gpt-4o-mini".to_string(),
            model_name: "gpt-4o-mini".to_string(),
            model_prompt: "You are a ditzy valley girl secretary that is obsessed with all things adorable and frequently gets distracted".to_string(),
            message_retention_limit: 10,
            message_retention_maximum: 20,
            conversation_file_name: "conversation".to_string(),
            conversation_file_path: config_file_path_base_env.display().to_string(),
            config_file_name: config_file_name,
            config_file_path: config_file_path_base_env.display().to_string(),
            env_key: "OPEN_API".to_string(),
        }
    }

    pub fn file_exists(file_name: &String) -> bool {
        let name_with_extension = format!("{}.yaml", file_name);
        let mut file_path_base_env = env::current_dir().expect("Failed to get current directory");
        file_path_base_env.push(&name_with_extension);
        Path::new(&file_path_base_env).exists()
    }

    pub async fn load_config(file_name: String) -> Config {
        let file_path = format!("{}.yaml", file_name);
        let mut file = File::open(file_path).await.expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).await.expect("Failed to read file");
        serde_yaml::from_str(&contents).expect("Failed to deserialize config")
    }
}