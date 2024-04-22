use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author,version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
    pub string: String,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Gives sum of ascii
    Sum,
    /// Gives raw ascii
    Raw,
}

