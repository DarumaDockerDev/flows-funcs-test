use flowsnet_platform_sdk::logger;
use google_cloud_service_flows::cloud_vision::text_detection;
use lambda_flows::{request_received, send_response};
use serde_json::Value;
use std::collections::HashMap;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    logger::init();
    request_received(handler).await;
}

async fn handler(_qry: HashMap<String, Value>, body: Vec<u8>) {
    let r = text_detection::text_detection(String::from_utf8_lossy(&body).into_owned())
        .await
        .unwrap();

    send_response(
        200,
        vec![(
            String::from("content-type"),
            String::from("text/plain; charset=UTF-8"),
        )],
        r.as_bytes().to_vec(),
    );
}
