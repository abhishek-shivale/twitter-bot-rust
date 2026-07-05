use crate::prompt::{BASIC_SYSTEM_PROMPT, MODEL, MODEL_MAX_TOKEN, MODEL_TEMP};
use chronographer::prelude::*;
use groqai::{ChatMessage, GroqClient, MessageContent, Role};
use rand::Rng;
use std::sync::LazyLock;

static GROQ_CLIENT: LazyLock<GroqClient> =
    LazyLock::new(|| GroqClient::new().expect("Unable to initiate groq"));

    
static HTTP_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(reqwest::Client::new);

type ErrorType = Box<dyn std::error::Error + Send + Sync>;

#[task(schedule = cron!(* * * * * *), singleton = false)]
pub async fn TweetPerDayTask(_ctx: &TaskFrameContext) -> Result<(), ErrorType> {
    let client = &*GROQ_CLIENT;
    let topic = fetch_trending_topic().await?;

    let messages = vec![
        ChatMessage::new_text(Role::System, BASIC_SYSTEM_PROMPT),
        ChatMessage::new_text(Role::User, format!("Topic: {topic}")),
    ];

    let tweet = format!("{:?}", call_ai(client, messages).await.unwrap());

    Ok(())
}


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
        .get(format!("https://hacker-news.firebaseio.com/v0/item/{id}.json"))
        .send()
        .await?
        .json()
        .await?;

    Ok(item["title"].as_str().unwrap_or("systems programming").to_string())
}


pub async fn call_ai(client: &GroqClient, messages: Vec<ChatMessage>) -> Result<MessageContent,  ErrorType> {
   let response = client
        .chat(MODEL)
        .messages(messages)
        .temperature(MODEL_TEMP)
        .max_completion_tokens(MODEL_MAX_TOKEN)
        .send()
        .await?;

    Ok(response.choices[0].message.content.clone())
}