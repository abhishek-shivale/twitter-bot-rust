use crate::{
    ai::{call_ai, fetch_trending_topic},
    constant::*,
    post::post_to_x,
};
use chronographer::prelude::*;
use groqai::{ChatMessage, MessageContent, Role};

#[task(schedule = cron!(* * * * * *), singleton = false)]
pub async fn TweetPerDayTask(_ctx: &TaskFrameContext) -> Result<(), ErrorType> {
    let client = &GROQ_CLIENT;
    let topic = fetch_trending_topic().await?;

    let messages = vec![
        ChatMessage::new_text(Role::System, BASIC_SYSTEM_PROMPT),
        ChatMessage::new_text(Role::User, format!("Topic: {topic}")),
    ];

    let tweet = match call_ai(client, messages).await? {
        MessageContent::Text(text) => text,
        other => return Err(format!("unexpected response content: {other:?}").into()),
    };
    post_to_x(&tweet).await?;

    Ok(())
}
