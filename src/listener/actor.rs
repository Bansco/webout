use actix::io::SinkWrite;
use actix::*;
use actix_codec::Framed;
use awc::{
    error::WsProtocolError,
    ws::{Codec, Frame, Message},
    BoxedSocket,
};
use bytes::Bytes;
use futures::stream::SplitSink;
use std::io::{self, Write};
use std::time::Duration;

pub struct Listener {
    pub id: String,
    pub sink: SinkWrite<Message, SplitSink<Framed<BoxedSocket, Codec>, Message>>,
}

impl Actor for Listener {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        self.hb(ctx)
    }

    fn stopped(&mut self, _: &mut Context<Self>) {
        System::current().stop();
    }
}

impl Listener {
    fn hb(&self, ctx: &mut Context<Self>) {
        ctx.run_later(Duration::new(5, 0), |act, ctx| {
            act.sink
                .write(Message::Ping(Bytes::from_static(b"")))
                .unwrap();

            act.hb(ctx);

            // client should also check for a timeout here, similar to the
            // server code
        });
    }
}

#[derive(Message)]
#[rtype(result = "()")]
struct ClientCommand(String);

impl Handler<ClientCommand> for Listener {
    type Result = ();

    fn handle(&mut self, msg: ClientCommand, _ctx: &mut Context<Self>) {
        self.sink.write(Message::Text(msg.0)).unwrap();
    }
}

impl StreamHandler<Result<Frame, WsProtocolError>> for Listener {
    fn handle(&mut self, msg: Result<Frame, WsProtocolError>, _: &mut Context<Self>) {
        if let Ok(Frame::Text(chunk)) = msg {
            print!("{}", std::str::from_utf8(&chunk).expect("Failed to parse stream bytes to valid utf8"));
            io::stdout().flush().expect("Failed to flush stream bytes to stdout");
        }
    }

    fn started(&mut self, _ctx: &mut Context<Self>) {}

    fn finished(&mut self, ctx: &mut Context<Self>) {
        ctx.stop();
    }
}

impl actix::io::WriteHandler<WsProtocolError> for Listener {}
