use std::{env, path::PathBuf};

use clap_generate::generators::{Bash, Elvish, Fish, PowerShell, Zsh};
use cli::app;
use mit_build_tools::{completion, manpage};
use mit_lint::Lint;

#[path = "src/cli/mod.rs"]
mod cli;

fn main() {
    let lint_names: Vec<_> = Lint::all_lints().map(mit_lint::Lint::name).collect();
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let app = app::app(&lint_names);

    completion::generate::<Elvish>(&app, &out_dir.join("elvish_completion"));
    completion::generate::<Fish>(&app, &out_dir.join("fish_completion"));
    completion::generate::<Zsh>(&app, &out_dir.join("zsh_completion"));
    completion::generate::<Bash>(&app, &out_dir.join("bash_completion"));
    completion::generate::<PowerShell>(&app, &out_dir.join("power_shell_completion"));

    manpage::generate(&app, &out_dir, "docs/manpage.template.md");
}
