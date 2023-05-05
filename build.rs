use std::path::Path;

include!("./src/cli_command.rs");


fn generate_man_page(out_dir: &Path) -> std::io::Result<()> {
    let cmd = get_cli_command();
    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;
    std::fs::write(out_dir.join("tri.1"), buffer)?;
    Ok(())
}

fn generate_autocomplete(out_dir: &Path) -> std::io::Result<()> {
    let mut cmd = get_cli_command();
    let shells: Vec<clap_complete::Shell> = vec![
        clap_complete::Shell::Bash
      , clap_complete::Shell::Elvish
      , clap_complete::Shell::Fish
      , clap_complete::Shell::PowerShell
      , clap_complete::Shell::Zsh
    ];
    for shell in shells {
        clap_complete::generate_to(
            shell,
            &mut cmd,
            "tri",
            out_dir.as_os_str().to_str().unwrap()
        )?;
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let out_dir = Path::new("artifacts");
    if !out_dir.exists() {
        std::fs::create_dir(out_dir).unwrap();
    }
    generate_autocomplete(out_dir)?;
    generate_man_page(out_dir)?;
    Ok(())
}
