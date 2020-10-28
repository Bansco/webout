use serde::{Deserialize, Serialize};
use std::fs::File;
use std::process::ExitStatus;
use std::thread;

mod cli;
mod constants;
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
    InputListenerStopped(Result<ExitStatus, std::io::Error>),
    EmitterSystemStopped,
}

fn main() {
    let args = cli::prompt().get_matches();
    match args.subcommand() {
        ("stream", Some(_subcommand)) => {
            stream();
        }
        ("watch", Some(subcommand)) => {
            // Safe to unwrap because is a required arg attribute
            let session_id = subcommand.value_of("session-id").unwrap().to_owned();
            watch(session_id);
        }
        // Clap handles non "stream" and "watch" options
        _ => {}
    };
}

fn stream() {
    let create_session_url = format!("{}/api/session/create", constants::SERVER_URL);
    let session: Session = reqwest::blocking::get(create_session_url.as_str())
        .expect("Failed to connect to Webout servers")
        .json()
        .expect("Failed to read Webout server response");

    let session_log_name = session.get_log_name();
    File::create(&session_log_name).expect("Failed to create Webout file");

    println!("Webout session started");
    println!("Session id: {}\n", session.id);

    let (exit, on_exit) = crossbeam_channel::bounded(1);
    let exit_emitter = exit.clone();
    let exit_listener = exit.clone();

    thread::spawn(move || {
        emitter::system::spawn(session.clone()).unwrap();
        exit_emitter
            .send(WeboutExitReason::EmitterSystemStopped)
            .unwrap();
    });

    thread::spawn(move || {
        let exit_status = spawn_input_listener(&session_log_name);
        exit_listener
            .send(WeboutExitReason::InputListenerStopped(exit_status))
            .unwrap();
    });

    match on_exit.recv().expect("Webout terminated unexpectally") {
        WeboutExitReason::EmitterSystemStopped => {
            println!("Webout server connection terminated unexpectally");
            std::process::exit(1);
        }
        WeboutExitReason::InputListenerStopped(Err(err)) => {
            println!("Webout terminated unexpectally. Error {}", err);
            std::process::exit(1);
        }
        WeboutExitReason::InputListenerStopped(Ok(status)) => {
            if status.success() {
                println!("Webout session ended! Bye :)");
                std::process::exit(0);
            } else {
                println!("Webout process terminated unexpectally");
                std::process::exit(status.code().unwrap_or(1));
            }
        }
    };
}

fn spawn_input_listener(session_log_name: &String) -> Result<ExitStatus, std::io::Error> {
    // -f: Immediately flush output after each write (-F on macOS).
    // -q: Run in quiet mode, omit the start, stop and command status messages.
    let cmd = if cfg!(target_os = "linux") {
        format!("script -f -q {}", &session_log_name)
    } else {
        format!("script -F -q {}", &session_log_name)
    };

    std::process::Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .spawn()
        .expect("Failed to spawn script command")
        .wait()
}

fn watch(session_id: String) {
    let system = thread::spawn(move || {
        listener::system::spawn(session_id);
    });

    system.join().unwrap();
}
