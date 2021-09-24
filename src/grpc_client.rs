//
//  grpc_client.rs - I hate GRPC
//

use crate::{
    client::SubmitGameEvent,
    consts::{SUBMIT_RETRY_MAX, V3_SURVIVAL_HASH},
};
use tokio::sync::mpsc::{Receiver, Sender};

pub struct GameSubmissionClient;

impl GameSubmissionClient {
    pub async fn init(mut sge_recv: Receiver<SubmitGameEvent>, log_sender: Sender<String>) {
        tokio::spawn(async move {
            use crate::grpc_models::game_recorder_client::GameRecorderClient;
            use crate::grpc_models::{ClientStartRequest, SubmitGameRequest};
            let cfg = crate::config::cfg();
            let mut client = GameRecorderClient::connect(cfg.grpc_host.clone())
                .await
                .expect("No Connection");
            let res = client
                .client_start(ClientStartRequest {
                    version: "0.6.8".to_owned(),
                })
                .await
                .expect("A");
            log::info!("MOTD: {}", res.get_ref().motd);
            while let Some(sge) = sge_recv.recv().await {
                log::info!("Got submit req");

                if !should_submit(&sge) {
                    continue;
                }

                let mut res = client
                    .submit_game(SubmitGameRequest::from_compiled_run(sge.0.clone()))
                    .await;
                for _i in 0..SUBMIT_RETRY_MAX {
                    if res.is_ok() {
                        break;
                    }
                    res = client
                        .submit_game(SubmitGameRequest::from_compiled_run(sge.0.clone()))
                        .await;
                }

                if res.is_ok() {
                    let res = res.as_ref().unwrap().get_ref();
                    log_sender
                        .send(format!("Submitted {}", res.game_id))
                        .await
                        .expect("AAA");

                    if cfg.auto_clipboard {
	                    let mut clipboard = arboard::Clipboard::new().unwrap();
                        let cool = format!("{}/games/{}", cfg.host, res.game_id);
                        clipboard.set_text(cool.into()).unwrap();
                    }
                } else {
                    log_sender
                        .send(format!("Failed to Submit"))
                        .await
                        .expect("AAA");
                }
            }
        });
    }
}

#[rustfmt::skip]
fn should_submit(data: &SubmitGameEvent) -> bool{
    let cfg = crate::config::cfg();
    let is_non_default = data.0.level_hash_md5.ne(&V3_SURVIVAL_HASH.to_uppercase());
    if is_non_default && !cfg.submit.non_default_spawnsets { return false; }
    if data.0.is_replay && !cfg.submit.replay_stats { return false; }
    if !data.0.is_replay && !cfg.submit.stats { return false; }
    true
}
