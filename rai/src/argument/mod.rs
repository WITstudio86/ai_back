use std::fmt::Display;

use clap::{Parser, Subcommand};
mod arg_type;
pub use arg_type::Figure;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Arg {
    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(about = "Clears the history massage")]
    Clear(Target),
    #[command(about = "select the figure")]
    Chat(Target),
}

#[derive(Debug, Parser)]
pub struct Target {
    #[arg(short, long, default_value = "chat")]
    pub target: Figure,
}

impl Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.target)
    }
}
