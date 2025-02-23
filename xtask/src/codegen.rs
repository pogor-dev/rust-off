use std::{fs, path::Path};

use xshell::{cmd, Shell};

use crate::{
    flags::{self, CodegenType},
    project_root,
};

// pub(crate) mod assists_doc_tests;
// pub(crate) mod diagnostics_docs;
// pub(crate) mod feature_docs;

mod grammar;
// mod lints;
// mod parser_inline_tests;

impl flags::Codegen {
    pub(crate) fn run(self, _sh: &Shell) -> anyhow::Result<()> {
        match self.codegen_type.unwrap_or_default() {
            flags::CodegenType::All => {
                grammar::generate(self.check);
            }
            flags::CodegenType::Grammar => grammar::generate(self.check),
        }
        Ok(())
    }
}

fn reformat(text: String) -> String {
    let sh = Shell::new().unwrap();
    let rustfmt_toml = project_root().join("rustfmt.toml");
    let version = cmd!(sh, "rustup run stable rustfmt --version").read().unwrap_or_default();

    // First try explicitly requesting the stable channel via rustup in case nightly is being used by default,
    // then plain rustfmt in case rustup isn't being used to manage the compiler (e.g. when using Nix).
    let mut stdout = if !version.contains("stable") {
        let version = cmd!(sh, "rustfmt --version").read().unwrap_or_default();
        if !version.contains("stable") {
            panic!(
                "Failed to run rustfmt from toolchain 'stable'. \
                 Please run `rustup component add rustfmt --toolchain stable` to install it.",
            );
        } else {
            cmd!(sh, "rustfmt --config-path {rustfmt_toml} --config fn_single_line=true")
                .stdin(text)
                .read()
                .unwrap()
        }
    } else {
        cmd!(sh, "rustup run stable rustfmt --config-path {rustfmt_toml} --config fn_single_line=true")
            .stdin(text)
            .read()
            .unwrap()
    };
    if !stdout.ends_with('\n') {
        stdout.push('\n');
    }
    stdout
}

fn add_preamble(cg: CodegenType, mut text: String) -> String {
    let preamble = format!("//! Generated by `cargo xtask codegen {cg}`, do not edit by hand.\n\n");
    text.insert_str(0, &preamble);
    text
}

/// Checks that the `file` has the specified `contents`. If that is not the
/// case, updates the file and then fails the test.
#[allow(clippy::print_stderr)]
fn ensure_file_contents(cg: CodegenType, file: &Path, contents: &str, check: bool) -> bool {
    let contents = normalize_newlines(contents);
    if let Ok(old_contents) = fs::read_to_string(file) {
        if normalize_newlines(&old_contents) == contents {
            // File is already up to date.
            return false;
        }
    }

    let display_path = file.strip_prefix(project_root()).unwrap_or(file);
    if check {
        panic!(
            "{} was not up-to-date{}",
            file.display(),
            if std::env::var("CI").is_ok() {
                format!("\n    NOTE: run `cargo xtask codegen {cg}` locally and commit the updated files\n")
            } else {
                "".to_owned()
            }
        );
    } else {
        eprintln!("\n\x1b[31;1merror\x1b[0m: {} was not up-to-date, updating\n", display_path.display());

        if let Some(parent) = file.parent() {
            let _ = fs::create_dir_all(parent);
        }
        fs::write(file, contents).unwrap();
        true
    }
}

fn normalize_newlines(s: &str) -> String {
    s.replace("\r\n", "\n")
}
