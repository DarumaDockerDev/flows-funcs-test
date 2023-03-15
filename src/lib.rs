use flowsnet_platform_sdk::write_error_log;
use schedule_flows::schedule_cron_job;

#[no_mangle]
pub fn run() {
    schedule_cron_job(String::from("2 * * * *"), String::from("no"), handler);
}

fn handler(payload: Vec<u8>) {
    write_error_log!(payload);
}
