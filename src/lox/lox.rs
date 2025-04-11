use core::error;

use std::io::{self, Write};

// TODO :: peut etre integrer clap pour rendre le cli propre - ou pas et j fais mon setup mio meme
// du cil et j apprends plus
use tracing::{debug, error, info, warn};

pub struct Lox;

#[derive(thiserror::Error, Debug)]
pub enum LoxError {
    // todo improve later
    #[error("Arguments error: {0} ")]
    ArgsError(String),
    #[error("Input error: {0}")]
    InputStreamError(String),
}

pub fn run_file(file: &str) -> Result<Lox, LoxError> {
    info!("Running Lox interpreter over {:?}", file);
    Ok(Lox)
}

pub fn run_prompt() -> Result<Lox, LoxError> {
    // Open the equivalent of java InputStreamReader + BufferedReader
    let stdin = io::stdin();
    let mut buffer = String::new();

    loop {
        // Clear the buffer before reading new input
        buffer.clear();

        // Prompt the user for input
        print!("> ");

        let _ = io::stdout().flush(); // Ensure the prompt is displayed immediately

        match stdin.read_line(&mut buffer) {
            Ok(n) => {
                debug!("{n} bytes read");
                // Trim the buffer to remove newline characters
                let input = buffer.trim();
                debug!("user wrote {input}");

                match input {
                    "q" => {
                        info!("q was pressed");
                        info!("Closing Lox's CLI ...");
                        break;
                    }
                    "meow" => {
                        warn!("meow meow");
                        break;
                    }
                    _ => {
                        // TODO: Implement linked behaviour
                    }
                }
            }
            Err(e) => {
                error!("error: {e}");
                // You might want to return an error here or continue the loop
            }
        }
    }

    // Return an instance of Lox (you may need to adjust this based on your implementation)
    Ok(Lox)
}

impl Lox {
    pub fn new(args: Vec<&str>) -> Result<Self, LoxError> {
        // Too many arguments
        // One argument (expected use case)
        // Reste des cas
        match args.len() {
            0 => {
                // prompt where user can enter and execute code one line at a time
                run_prompt()
            }
            1 => run_file(args[0]),
            _ => {
                let mut error_string = String::from("too many arguments - expected 1 got ");
                error_string.push_str(args.len().to_string().as_str());
                Err(LoxError::ArgsError(error_string))
            }
        }
    }
}
