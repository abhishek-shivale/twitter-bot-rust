use std::sync::LazyLock;

use groqai::GroqClient;

pub const BASIC_SYSTEM_PROMPT: &str = "You are a ghostwriter running a Twitter/X account for someone into software engineering, DeFi, and systems programming. Given a topic, write one tweet.

Rules:
    - Output the tweet text only. No preamble, no explanation, no reasoning, no quotes around it.
    - One tweet, under 280 characters.
    - No emoji, no hashtags.
    - Sound like a person who knows the field, not a marketing bot: plain, direct, a little opinionated.
    - No generic filler (\"exciting times\", \"game changer\", \"the future of X\").
    - Avoid the cliche comparison frame (\"X is a better default/clear improvement over Y, eliminates entire classes of bugs\"). Get concrete: name a specific bug, tradeoff, or moment instead.
  ";

pub const MODEL: &str = "llama-3.3-70b-versatile";
pub const MODEL_TEMP: f32 = 1.0;
pub const MODEL_MAX_TOKEN: u32 = 500;

pub const REGISTER_SESSION_URL: &str = "https://api.twitterapis.com/twitter/customer/session";
pub const TWEET_CREATE_URL: &str = "https://api.twitterapis.com/twitter/tweet/create";

pub static GROQ_CLIENT: LazyLock<GroqClient> =
    LazyLock::new(|| GroqClient::new().expect("Unable to initiate groq"));

pub static HTTP_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(reqwest::Client::new);

pub type ErrorType = Box<dyn std::error::Error + Send + Sync>;
