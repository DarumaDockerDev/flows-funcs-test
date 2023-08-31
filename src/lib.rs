use flowsnet_platform_sdk::logger;
use github_flows::{
    event_handler, get_octo, listen_to_event,
    octocrab::models::{events::payload::EventPayload, reactions::ReactionContent},
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
        vec!["issue_comment"],
    )
    .await;
}

#[event_handler]
async fn handler(payload: EventPayload) {
    logger::init();
    log::debug!("running github issue comment handler");

    let owner = std::env::var("GITHUB_OWNER").unwrap();
    let repo = std::env::var("GITHUB_REPO").unwrap();

    if let EventPayload::IssueCommentEvent(e) = payload {
        let issue_number = e.comment.id.0;

        // installed app login
        let octo = get_octo(&GithubLogin::Provided(owner.clone()));

        octo.issues(owner.as_str(), repo.as_str())
            .create_comment_reaction(issue_number, ReactionContent::Rocket)
            .await
            .unwrap();
    };
}
