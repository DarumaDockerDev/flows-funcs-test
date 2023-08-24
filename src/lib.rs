use flowsnet_platform_sdk::logger;
use schedule_flows::schedule_cron_job;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    logger::init();
    schedule_cron_job(
        std::env::var("SCHEDULE_CRONTAB").unwrap(),
        String::from("no"),
        handler,
    )
    .await;
}

async fn handler(_payload: Vec<u8>) {
    log::info!("schedule triggered");
}
