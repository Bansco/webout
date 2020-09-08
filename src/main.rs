use std::process::Command;
use std::thread;
use notify::{Watcher, RecursiveMode, watcher};
use notify::{RawEvent, raw_watcher};
use std::sync::mpsc::channel;
use std::fs::File;

fn main() {
    // Test file to simulate notifications
    let mut file = File::create("watch.log").unwrap();

    // -F: Immediately flush output after each write.
    // -q: Run in quiet mode, omit the start, stop and command status messages.
    let script = "script -F -q terminal.log";
    let command = Command::new("sh")
        .arg("-c")
        .arg(script)
        .spawn();

    thread::spawn(move || {
        let (tx, rx) = channel();

        // Create a watcher object, delivering debounced events. The notification
        // back-end is selected based on the platform.
        let mut watcher = raw_watcher(tx).unwrap();

        // Add a path to be watched. The files will be monitored for changes.
        watcher.watch("./terminal.log", RecursiveMode::Recursive).unwrap();


        loop {
            match rx.recv() {
                Ok(RawEvent{path: Some(path), op: Ok(op), cookie}) => {
                    let msg = format!("{:?} {:?} ({:?})", op, path, cookie);
                    // file.write_all(msg.as_bytes()).unwrap()
                    // Notify
                },
                Ok(event) => println!("broken event: {:?}", event),
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    });

    if let Ok(mut child) = command {
        child.wait().expect("command wasn't running");
        println!("Child has finished its execution!");
    } else {
        println!("ls command didn't start");
    }
}
