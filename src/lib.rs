use std::{collections::HashMap, thread, time::Duration};

use http_req::request;
use lambda_flows::{request_received, send_response};
use serde_json::Value;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    request_received(handler).await;
}

async fn handler(_qry: HashMap<String, Value>, _body: Vec<u8>) {
    // thread::sleep(Duration::from_secs(10));

    let mut writer = Vec::new();
    let query_str = format!("https://hub.dummyapis.com/delay?seconds=3");

    let resp = request::get(query_str, &mut writer)
        .map_err(|e| e.to_string())
        .and_then(|_| String::from_utf8(writer).map_err(|_| "Unexpected error".to_string()));

    match resp {
        Ok(r) => send_response(
            200,
            vec![(
                String::from("content-type"),
                String::from("text/html; charset=UTF-8"),
            )],
            r.as_bytes().to_vec(),
        ),
        Err(e) => send_response(
            400,
            vec![(
                String::from("content-type"),
                String::from("text/html; charset=UTF-8"),
            )],
            e.as_bytes().to_vec(),
        ),
    }
}
