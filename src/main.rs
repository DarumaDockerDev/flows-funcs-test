use whatsapp_cloud_api::models::{
    Component, ComponentSubType, ComponentType, Message, Parameter, Template,
};
use whatsapp_cloud_api::WhatasppClient;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let access_token = std::env::args().nth(1).expect("No access token");
    let phone_number_id = "288375664358496";
    let to = std::env::args().nth(2).expect("No to number");

    let template_name = "codaotp";
    let language = "en_GB";
    let parameters = Vec::from([Parameter::from_text("123456")]);
    let parameters2 = Vec::from([Parameter::from_text("123456")]);
    let components = Vec::from([
        Component::with_parameters(ComponentType::Body, parameters),
        Component::for_button(ComponentType::Button, ComponentSubType::Url, parameters2, 0),
    ]);
    let template = Template::with_components(template_name, language, components);
    let message = Message::from_template(&to, template, None);

    let client = WhatasppClient::new(&access_token, &phone_number_id);
    let resp = client.send_message(&message).await.unwrap();
    println!("{resp:?}");
}
