use futures::future::FutureExt;
use serde::{Deserialize, Serialize};
use std::process::ExitStatus;
use tokio::fs::File;
use tokio::process;

mod emitter;
mod ws_client;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    id: String,
    url: String,
    token: String,
}

impl Session {
    fn get_log_name(&self) -> String {
        format!("{}.webout", &self.id)
    }
}

enum WeboutExitReason {
    InputListenerStopped(ExitStatus),
    EmitterSystemStopped,
}

#[tokio::main]
async fn main() {
    // TODO: Use actix HTTP client to avoid an extra dependency
    let session: Session = reqwest::get("http://localhost:9000/api/session/create")
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let session_log_name = session.get_log_name();

    File::create(&session_log_name)
        .await
        .expect("Failed to create webout file");

    println!("Webout session started");
    println!("View online: {}", session.url); // TODO copy id / url to clipboard
    println!("Session id:  {}\n", session.id);

    let emitter_system = {
        let session = session.clone();
        tokio::task::spawn_blocking(move || {
            emitter::system::spawn(session);
        })
    };

    let input_listener = spawn_input_listener(&session_log_name);

    futures::pin_mut!(emitter_system);
    futures::pin_mut!(input_listener);

    let exit_reason = futures::select! {
        _ = emitter_system.fuse() => WeboutExitReason::EmitterSystemStopped,
        exit_status = input_listener.fuse() => WeboutExitReason::InputListenerStopped(exit_status.unwrap()),
    };

    match exit_reason {
        WeboutExitReason::EmitterSystemStopped => {
            println!("Webout server connection terminated unexpectally");
            std::process::exit(1);
        }
        WeboutExitReason::InputListenerStopped(status) => {
            if status.success() {
                println!("Webout session ended! Bye :)");
                std::process::exit(0);
            } else {
                println!("Webout process terminated unexpectally");
                std::process::exit(1);
            }
        }
    }
}

async fn spawn_input_listener(session_log_name: &String) -> Result<ExitStatus, std::io::Error> {
    // -f: Immediately flush output after each write (-F on macOS).
    // -q: Run in quiet mode, omit the start, stop and command status messages.
    let cmd = if cfg!(target_os = "linux") {
        format!("script -f -q {}", &session_log_name)
    } else {
        format!("script -F -q {}", &session_log_name)
    };

    process::Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .spawn()
        .expect("Failed to spawn script command")
        .await
}
