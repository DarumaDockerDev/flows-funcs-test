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
        vec!["push", "issue_comment"],
    )
    .await;
}

#[event_handler]
async fn handler(payload: EventPayload) {
    logger::init();
    log::debug!("running github issue comment handler");

    let owner = std::env::var("GITHUB_OWNER").unwrap();
    let repo = std::env::var("GITHUB_REPO").unwrap();

    match payload {
        EventPayload::IssueCommentEvent(e) => {
            let issue_number = e.comment.id.0;

            // installed app login
            let octo = get_octo(&GithubLogin::Provided(owner.clone()));

            octo.issues(owner.as_str(), repo.as_str())
                .create_comment_reaction(issue_number, ReactionContent::Rocket)
                .await
                .unwrap();
        }
        EventPayload::PushEvent(e) => {
            let octo = get_octo(&GithubLogin::Provided(owner.clone()));

            for c in e.commits.iter() {
                log::debug!("Found commit#{}", c.sha);
                let mut commits = octo
                    .repos(owner.as_str(), repo.as_str())
                    .list_commits()
                    .sha(&c.sha)
                    .send()
                    .await
                    .unwrap();
                let commit = commits.items.get_mut(0).unwrap();
                let file_modified = match commit.files {
                    Some(_) => {
                        let files = commit.files.take().unwrap();
                        files
                            .iter()
                            .find(|f| {
                                println!("{:?}", f);
                                f.filename == "lib.rs"
                            })
                            .is_some()
                    }
                    None => false,
                };
                log::debug!("File modified: {}", file_modified);
            }
        }
        _ => {}
    };
}
