pub fn matches<'a, 'b>() -> clap::App<'a, 'b> {
    clap::App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .subcommand(clap::App::new("stream").about("Start streaming terminal session"))
        .subcommand(
            clap::App::new("watch").about("Watch a Webout stream").arg(
                clap::Arg::with_name("session-id")
                    .short("i")
                    .long("session-id")
                    .help("Webout stream session identifier")
                    .takes_value(true)
                    .required(true),
            ),
        )
}