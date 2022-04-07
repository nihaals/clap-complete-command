use clap::{Arg, Command};

fn build_cli() -> Command<'static> {
    Command::new(env!("CARGO_PKG_NAME")).arg(
        Arg::new("completion")
            .help("Generate shell completions")
            .possible_values(clap_complete_command::Shell::possible_values()),
    )
}

fn main() {
    let matches = build_cli().get_matches();

    if let Ok(shell) = matches.value_of_t::<clap_complete_command::Shell>("completion") {
        let mut command = build_cli();
        shell.generate(&mut command, &mut std::io::stdout());
    }
}
