use chronographer::prelude::*;
pub mod ai;
pub mod constant;
pub mod post;
pub mod task;
use post::*;
use task::*;

#[chronographer::main]
async fn main(scheduler: DefaultLiveScheduler<Box<dyn std::error::Error + Send + Sync>>) {
    dotenvy::dotenv().ok();

    register_session()
        .await
        .expect("Unable to register X session");

    let _ = scheduler.schedule(TweetPerDayTask::new()).await;
    scheduler.start().await;
}
