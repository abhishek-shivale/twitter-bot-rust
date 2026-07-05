use chronographer::prelude::*;

#[task(schedule = cron!(* * * * * *), singleton = false)]
async fn TweetPerDayTask(_ctx: &TaskFrameContext) -> Result<(), String> {
  println!("hi");
  Ok(())
}

#[chronographer::main]
async fn main(scheduler: DefaultLiveScheduler<String>) {
  let _ = scheduler.schedule(TweetPerDayTask::new()).await;
  scheduler.start().await;
}