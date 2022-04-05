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

#[derive(clap::ArgEnum, Clone)]
#[non_exhaustive]
pub enum Shell {
    /// Bourne Again SHell (bash)
    Bash,
    /// Elvish shell
    Elvish,
    /// Friendly Interactive SHell (fish)
    Fish,
    /// PowerShell
    PowerShell,
    /// Z SHell (zsh)
    Zsh,

    /// Fig
    Fig,
}

enum Generators {
    Shell(clap_complete::Shell),
    Fig(clap_complete_fig::Fig),
}

impl clap_complete::Generator for Generators {
    fn file_name(&self, name: &str) -> String {
        match self {
            Generators::Shell(shell) => shell.file_name(name),
            Generators::Fig(fig) => fig.file_name(name),
        }
    }

    fn generate(&self, cmd: &clap::Command, buf: &mut dyn std::io::Write) {
        match self {
            Generators::Shell(shell) => shell.generate(cmd, buf),
            Generators::Fig(fig) => fig.generate(cmd, buf),
        }
    }
}

impl Shell {
    #[must_use]
    pub fn to_generator(&self) -> impl clap_complete::Generator {
        match self {
            Self::Bash => Generators::Shell(clap_complete::Shell::Bash),
            Self::Elvish => Generators::Shell(clap_complete::Shell::Elvish),
            Self::Fish => Generators::Shell(clap_complete::Shell::Fish),
            Self::PowerShell => Generators::Shell(clap_complete::Shell::PowerShell),
            Self::Zsh => Generators::Shell(clap_complete::Shell::Zsh),

            Self::Fig => Generators::Fig(clap_complete_fig::Fig),
        }
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
