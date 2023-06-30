use flowsnet_platform_sdk::write_error_log;
use schedule_flows::schedule_cron_job;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    schedule_cron_job(String::from("7 * * * *"), String::from("no"), handler).await;
}

async fn handler(payload: Vec<u8>) {
    write_error_log!(payload);
}
