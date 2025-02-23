// filepath: xtask/src/main.rs
use std::{env, path::PathBuf};

use xshell::Shell;

mod flags;

mod codegen;

fn main() -> anyhow::Result<()> {
    let sh = &Shell::new()?;
    sh.change_dir(project_root());

    if cfg!(debug_assertions) {
        let cmd = flags::Codegen {
            check: false,
            codegen_type: Some(flags::CodegenType::Grammar),
        }; // Run codegen in debug mode
        cmd.run(sh)?;
        return Ok(());
    }

    let flags = flags::Xtask::from_env_or_exit();

    match flags.subcommand {
        flags::XtaskCmd::Codegen(cmd) => cmd.run(sh),
    }
}

/// Returns the path to the root directory of `rust-analyzer` project.
fn project_root() -> PathBuf {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned());
    PathBuf::from(dir).parent().unwrap().to_owned()
}
