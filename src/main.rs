#[macro_use]
extern crate quicli;
extern crate console;
extern crate dialoguer;
extern crate git2;
extern crate heck;
extern crate indicatif;
extern crate liquid;
extern crate regex;
extern crate remove_dir_all;
extern crate walkdir;
extern crate chrono;

mod cargo;
mod emoji;
mod git;
mod interactive;
mod progressbar;
mod template;
mod placeholders;

use console::style;
use quicli::prelude::*;
use std::env;

/// Generate a new Cargo project from a given template
///
/// Right now, only git repositories can be used as templates. Just execute
///
/// $ cargo generate --git https://github.com/user/template.git --name foo
///
/// and a new Cargo project called foo will be generated.
///
/// TEMPLATES:
///
/// In templates, the following placeholders can be used:
///
/// - `project-name`: Name of the project, in dash-case
///
/// - `crate_name`: Name of the project, but in a case valid for a Rust
///   identifier, i.e., snake_case
///
/// - `authors`: Author names, taken from usual environment variables (i.e.
///   those which are also used by Cargo and git)
#[derive(StructOpt)]
#[structopt(bin_name = "cargo")]
pub enum Cli {
    #[structopt(name = "generate")]
    Generate(Args),
}

#[derive(Debug, StructOpt)]
pub struct Args {
    #[structopt(long = "git")]
    git: String,
    #[structopt(long = "name")]
    name: Option<String>,
}

main!(|_cli: Cli| {
    let Cli::Generate(args) = Cli::from_args();
    let name = match &args.name {
        Some(ref n) => n.to_string(),
        None => interactive::name()?,
    };

    let project_dir = env::current_dir()
        .unwrap_or_else(|_e| ".".into())
        .join(&name);

    ensure!(
        !project_dir.exists(),
        "Target directory `{}` already exists, aborting.",
        project_dir.display()
    );

    git::create(&project_dir, args)?;
    git::remove_history(&project_dir)?;

    let template = template::substitute(&name)?;

    let pbar = progressbar::new();
    pbar.tick();

    template::walk_dir(&project_dir, template, pbar)?;

    git::init(&project_dir)?;

    let dir_string = &project_dir.to_str().unwrap_or("");
    println!(
        "{} {} {} {}",
        emoji::SPARKLE,
        style("Done!").bold().green(),
        style("New project created").bold(),
        style(dir_string).underlined()
    );
});
