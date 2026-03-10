use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Payload {
    pub choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
pub struct Choice {
    pub message: Message,
}

#[derive(Deserialize, Debug)]
pub struct Message {
    pub content: String,
}

#[derive(Deserialize, Debug)]
pub struct Models {
    pub data: Vec<Model>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Model {
    pub id: String,
}
