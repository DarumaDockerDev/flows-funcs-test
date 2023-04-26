// use cloud_vision_flows::text_detection;
use flowsnet_platform_sdk::logger;
use lambda_flows::{request_received, send_response};

#[no_mangle]
pub fn run() {
    logger::init();
    request_received(|_qry, body| {
        log::debug!("{}", String::from_utf8(body.clone()).unwrap());
        // let text = text_detection(String::from_utf8(body).unwrap());
        send_response(
            200,
            vec![(
                String::from("content-type"),
                String::from("text/plain; charset=UTF-8"),
            )],
            // text.unwrap().as_bytes().to_vec(),
            body,
        );
    });
}
