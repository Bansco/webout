use actix::io::SinkWrite;
use actix::Actor;
use actix::Arbiter;
use actix::StreamHandler;
use actix::System;
use futures::stream::StreamExt;
use std::process::Stdio;
use tokio::process::Command;
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::sender::actor::Sender;
use crate::ws_client;
use crate::Session;

pub fn spawn(session: Session) {
    let session_log_name = session.get_log_name();
    let system = System::new("webout-sender-client");

    Arbiter::spawn(async move {
        let wss_url = format!(
            "http://localhost:9000/api/cast/ws/{}?token={}",
            &session.id, session.token
        );

        let client = ws_client::create_client();
        let (_response, framed) = client
            .ws(wss_url)
            .connect()
            .await
            .map_err(|error| {
                println!("Failed to connect to webout servers. {}", error);
            })
            .expect("Failed to connect to webout servers");

        let cmd = format!("tail -f {}", session_log_name);
        let mut command = Command::new("sh");
        command.arg("-c");
        command.arg(cmd);
        command.stdout(Stdio::piped());

        let mut child = command.spawn().expect("failed to spawn command");
        let stdout = child
            .stdout
            .take()
            .expect("child did not have a handle to stdout");
        let framed_stream = FramedRead::new(stdout, BytesCodec::new());

        let (sink, stream) = framed.split();
        let _sender = Sender::create(|ctx| {
            Sender::add_stream(stream, ctx);
            Sender::add_stream(framed_stream, ctx);
            Sender {
                id: session.id,
                sink: SinkWrite::new(sink, ctx),
            }
        });
    });

    system.run().unwrap();
}
