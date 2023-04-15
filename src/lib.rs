use cloud_vision_flows::text_detection;
use lambda_flows::{request_received, send_response};

extern "C" {
    fn set_log(p: *const u8, len: i32);
}

#[no_mangle]
pub fn run() {
    request_received(|_qry, body| {
        let log = b"abc".to_vec();
        unsafe {
            set_log(log.as_ptr(), log.len() as i32);
        }

        let text = text_detection(String::from_utf8(body).unwrap());
        match text {
            Ok(r) => send_response(
                200,
                vec![(
                    String::from("content-type"),
                    String::from("text/plain; charset=UTF-8"),
                )],
                r.as_bytes().to_vec(),
            ),
            Err(e) => send_response(
                500,
                vec![(
                    String::from("content-type"),
                    String::from("text/plain; charset=UTF-8"),
                )],
                e.as_bytes().to_vec(),
            ),
        }
    });
}
