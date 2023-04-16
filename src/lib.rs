use lambda_flows::{request_received, send_response};

#[no_mangle]
pub fn run() {
    request_received(|_qry, body| {
        send_response(
            200,
            vec![(
                String::from("content-type"),
                String::from("text/plain; charset=UTF-8"),
            )],
            body,
        );
    });
}
