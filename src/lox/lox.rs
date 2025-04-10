use core::error;

// TODO :: peut etre integrer clap pour rendre le cli propre - ou pas et j fais mon setup mio meme
// du cil et j apprends plus 
use tracing::{error, info, warn}; 

pub struct Lox;


#[derive(thiserror::Error,Debug)]
pub enum SetupError {
    // todo improve later 
    #[error("Arguments error: {0}")]
    ArgsError(String),
}

pub fn run_file(file: &str) {
    info!("Running Lox interpreter over {:?}", file);
}

impl Lox {
    pub fn new(args: Vec<&str>) -> Result<Self, SetupError> {
        // Too many arguments
        // One argument (expected use case)
        // Reste des cas
        match args.len() {
            0 => {
                // TODO: improve and make a dynamic cli parser so use can type the name of the file
                // he want to lex 
                Err(SetupError::ArgsError(String::from("not enough args - expected 1")))
            },
            1 => {
                run_file(args[0]);
                Ok(Lox)
            },
            _ => {
                Err(SetupError::ArgsError(String::from("too many arguments - expected 1")))
            } 
        }
    }        

}

