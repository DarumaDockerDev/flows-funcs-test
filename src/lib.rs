use std::collections::HashMap;

use serde_json::Value;
use webhook_flows::{
    create_endpoint, request_handler,
    route::{post, route, RouteError, Router},
    send_response,
};

use tencentcloud_sdk_sms::{
    client::{Client as TcClient, SendSmsRequest},
    credentials::Credential,
};
use twilio_wasi::{Client, OutboundMessage};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn on_deploy() {
    create_endpoint().await;
}

#[request_handler]
async fn handler() {
    let mut router = Router::new();
    router.insert("/twilio", vec![post(twilio)]).unwrap();
    router.insert("/tencent", vec![post(tencent)]).unwrap();
    if let Err(e) = route(router).await {
        match e {
            RouteError::NotFound => {
                send_response(404, vec![], b"No route matched".to_vec());
            }
            RouteError::MethodNotAllowed => {
                send_response(405, vec![], b"Method not allowed".to_vec());
            }
        }
    }
}

async fn tencent(_headers: Vec<(String, String)>, qry: HashMap<String, Value>, _body: Vec<u8>) {
    flowsnet_platform_sdk::logger::init();

    let text = qry.get("text").unwrap_or(&Value::Null).as_str();
    let to = qry.get("to").unwrap_or(&Value::Null).as_str();
    let response = match text.and(to) {
        Some(_) => {
            let secret_id = std::env::var("TENCENTCLOUD_SECRET_ID").unwrap();
            let secret_key = std::env::var("TENCENTCLOUD_SECRET_KEY").unwrap();
            let sms_sdk_app_id = std::env::var("TENCENTCLOUD_SMS_APP_ID").unwrap();
            let template_id = std::env::var("TENCENTCLOUD_SMS_TEMPLATE_ID").unwrap();
            let sign_name = "";
            let region = std::env::var("TENCENTCLOUD_REGION").unwrap();
            let phone_number_set = vec![to.unwrap().to_owned()];
            let template_param_set = vec![text.unwrap().to_owned()];
            // build client
            let credential = Credential::new(secret_id, secret_key, None);
            let client = TcClient::new(credential, region);

            // build request
            let request = SendSmsRequest::new(
                phone_number_set.clone(),
                sms_sdk_app_id,
                template_id,
                sign_name,
                template_param_set,
            );
            // send
            client
                .send_sms(request)
                .await
                .map_err(|x| format!("{:?}", x))
        }
        None => Err(String::from("No 'text' or 'to' in query")),
    };

    // check
    match response {
        Ok(res) => match res.check_is_success(to.unwrap().to_owned()) {
            true => {
                send_response(
                    200,
                    vec![(
                        String::from("content-type"),
                        String::from("text/html; charset=UTF-8"),
                    )],
                    b"success".to_vec(),
                );
            }
            false => {
                send_response(
                    400,
                    vec![(
                        String::from("content-type"),
                        String::from("text/html; charset=UTF-8"),
                    )],
                    b"failed".to_vec(),
                );
            }
        },
        Err(e) => {
            send_response(
                400,
                vec![(
                    String::from("content-type"),
                    String::from("text/html; charset=UTF-8"),
                )],
                e.as_bytes().to_vec(),
            );
        }
    }
}

async fn twilio(_headers: Vec<(String, String)>, qry: HashMap<String, Value>, _body: Vec<u8>) {
    flowsnet_platform_sdk::logger::init();

    let text = qry.get("text").unwrap_or(&Value::Null).as_str();
    let to = qry.get("to").unwrap_or(&Value::Null).as_str();
    let resp = match text.and(to) {
        Some(_) => {
            let account_id = std::env::var("TWILIO_ACCOUNT_ID").unwrap();
            let auth_token = std::env::var("TWILIO_AUTH_TOKEN").unwrap();
            let from = std::env::var("TWILIO_FROM").unwrap();
            let client = Client::new(account_id.as_str(), auth_token.as_str());
            client
                .send_message(OutboundMessage::new(
                    from.as_str(),
                    to.unwrap(),
                    text.unwrap(),
                ))
                .await
                .map_err(|x| format!("{:?}", x))
        }
        None => Err(String::from("No 'text' or 'to' in query")),
    };

    match resp {
        Ok(r) => send_response(
            200,
            vec![(
                String::from("content-type"),
                String::from("text/html; charset=UTF-8"),
            )],
            format!("{:?}", r).as_bytes().to_vec(),
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
