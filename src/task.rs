use crate::prompt::{BASIC_SYSTEM_PROMPT, MODEL, MODEL_MAX_TOKEN, MODEL_TEMP};
use chronographer::prelude::*;
use groqai::{ChatMessage, GroqClient, MessageContent, Role, Tool, ToolChoice};
use std::sync::LazyLock;

static GROQ_CLIENT: LazyLock<GroqClient> =
    LazyLock::new(|| GroqClient::new().expect("Unable to initiate groq"));

type ErrorType = Box<dyn std::error::Error + Send + Sync>;

#[task(schedule = cron!(* * * * * *), singleton = false)]
pub async fn TweetPerDayTask(_ctx: &TaskFrameContext) -> Result<(), ErrorType> {
    let client = &*GROQ_CLIENT;

    let messages = vec![
        ChatMessage::new_text(Role::System, BASIC_SYSTEM_PROMPT),
    ];
 

    Ok(())
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