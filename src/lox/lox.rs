use core::error;
use std::io;

// TODO :: peut etre integrer clap pour rendre le cli propre - ou pas et j fais mon setup mio meme
// du cil et j apprends plus 
use tracing::{error, info, warn}; 

pub struct Lox;


#[derive(thiserror::Error,Debug)]
pub enum LoxError {
    // todo improve later 
    #[error("Arguments error: {0}")]
    ArgsError(String),
    #[error("meow")]
    Meow(),
}

pub fn run_file(file: &str) -> Result<Lox, LoxError> {
    info!("Running Lox interpreter over {:?}", file);
    Ok(Lox)
}

pub fn run_prompt() -> Result<Lox, LoxError> {
    // Open the equivalent of java InputStreamReader + BufferedReader
    let mut buffer = String::new();
    let stdin = io::stdin();

    // essayer une fois abec un input 
    stdin.read_line(&mut buffer); // should use ? and map the correct event with a match I guess
    // (on the next level on top)
    // loop et laisser le mec cuisiner tant qu'il leave pas
    //
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
            },
            1 => {
                run_file(args[0])
            },
            _ => {
                Err(LoxError::ArgsError(String::from("too many arguments - expected 1")))
            } 
        }
    }        

}

