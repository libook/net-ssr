fn main() -> std::io::Result<()> {
    // Check if ./man directory exists, if not create it
    let man_dir =std::path::Path::new("./man");
    if !man_dir.exists() {
        std::fs::create_dir("./man")?;
    }

    {
        let cmd = command::get_interrogator_command();

        let man = clap_mangen::Man::new(cmd);
        let mut buffer: Vec<u8> = Default::default();
        man.render(&mut buffer)?;

        std::fs::write(man_dir.join("interrogator.1"), buffer)?;
    }

    {
        let cmd = command::get_transponder_command();

        let man = clap_mangen::Man::new(cmd);
        let mut buffer: Vec<u8> = Default::default();
        man.render(&mut buffer)?;

        std::fs::write(man_dir.join("transponder.1"), buffer)?;
    }

    Ok(())
}
