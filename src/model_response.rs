pub mod response {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Response {
        pub id: String,
        pub object: String,
        #[serde(rename = "created_at")]
        pub created_at: i64,
        pub status: String,
        #[serde(rename = "error")]
        pub err: Option<String>,
        pub output: Vec<Output>,
        pub parallel_tool_cals: Option<bool>,
        pub previous_response_id: Option<String>,
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