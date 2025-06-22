use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Subcommand we want to run
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    /// Cipher an input string
    #[command()]
    Cipher,

    /// Decipher an input string
    #[command()]
    Decipher
}
