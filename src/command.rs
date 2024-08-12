use clap::{crate_name, crate_version, Arg, ArgAction, Command};

pub fn get_interrogator_command() -> Command {
    Command::new("interrogator")
        .version(crate_version!())
        .about(format!("Find answering devices. Part of {}.", crate_name!()))
        .long_about("This will broadcast an interrogation into the network(defaults to all networks currently accessed by the device) and listen on port 1090. When an answering machine answers with an IP address, the interrogator will print out the IP address. The interrogator will continue to wait for answers until the user exits using Ctrl+c.")
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
    Command::new("transponder")
        .version(crate_version!())
        .about(format!("Report IP and hostname. Part of {}.", crate_name!()))
        .long_about("This will start a process listening on port 1030. When an interrogator asks within the network, the transponder will answer the IP address.")
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
