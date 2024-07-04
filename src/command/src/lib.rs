use clap::{crate_description, crate_name, crate_version, Arg, ArgAction, Command};

pub fn get_interrogator_command() -> Command {
    Command::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::new("start")
                .short('s')
                .long("start")
                .value_name("START")
                .help("Optional. Broadcast starting at IP address.")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("to")
                .short('t')
                .long("to")
                .value_name("TO")
                .help("Optional. Broadcast IP addresses from --start to --to")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("bind")
                .short('b')
                .long("bind")
                .value_name("BIND")
                .default_value("0.0.0.0")
                .help("Optional. Bind and listen to specified broadcast address (not your IP). You can also customize the port like 0.0.0.0:1090")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .value_name("PORT")
                .default_value("1030")
                .help("Optional. Target port for broadcasting")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("More output about what's happening")
                .action(ArgAction::SetTrue),
        )
}

pub fn get_transponder_command() -> Command {
    Command::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::new("bind")
                .short('b')
                .long("bind")
                .value_name("BIND")
                .default_value("0.0.0.0")
                .help("Optional. Bind and listen to specified broadcast address (not your IP). You can also customize the port like 0.0.0.0:1030")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("More output about what's happening")
                .action(ArgAction::SetTrue),
        )
}
