use clap::ValueEnum;
use std::collections::HashMap;
use std::error::Error;
use std::process::Command;

pub use context::Context;
pub use rel_path::RelPath;

#[macro_use]
extern crate anytest_derive;

mod context;
mod language;
mod named_pattern;
mod registry;
mod rel_path;
mod test_framework;

pub type LineNr = usize;
pub type ArgsVec = Vec<String>;
pub type EnvHashMap = HashMap<String, String>;

#[derive(ValueEnum, Clone, Debug)]
pub enum Scope {
    Suite,
    File,
    Line,
}

pub fn build_command(scope: Scope, context: Context) -> Result<Command, Box<dyn Error>> {
    let mut command = Command::new("echo");
    command.current_dir(&context.root());

    let registry = registry::Registry::new();

    for framework in registry {
        if framework.is_suitable_for(&context) {
            println!("{}", framework.program());
            println!("{} - {}", framework.language_name(), framework.name());
        }
    }

    match scope {
        Scope::Suite => command.args(["suite"]),
        Scope::File => command.args(["file", context.rel().to_str().unwrap()]),
        Scope::Line => command.args([
            "line",
            context.rel().to_str().unwrap(),
            context.line().unwrap_or(1).to_string().as_str(),
        ]),
    };

    Ok(command)
}
