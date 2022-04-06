//! # Examples
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
//!     /// Generate shell completion
//!     Completion {
//!         /// The shell to generate the completions for
//!         #[clap(arg_enum)]
//!         shell: clap_complete_command::Shell,
//!     },
//! }
//!
//! let cli = Cli::parse();
//!
//! match cli.command {
//!     // e.g. `$ cli completion bash`
//!     Commands::Completion { shell } => {
//!         shell.generate(
//!             &mut Cli::command(),
//!             env!("CARGO_PKG_NAME"),
//!             &mut std::io::stdout(),
//!         );
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

use std::{ffi::OsString, path::PathBuf};

/// A [`clap::ArgEnum`] for available shell completions.
/// # Examples
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
///     Completion {
///         #[clap(arg_enum)]
///         shell: clap_complete_command::Shell,
///     },
/// }
/// ```
#[derive(clap::ArgEnum, Clone)]
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
    #[clap(name = "powershell")]
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
    #[must_use]
    pub fn to_generator(&self) -> impl clap_complete::Generator {
        self.clone()
    }

    /// See [`clap_complete::generate()`].
    pub fn generate<S>(
        &self,
        command: &mut clap::Command,
        bin_name: S,
        buffer: &mut dyn std::io::Write,
    ) where
        S: Into<String>,
    {
        clap_complete::generate(self.to_generator(), command, bin_name, buffer)
    }

    /// See [`clap_complete::generate_to()`].
    pub fn generate_to<S, T>(
        &self,
        command: &mut clap::Command,
        bin_name: S,
        out_dir: T,
    ) -> Result<PathBuf, std::io::Error>
    where
        S: Into<String>,
        T: Into<OsString>,
    {
        clap_complete::generate_to(self.to_generator(), command, bin_name, out_dir)
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
