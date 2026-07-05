use crate::constant::*;
use groqai::{ChatMessage, GroqClient, MessageContent};
use rand::Rng;

pub async fn fetch_trending_topic() -> Result<String, ErrorType> {
    let http = &*HTTP_CLIENT;

    let ids: Vec<u64> = http
        .get("https://hacker-news.firebaseio.com/v0/topstories.json")
        .send()
        .await?
        .json()
        .await?;

    let id = ids[rand::thread_rng().gen_range(0..ids.len())];

    let item: serde_json::Value = http
        .get(format!(
            "https://hacker-news.firebaseio.com/v0/item/{id}.json"
        ))
        .send()
        .await?
        .json()
        .await?;

    Ok(item["title"]
        .as_str()
        .unwrap_or("systems programming")
        .to_string())
}

pub async fn call_ai(
    client: &GroqClient,
    messages: Vec<ChatMessage>,
) -> Result<MessageContent, ErrorType> {
    let response = client
        .chat(MODEL)
        .messages(messages)
        .temperature(MODEL_TEMP)
        .max_completion_tokens(MODEL_MAX_TOKEN)
        .send()
        .await?;

    Ok(response.choices[0].message.content.clone())
}
