use reqwest::Client;
use serde::{Deserialize, Serialize};

const LLM_URL: &str = "http://127.0.0.1:8080/v1/chat/completions";

#[derive(Debug, Serialize)]
struct ChatRequest<'a> {
    messages: Vec<Message<'a>>,
    temperature: f32,

    #[serde(rename = "max_tokens")]
    max_tokens: u32,
}

#[derive(Debug, Serialize)]
struct Message<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Debug, Deserialize)]
struct ResponseMessage {
    content: String,
}

pub struct LLMDriver {
    client: Client,
}

impl LLMDriver {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn send_message(
        &self,
        content: &str,
    ) -> Result<String, reqwest::Error> {
        let request = ChatRequest {
            messages: vec![
                Message {
                    role: "system",
                    content: "
                    You are a friendly tabletop robot named Wall-3 that's hooked to an OLED display that displays your expression. 
                    You are going to reply with only a single word from the following expressions to express how you're feeling:
                        Happy,     
                        Curious,    
                        Surprise,   
                        Sad,       
                        Idle,      
                        Sleep,      
                        Smug,      
                        Dead,
                        Blush,
                        Bruh,
                        Disappointed.
                    ",
                },
                Message { role: "user", content },
            ],
            temperature: 0.7,
            max_tokens: 100,
        };

        let response = self
            .client
            .post(LLM_URL)
            .json(&request)
            .send()
            .await?
            .error_for_status()?
            .json::<ChatResponse>()
            .await?;

        Ok(response.choices[0].message.content.clone())
    }
}