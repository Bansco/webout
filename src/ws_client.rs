use awc::ClientBuilder;
use awc::Connector;
use std::sync::Arc;

pub fn create_client() -> awc::Client {
    // Websockets must use http/1.1 as long as RFC8441 is not implemented.
    // https://github.com/actix/actix-web/issues/1069
    let mut config = rustls::ClientConfig::new();
    config.alpn_protocols = vec![b"http/1.1".to_vec()];
    config
        .root_store
        .add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);

    let connector = Connector::new().rustls(Arc::new(config)).finish();

    ClientBuilder::new().connector(connector).finish()
}
