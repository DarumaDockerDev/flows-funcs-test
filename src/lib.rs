use std::{ffi::OsStr, io::Write};

use async_openai::{
    error::OpenAIError,
    types::{CreateAssistantFileRequest, CreateFileRequestArgs, OpenAIFilePurpose},
    Client,
};
use flowsnet_platform_sdk::logger;
use github_flows::{
    event_handler, get_octo, listen_to_event,
    octocrab::models::{
        reactions::ReactionContent,
        webhook_events::{WebhookEvent, WebhookEventPayload},
    },
    GithubLogin,
};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn on_deploy() {
    let owner = std::env::var("GITHUB_OWNER").unwrap();
    let repo = std::env::var("GITHUB_REPO").unwrap();
    listen_to_event(
        &GithubLogin::Provided(owner.clone()),
        owner.as_str(),
        repo.as_str(),
        vec!["push", "issue_comment"],
    )
    .await;
}

#[event_handler]
async fn handler(event_result: Result<WebhookEvent, serde_json::Error>) {
    logger::init();
    let owner = std::env::var("GITHUB_OWNER").unwrap();
    let repo = std::env::var("GITHUB_REPO").unwrap();

    match event_result {
        Ok(payload) => {
            match payload.specific {
                WebhookEventPayload::IssueComment(e) => {
                    let octo = get_octo(&GithubLogin::Provided(owner.clone()));

                    octo.issues(owner.as_str(), repo.as_str())
                        .create_comment_reaction(e.comment.id.0, ReactionContent::Rocket)
                        .await
                        .unwrap();
                }
                WebhookEventPayload::Push(e) => {
                    let file_path = std::env::var("UPLOAD_FILE_OF_REPO").unwrap();
                    let file_updated = e.commits.iter().find(|c| {
                        c.modified
                            .iter()
                            .find(|f| f.as_str() == file_path.as_str())
                            .is_some()
                    });

                    if file_updated.is_some() {
                        let octo = get_octo(&GithubLogin::Provided(owner.clone()));
                        let resp = octo
                            .repos(owner.as_str(), repo.as_str())
                            .raw_file(e.commits.get(0).unwrap().id.clone(), &file_path)
                            .await
                            .unwrap();
                        let content = resp.text().await.unwrap();

                        // Write content to local temp file
                        let file_name = std::path::Path::new(&file_path).file_name().unwrap();
                        let mut file = std::fs::File::create(&file_name).unwrap();
                        file.write_all(content.as_bytes()).unwrap();

                        // Upload file
                        upload_file(file_name).await.unwrap();

                        // Remove file
                        std::fs::remove_file(&file_name).unwrap();
                    }
                }
                WebhookEventPayload::Unknown(_) => {
                    log::debug!("Unknown event");
                }
                _ => {}
            };
        }
        Err(e) => {
            log::error!("{:?}", e);
        }
    }
}

async fn upload_file(file_name: &OsStr) -> Result<(), OpenAIError> {
    let client = Client::new();
    let assistant_id = std::env::var("ASSISTANT_ID").unwrap();

    if let Some(uploaded_file_id) = store_flows::get(file_name.to_str().unwrap()) {
        let uploaded_file_id = uploaded_file_id.as_str().unwrap();
        // The file may has been deleted, so there is no need to check the result
        _ = client
            .assistants()
            .files(&assistant_id)
            .delete(uploaded_file_id)
            .await;
    }

    let create_file_req = CreateFileRequestArgs::default()
        .file(file_name)
        .purpose(
            serde_json::to_value(&OpenAIFilePurpose::Assistants)
                .unwrap()
                .as_str()
                .unwrap(),
        )
        .build()?;
    let file_id = client.files().create(create_file_req).await?.id;

    log::debug!("File uploaded with file_id: {file_id}");
    store_flows::set(
        file_name.to_str().unwrap(),
        serde_json::Value::String(file_id.clone()),
        None,
    );

    let create_assis_file_req = CreateAssistantFileRequest { file_id };
    client
        .assistants()
        .files(&assistant_id)
        .create(create_assis_file_req)
        .await?;

    Ok(())
}
