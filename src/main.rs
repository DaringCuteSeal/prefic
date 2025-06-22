use std::{io::{self, Read, IsTerminal}, process::exit};

use clap::Parser;
use cli::Args;
use errors::{AppError, AppResult};
use matcher::Matcher;
use words::{get_dict_map, get_dict_reverse_map};

mod words;
mod matcher;
mod errors;
mod cli;

fn main() -> AppResult<()> {
    let args = Args::parse();

    let mut buf = Vec::new();
    let mut stdin = io::stdin();
    if let Err(e) = stdin.read_to_end(&mut buf) {
        println!("I/O error: {}", e);
        exit(1);
    };

    let input_str = match String::from_utf8(buf) {
        Ok(res) => res,
        Err(_) => return Err(AppError::InputError("Failed to convert to string from UTF-8 input".into()))
    };

    let dict = get_dict_map();
    let reverse_dict = get_dict_reverse_map();
    let matcher = Matcher::new(&dict, &reverse_dict);

    let result = match args.command {
        cli::Command::Cipher => matcher.cipher(&input_str),
        cli::Command::Decipher => matcher.decipher(&input_str)
    }?;

    if stdin.is_terminal() {
        // Print extra newline so it's clear to look at?
        println!("\n{}", result);
    } else {
        // Print the raw result
        print!("{}", result);
    }


    Ok(())
}
