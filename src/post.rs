use crate::constant::*;

pub async fn register_session() -> Result<(), ErrorType> {
    let http = &*HTTP_CLIENT;
    let api_key = std::env::var("TWITTERAPIS_KEY")?;
    let auth_token = std::env::var("X_AUTH_TOKEN")?;
    let ct0 = std::env::var("X_CT0")?;

    let res = http
        .post(REGISTER_SESSION_URL)
        .bearer_auth(&api_key)
        .json(&serde_json::json!({ "auth_token": auth_token, "ct0": ct0 }))
        .send()
        .await?;

    let status = res.status();
    let body = res.text().await?;

    if !status.is_success() {
        return Err(format!("session registration failed: {status} {body}").into());
    }
    
    Ok(())
}

pub async fn post_to_x(text: &str) -> Result<(), ErrorType> {
    let http = &*HTTP_CLIENT;
    let api_key = std::env::var("TWITTERAPIS_KEY")?;

    let res = http
        .post(TWEET_CREATE_URL)
        .bearer_auth(&api_key)
        .json(&serde_json::json!({ "text": text }))
        .send()
        .await?;

    let status = res.status();
    let body: serde_json::Value = res.json().await?;

    if !status.is_success() || body["ok"].as_bool() != Some(true) {
        return Err(format!("tweet post failed: {status} {body}").into());
    }
    Ok(())
}
