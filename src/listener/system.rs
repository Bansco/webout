use actix::io::SinkWrite;
use actix::Actor;
use actix::Arbiter;
use actix::StreamHandler;
use actix::System;
use futures::stream::StreamExt;

use crate::listener::actor::Listener;
use crate::ws_client;

pub fn spawn(session_id: String) {
    let system = System::new("webout-listener-system");

    Arbiter::spawn(async move {
        let wss_url = format!("http://localhost:9000/api/session/ws/{}", &session_id);

        let client = ws_client::create_client();
        let (_response, framed) = client
            .ws(wss_url)
            .connect()
            .await
            .map_err(|error| {
                println!("Failed to connect to webout servers. {}", error);
            })
            .expect("Failed to connect to webout servers");

        let (sink, stream) = framed.split();
        let _listener = Listener::create(|ctx| {
            Listener::add_stream(stream, ctx);
            Listener {
                id: session_id,
                sink: SinkWrite::new(sink, ctx),
            }
        });
    });

    system.run().unwrap();
}
