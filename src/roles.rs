pub mod roles {
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