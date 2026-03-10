use serde::Serialize;

#[derive(Serialize)]
pub struct Payload {
    pub model: String,
    pub messages: Vec<Message>,
}

#[derive(Serialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}
