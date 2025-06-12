pub mod response {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Response {
        pub id: String,
        pub object: String,
        #[serde(rename = "created_at")]
        pub created: Option<i64>,
        pub model: String,
        pub choices: Vec<ResponseChoices>,
        pub usage: Usage,
        pub service_tier: String,
        pub system_fingerprint: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Usage {
        pub prompt_tokens: i64,
        pub completion_tokens: i64,
        pub total_tokens: i64,
        pub prompt_tokens_details: PromptTokensDetails,
        pub completion_tokens_details: CompletionTokensDetails,
        }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PromptTokensDetails {
        pub cached_tokens: i64,
        pub audio_tokens: i64,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct CompletionTokensDetails {
        pub reasoning_tokens: i64,
        pub audio_tokens: i64,
        pub accepted_prediction_tokens: i64,
        pub rejected_prediction_tokens: i64,
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct ResponseChoices {
        pub index: i64,
        pub message: ResponseMessage,
        pub finish_reason: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ResponseMessage {
        pub role: String,
        pub content: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Output {
        #[serde(rename = "type")]
        pub output_type: String,
        pub id: String,
        pub status: String,
        pub role: String,
        pub content: Vec<Content>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Content {
        #[serde(rename = "type")]
        pub content_type: String,
        pub text: String,
        pub annotations: Vec<Annotation>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Annotation {
        pub start: i64,
        pub end: i64,
        #[serde(rename = "type")]
        pub annotation_type: String,
    }
    
    
}