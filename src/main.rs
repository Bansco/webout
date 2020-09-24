use futures::future::FutureExt;
use serde::{Deserialize, Serialize};
use std::process::ExitStatus;
use tokio::fs::File;
use tokio::process;

mod constants;
mod cli;
mod emitter;
mod listener;
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
    let args = cli::matches().get_matches();

    match args.subcommand() {
        ("stream", Some(_subcommand)) => {
            stream().await;
        }
        ("watch", Some(subcommand)) => {
            // Safe to unwrap because is a required arg attribute
            let session_id = subcommand.value_of("session-id").unwrap().to_owned();
            watch(session_id).await;
        }
        // Clap handles non "stream" and "watch" options
        _ => {}
    };
}

async fn stream() {
    let create_session_url = format!("{}/api/session/create", constants::SERVER_URL);
    let session: Session = reqwest::get(create_session_url.as_str())
        .await
        .expect("Failed to connect to Webout servers")
        .json()
        .await
        .expect("Failed to read Webout server response");

    let session_log_name = session.get_log_name();

    File::create(&session_log_name)
        .await
        .expect("Failed to create Webout file");

    println!("Webout session started");
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
                std::process::exit(status.code().unwrap_or(1));
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

async fn watch(session_id: String) {
    let listener_system = tokio::task::spawn_blocking(move || {
        listener::system::spawn(session_id);
    });

    listener_system.await.unwrap();
}
