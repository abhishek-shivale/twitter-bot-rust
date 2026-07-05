use chronographer::prelude::*;
pub mod task;
pub mod prompt;
use task::*;

#[chronographer::main]
async fn main(scheduler: DefaultLiveScheduler<Box<dyn std::error::Error + Send + Sync>>) {
    let _ = scheduler.schedule(TweetPerDayTask::new()).await;
    scheduler.start().await;
}
