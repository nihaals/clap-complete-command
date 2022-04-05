# clap-complete-command

[![Crates.io](https://img.shields.io/crates/v/clap_complete_command)](https://crates.io/crates/clap_complete_command)

Reduces boilerplate for adding a completion command to Clap

## Example

```rust
use clap::{IntoApp, Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate shell completion
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
            shell.generate(
                &mut Cli::command(),
                env!("CARGO_PKG_NAME"),
                &mut std::io::stdout(),
            );
        }
    }
}
```
