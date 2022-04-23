use clap::{Arg, Command};

fn build_cli() -> Command<'static> {
    Command::new(env!("CARGO_PKG_NAME"))
        .subcommand_required(true)
        .subcommand(
            Command::new("completions")
                .about("Generate shell completions")
                .arg(
                    Arg::new("shell")
                        .value_name("SHELL")
                        .help("The shell to generate the completions for")
                        .required(true)
                        .possible_values(clap_complete_command::Shell::possible_values()),
                ),
        )
}

fn main() {
    let matches = build_cli().get_matches();

    match matches.subcommand() {
        Some(("completions", sub_matches)) => {
            if let Ok(shell) = sub_matches.value_of_t::<clap_complete_command::Shell>("shell") {
                let mut command = build_cli();
                shell.generate(&mut command, &mut std::io::stdout());
            }
        }
        _ => {
            unreachable!("Exhausted list of subcommands and `subcommand_required` prevents `None`")
        }
    }
}
