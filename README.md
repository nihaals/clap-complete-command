# clap-complete-command

[![crates.io](https://img.shields.io/crates/v/clap_complete_command)](https://crates.io/crates/clap_complete_command)

Reduces boilerplate for adding a completion command to Clap

## Examples

### Derive

```rust
use clap::{IntoApp, Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate shell completions
    Completion {
        /// The shell to generate the completions for
        #[clap(arg_enum)]
        shell: clap_complete_command::Shell,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        // e.g. `$ cli completion bash`
        Commands::Completion { shell } => {
            shell.generate(&mut Cli::command(), &mut std::io::stdout());
        }
    }
}
```

### Builder

```rust
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

    // e.g. `$ cli bash`
    if let Ok(shell) = matches.value_of_t::<clap_complete_command::Shell>("completion") {
        let mut command = build_cli();
        shell.generate(&mut command, &mut std::io::stdout());
    }
}
```

## Supported shells

The supported shells can be seen in `clap_complete_command::Shell`.
