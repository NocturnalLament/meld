pub mod roles {
    use serde::{Serialize, Deserialize};
    use serde;
    // pub const USER: &str = "user";
    // pub const ASSISTANT: &str = "assistant";
    // pub const SYSTEM: &str = "system";
    pub enum Role {
        User,
        Assistant,
        System,
    }
    impl Role {
        pub fn as_str(&self) -> &str {
            match self {
                Role::User => "user",
                Role::Assistant => "assistant",
                Role::System => "system",
            }
        }
    }
}