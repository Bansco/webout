use serde::{Deserialize, Serialize};
use std::fs::File;

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

fn main() {
    // TODO: Use actix HTTP client to avoid an extra dependency
    let session: Session = reqwest::blocking::get("http://localhost:9000/api/session/create")
        .unwrap()
        .json()
        .unwrap();

    let session_log_name = session.get_log_name();

    File::create(&session_log_name).expect("Failed to create webout file");

    println!("Webout session started");
    println!("  View online: {}", session.url); // TODO copy id / url to clipboard
    println!("  Session id:  {}\n", session.id);

    let emitter_system = std::thread::spawn(move || emitter::system::spawn(session.clone()));

    // -F: Immediately flush output after each write (-f on linux).
    // -q: Run in quiet mode, omit the start, stop and command status messages.
    let cmd = if cfg!(target_os = "linux") {
        format!("script -f -q {}", &session_log_name)
    } else {
        format!("script -F -q {}", &session_log_name)
    };
    let command = std::process::Command::new("sh").arg("-c").arg(cmd).spawn();

    emitter_system.join().unwrap();

    match command {
        Ok(mut child) => {
            child.wait().expect("Failed to start webout client");
            println!("Webout session ended!");
        }
        Err(error) => {
            println!("Failed to run webout. Error {}", error);
        }
    }
}
