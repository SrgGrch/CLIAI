use crate::model::openrouter::{request, response};

pub struct Api {
    api_key: String,
    model: String,
    endpoint: String,
}

// MARK: Public methods
impl Api {
    pub fn new(api_key: &str, model: &str, endpoint: &str) -> Self {
        if api_key.is_empty() {
            eprintln!("No api_key found. Set it with: cliai --api-key <key>");
            std::process::exit(1);
        }

        Api {
            api_key: api_key.to_string(),
            model: model.to_string(),
            endpoint: endpoint.to_string(),
        }
    }

    pub async fn completions(&self, promt: &str) -> response::Payload {
        let client = reqwest::Client::new();
        let shell = std::env::var("SHELL").unwrap_or("unknown".to_string());
        let os = std::env::consts::OS;

        client
        .post(self.create_url("chat/completions"))
        .json(&request::Payload {
            model: self.model.clone(),
            messages: vec![
                request::Message {
                    role: "system".to_string(),
                    content:  format!("You are AI assistant that provides CLI comand from user promt. \
                        You always answer in promts language. You always provide ONLY requested \
                        command. First line of your response always contains indicator of command \
                        safety. It can be on of two values: \"SAFE\" or \"DANGER\". Dangerous  \
                        commands are those that can be destructife (e.g. rm -rf, chmod -R 777, \
                        chown etc.) OS: {os}, env: {shell}")
                },
                request::Message {
                    role: "user".to_string(),
                    content: promt.to_string()
                }
            ]
        })
        .header(
            "Authorization",
            format!("Bearer {}", self.api_key)
        )
        .header("Content-Type", "application/json")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
    }

    pub async fn models(&self) -> response::Models {
        let client = reqwest::Client::new();

        client
            .get(self.create_url("models"))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }
}

// MARK: Private methods
impl Api {
    fn create_url(&self, path: &str) -> String {
        let endpoint = self.endpoint.clone();
        let endpoint = if endpoint.ends_with('/') {
            endpoint
        } else {
            endpoint + "/"
        };

        format!("{endpoint}{path}")
    }
}
