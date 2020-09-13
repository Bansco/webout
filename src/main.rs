use serde::{Deserialize, Serialize};
use std::fs::File;

mod sender;
mod ws_client;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    id: String,
    url: String,
    token: String
}

impl Session {
    fn get_log_name(&self) -> String {
        format!("{}.webout", &self.id)
    }
}

fn main() {
    // TODO: Use actix HTTP client to avoid an extra dependency
    let session: Session = reqwest::blocking::get("http://localhost:9000/api/cast/create")
        .unwrap()
        .json()
        .unwrap();

    let session_log_name = session.get_log_name();

    File::create(&session_log_name).expect("Failed to create webout file");

    let _sender_system = std::thread::spawn(move || sender::system::spawn(session.clone()));

    // -F: Immediately flush output after each write.
    // -q: Run in quiet mode, omit the start, stop and command status messages.
    let cmd = format!("script -F -q {}", &session_log_name);
    let command = std::process::Command::new("sh").arg("-c").arg(cmd).spawn();

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
