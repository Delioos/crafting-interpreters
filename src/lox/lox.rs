use std::{
    fs,
    io::{self, Write},
    process::ExitCode,
};

// TODO :: peut etre integrer clap pour rendre le cli propre - ou pas et j fais mon setup mio meme
// du cil et j apprends plus
use tracing::{debug, error, info, warn};
use tracing_subscriber::reload::Error;

pub struct Lox {
    had_error: bool,
}

#[derive(thiserror::Error, Debug)]
pub enum LanguageError {
    // equivalent of
    /*
     * private static void report(in line, Sgtirngwhere, String message) ... in java
     */
    #[error("[Line {0}] @ {1} : {2}")]
    GenericError(u8, String, String),
    #[error("Scanning Error: {0}")]
    ScanningError(String),
}

#[derive(thiserror::Error, Debug)]
pub enum ScanningError {
    // (line?, where, message)
    #[error("[CLI] Arguments error in {0} : {1}")]
    ArgsError(String, String),
    #[error("[Line {0}] Input error in {1} : {2}")]
    InputStreamError(u64, String, String),
    #[error("[Line {0}] File error in {1} : {2}")]
    FileReadingError(u64, String, String),
}

pub fn run_file(source_file: &str) -> Result<Lox, ScanningError> {
    info!("Running Lox interpreter over {source_file}");
    let contents = fs::read_to_string(source_file);

    // had error handling might be useless but idk
    let mut lox = Lox { had_error: false };

    match contents {
        Ok(file_content) => {
            info!("Scanning...");
            // TODO: check if error in lines and change lox state to had_error
            //let tokens: Vec<Token> =
            let tokens: Vec<&str> = file_content.trim().split(" ").collect();
            tokens.iter().for_each(|token| {
                debug!("{token}");
            });
        }
        // TODO: improve w/ custom error
        Err(e) => error!("error: {e}"),
    }

    Ok(lox)
}

// TODO : add a help menu
pub fn run_prompt() -> Result<Lox, ScanningError> {
    // Open the equivalent of java InputStreamReader + BufferedReader
    let stdin = io::stdin();
    let mut buffer = String::new();
    let mut lox = Lox { had_error: false };

    // TODO : refacto into a run_line function
    loop {
        // Clear the buffer before reading new input
        buffer.clear();
        lox.had_error = false;

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
    Ok(Lox { had_error: false })
}

impl Lox {
    pub fn new(args: Vec<&str>) -> Result<Self, ScanningError> {
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
                let error_position = String::from("arguments passing");
                let mut error_message = String::from("too many arguments - expected 1 got ");
                error_message.push_str(args.len().to_string().as_str());
                Err(ScanningError::ArgsError(error_position, error_message))
            }
        }
    }
}
