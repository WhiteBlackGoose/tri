use std::path::Path;

include!("./src/cli_command.rs");

fn main() -> std::io::Result<()> {
    let cmd = get_cli_command();
    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;
    std::fs::write(Path::new("tri.1"), buffer)?;

    let mut cmd = get_cli_command();
    clap_complete::generate_to(
        clap_complete::shells::Bash,
        &mut cmd,
        "tri",
        "."
    ).unwrap();

    Ok(())
}
