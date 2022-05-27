//! # Examples
//!
//! ## Derive
//!
//! ```no_run
//! use clap::{IntoApp, Parser, Subcommand};
//!
//! #[derive(Parser)]
//! struct Cli {
//!     #[clap(subcommand)]
//!     command: Commands,
//! }
//!
//! #[derive(Subcommand)]
//! enum Commands {
//!     /// Generate shell completions
//!     Completions {
//!         /// The shell to generate the completions for
//!         #[clap(arg_enum)]
//!         shell: clap_complete_command::Shell,
//!     },
//! }
//!
//! let cli = Cli::parse();
//!
//! match cli.command {
//!     // e.g. `$ cli completions bash`
//!     Commands::Completions { shell } => {
//!         shell.generate(&mut Cli::command(), &mut std::io::stdout());
//!     }
//! }
//! ```
//!
//! ## Builder
//!
//! ```no_run
//! use clap::{Arg, Command};
//!
//! fn build_cli() -> Command<'static> {
//!     Command::new(env!("CARGO_PKG_NAME"))
//!         .subcommand_required(true)
//!         .subcommand(
//!             Command::new("completions")
//!                 .about("Generate shell completions")
//!                 .arg(
//!                     Arg::new("shell")
//!                         .value_name("SHELL")
//!                         .help("The shell to generate the completions for")
//!                         .required(true)
//!                         .possible_values(clap_complete_command::Shell::possible_values()),
//!                 ),
//!         )
//! }
//!
//! let matches = build_cli().get_matches();
//!
//! match matches.subcommand() {
//!     Some(("completions", sub_matches)) => {
//!         // e.g. `$ cli completions bash`
//!         if let Ok(shell) = sub_matches.value_of_t::<clap_complete_command::Shell>("shell") {
//!             let mut command = build_cli();
//!             shell.generate(&mut command, &mut std::io::stdout());
//!         }
//!     }
//!     _ => {
//!         unreachable!("Exhausted list of subcommands and `subcommand_required` prevents `None`")
//!     }
//! }
//! ```

#![warn(clippy::cast_lossless)]
#![warn(clippy::cast_possible_wrap)]
#![warn(clippy::default_trait_access)]
#![warn(clippy::else_if_without_else)]
#![warn(clippy::empty_enum)]
#![warn(clippy::empty_line_after_outer_attr)]
#![warn(clippy::enum_glob_use)]
#![warn(clippy::equatable_if_let)]
#![warn(clippy::float_cmp)]
#![warn(clippy::fn_params_excessive_bools)]
#![warn(clippy::get_unwrap)]
#![warn(clippy::inefficient_to_string)]
#![warn(clippy::integer_division)]
#![warn(clippy::let_unit_value)]
#![warn(clippy::linkedlist)]
#![warn(clippy::lossy_float_literal)]
#![warn(clippy::macro_use_imports)]
#![warn(clippy::manual_assert)]
#![warn(clippy::manual_ok_or)]
#![warn(clippy::many_single_char_names)]
#![warn(clippy::map_unwrap_or)]
#![warn(clippy::match_bool)]
#![warn(clippy::match_on_vec_items)]
#![warn(clippy::match_same_arms)]
#![warn(clippy::match_wild_err_arm)]
#![warn(clippy::match_wildcard_for_single_variants)]
#![warn(clippy::mem_forget)]
#![warn(clippy::missing_const_for_fn)]
#![warn(clippy::must_use_candidate)]
#![warn(clippy::mut_mut)]
#![warn(clippy::negative_feature_names)]
#![warn(non_ascii_idents)]
#![warn(clippy::option_option)]
#![warn(clippy::redundant_feature_names)]
#![warn(clippy::redundant_pub_crate)]
#![warn(clippy::single_match_else)]
#![warn(clippy::str_to_string)]
#![warn(clippy::string_to_string)]
#![warn(clippy::trait_duplication_in_bounds)]
#![warn(clippy::unused_async)]
#![warn(clippy::unused_self)]
#![warn(clippy::use_self)]
#![warn(clippy::wildcard_dependencies)]
#![warn(clippy::wildcard_imports)]
#![warn(clippy::zero_sized_map_values)]

use std::{ffi::OsString, path::PathBuf, str::FromStr};

use clap::ArgEnum;

/// A [`clap::ArgEnum`] for available shell completions.
///
/// # Examples
///
/// ## Derive
///
/// ```no_run
/// use clap::{Parser, Subcommand};
///
/// #[derive(Parser)]
/// struct Cli {
///     #[clap(subcommand)]
///     command: Commands,
/// }
///
/// #[derive(Subcommand)]
/// enum Commands {
///     Completions {
///         #[clap(arg_enum)]
///         shell: clap_complete_command::Shell,
///     },
/// }
/// ```
///
/// ## Builder
///
/// ```no_run
/// use clap::{Arg, Command};
///
/// fn build_cli() -> Command<'static> {
///     Command::new(env!("CARGO_PKG_NAME"))
///         .subcommand_required(true)
///         .subcommand(
///             Command::new("completions")
///                 .about("Generate shell completions")
///                 .arg(
///                     Arg::new("shell")
///                         .value_name("SHELL")
///                         .help("The shell to generate the completions for")
///                         .required(true)
///                         .possible_values(clap_complete_command::Shell::possible_values()),
///                 ),
///         )
/// }
///
/// let matches = build_cli().get_matches();
///
/// match matches.subcommand() {
///     Some(("completions", sub_matches)) => {
///         if let Ok(shell) = sub_matches.value_of_t::<clap_complete_command::Shell>("shell") {
///             // ...
///         }
///     }
///     _ => {
///         unreachable!("Exhausted list of subcommands and `subcommand_required` prevents `None`")
///     }
/// }
/// ```
#[derive(Clone, Copy)]
#[non_exhaustive]
pub enum Shell {
    /// Bourne Again SHell (bash)
    Bash,
    /// Elvish shell
    Elvish,
    /// Fig
    Fig,
    /// Friendly Interactive SHell (fish)
    Fish,
    /// PowerShell
    PowerShell,
    /// Z SHell (zsh)
    Zsh,
}

impl clap_complete::Generator for Shell {
    fn file_name(&self, name: &str) -> String {
        match self {
            Shell::Bash => clap_complete::Shell::Bash.file_name(name),
            Shell::Elvish => clap_complete::Shell::Elvish.file_name(name),
            Shell::Fish => clap_complete::Shell::Fish.file_name(name),
            Shell::PowerShell => clap_complete::Shell::PowerShell.file_name(name),
            Shell::Zsh => clap_complete::Shell::Zsh.file_name(name),

            Shell::Fig => clap_complete_fig::Fig.file_name(name),
        }
    }

    fn generate(&self, cmd: &clap::Command, buf: &mut dyn std::io::Write) {
        match self {
            Shell::Bash => clap_complete::Shell::Bash.generate(cmd, buf),
            Shell::Elvish => clap_complete::Shell::Elvish.generate(cmd, buf),
            Shell::Fish => clap_complete::Shell::Fish.generate(cmd, buf),
            Shell::PowerShell => clap_complete::Shell::PowerShell.generate(cmd, buf),
            Shell::Zsh => clap_complete::Shell::Zsh.generate(cmd, buf),

            Shell::Fig => clap_complete_fig::Fig.generate(cmd, buf),
        }
    }
}

impl Shell {
    /// See [`clap_complete::generate()`].
    ///
    /// The `command`'s bin name is used as the completion's bin name.
    /// If the `command`'s bin name is not set, it will be set to the `command`'s name.
    pub fn generate(self, command: &mut clap::Command, buffer: &mut dyn std::io::Write) {
        let bin_name = command
            .get_bin_name()
            .unwrap_or_else(|| command.get_name())
            .to_owned();
        clap_complete::generate(self, command, bin_name, buffer)
    }

    /// See [`clap_complete::generate_to()`].
    ///
    /// The `command`'s bin name is used as the completion's bin name.
    /// If the `command`'s bin name is not set, it will be set to the `command`'s name.
    pub fn generate_to<S>(
        self,
        command: &mut clap::Command,
        out_dir: S,
    ) -> Result<PathBuf, std::io::Error>
    where
        S: Into<OsString>,
    {
        let bin_name = command
            .get_bin_name()
            .unwrap_or_else(|| command.get_name())
            .to_owned();
        clap_complete::generate_to(self, command, bin_name, out_dir)
    }

    /// # Examples
    ///
    /// ```no_run
    /// use clap::{Arg, Command};
    ///
    /// Command::new("completions")
    ///     .about("Generate shell completions")
    ///     .arg(
    ///         Arg::new("shell")
    ///             .value_name("SHELL")
    ///             .help("The shell to generate the completions for")
    ///             .required(true)
    ///             .possible_values(clap_complete_command::Shell::possible_values()),
    ///     )
    ///     .get_matches()
    /// # ;
    /// ```
    pub fn possible_values() -> impl Iterator<Item = clap::PossibleValue<'static>> {
        Self::value_variants()
            .iter()
            .filter_map(ArgEnum::to_possible_value)
    }
}

// Used for builder API
impl FromStr for Shell {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for variant in Self::value_variants() {
            if variant.to_possible_value().unwrap().matches(s, false) {
                return Ok(*variant);
            }
        }
        Err(format!("invalid variant: {}", s))
    }
}

// Used for derive API
// Hand-rolled to avoid depending on Clap's `derive` feature
impl ArgEnum for Shell {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::Bash,
            Self::Elvish,
            Self::Fig,
            Self::Fish,
            Self::PowerShell,
            Self::Zsh,
        ]
    }

    fn to_possible_value<'a>(&self) -> Option<clap::PossibleValue<'a>> {
        Some(match self {
            Self::Bash => clap::PossibleValue::new("bash"),
            Self::Elvish => clap::PossibleValue::new("elvish"),
            Self::Fig => clap::PossibleValue::new("fig"),
            Self::Fish => clap::PossibleValue::new("fish"),
            Self::PowerShell => clap::PossibleValue::new("powershell"),
            Self::Zsh => clap::PossibleValue::new("zsh"),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::ArgEnum;

    #[test]
    fn check_casing_bash() {
        assert_eq!(Shell::Bash.to_possible_value().unwrap().get_name(), "bash");
    }
    #[test]
    fn check_casing_elvish() {
        assert_eq!(
            Shell::Elvish.to_possible_value().unwrap().get_name(),
            "elvish",
        );
    }
    #[test]
    fn check_casing_fig() {
        assert_eq!(Shell::Fig.to_possible_value().unwrap().get_name(), "fig");
    }
    #[test]
    fn check_casing_fish() {
        assert_eq!(Shell::Fish.to_possible_value().unwrap().get_name(), "fish");
    }
    #[test]
    fn check_casing_powershell() {
        assert_eq!(
            Shell::PowerShell.to_possible_value().unwrap().get_name(),
            "powershell",
        );
    }
    #[test]
    fn check_casing_zsh() {
        assert_eq!(Shell::Zsh.to_possible_value().unwrap().get_name(), "zsh");
    }

    #[test]
    fn check_order() {
        let names = Shell::value_variants()
            .iter()
            .map(|shell| shell.to_possible_value().unwrap().get_name())
            .collect::<Vec<_>>();

        let mut sorted = names.clone();
        sorted.sort_unstable();

        assert_eq!(names, sorted);
        assert_eq!(
            names,
            vec!["bash", "elvish", "fig", "fish", "powershell", "zsh"],
        );
    }
}
