#[path = "src/command.rs"]
mod command;

use clap::crate_name;
use clap_complete::{generate_to, Shell};

fn main() -> std::io::Result<()> {
    struct BinInfo {
        name: &'static str,
        get_command: Box<dyn Fn() -> clap::Command>,
    }

    let bin_info_list = vec![
        BinInfo {
            name: "interrogator",
            get_command: Box::new(command::get_interrogator_command),
        },
        BinInfo {
            name: "transponder",
            get_command: Box::new(command::get_transponder_command),
        },
    ];

    // Check NET_SSR_BUILD_WITH_MAN
    if std::env::var("NET_SSR_BUILD_WITH_MAN").is_ok() {
        // Check if ./man directory exists, if not create it
        let man_dir = std::path::Path::new("./man");
        if !man_dir.exists() {
            std::fs::create_dir("./man")?;
        }

        for bin_info in bin_info_list.iter() {
            let man = clap_mangen::Man::new((bin_info.get_command)()).title(crate_name!());
            let mut buffer: Vec<u8> = Default::default();
            man.render(&mut buffer)?;

            std::fs::write(man_dir.join(format!("{}.1", bin_info.name)), buffer)?;
        }
    }

    // Check NET_SSR_BUILD_WITH_COMPLETIONS
    if std::env::var("NET_SSR_BUILD_WITH_COMPLETIONS").is_ok() {
        // Check if ./completions directory exists, if not create it
        let completions_dir = std::path::Path::new("./completions");
        if !completions_dir.exists() {
            std::fs::create_dir("./completions")?;
        }

        for bin_info in bin_info_list.iter() {
            let cmd = &mut (bin_info.get_command)();

            generate_to(Shell::Bash, cmd, bin_info.name, completions_dir)?;
            generate_to(Shell::Fish, cmd, bin_info.name, completions_dir)?;
            generate_to(Shell::Zsh, cmd, bin_info.name, completions_dir)?;
            generate_to(Shell::Elvish, cmd, bin_info.name, completions_dir)?;
            generate_to(Shell::PowerShell, cmd, bin_info.name, completions_dir)?;
            generate_to(clap_complete_nushell::Nushell, cmd, bin_info.name, completions_dir)?;
            generate_to(clap_complete_fig::Fig, cmd, bin_info.name, completions_dir)?;
        }
    }

    Ok(())
}
