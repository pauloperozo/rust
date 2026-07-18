use serde::{Serialize, Deserialize};
 
#[derive(Serialize)]
pub struct ChatResponse {
    pub message: String 
}

#[derive(Deserialize)]
pub struct ChatRequest {
    pub enquiry: String
}