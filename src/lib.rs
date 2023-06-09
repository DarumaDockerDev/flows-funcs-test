use std::collections::HashMap;

use lambda_flows::{request_received, send_response};
use serde_json::Value;
use web_scraper_flows::get_page_text;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    request_received(handler).await;
}

async fn handler(qry: HashMap<String, Value>, _body: Vec<u8>) {
    let url = qry.get("url").expect("No url provided").as_str().unwrap();

    match get_page_text(url).await {
        Ok(text) => send_response(
            200,
            vec![(
                String::from("content-type"),
                String::from("text/plain; charset=UTF-8"),
            )],
            text.as_bytes().to_vec(),
        ),
        Err(e) => send_response(
            400,
            vec![(
                String::from("content-type"),
                String::from("text/plain; charset=UTF-8"),
            )],
            e.as_bytes().to_vec(),
        ),
    }
}
