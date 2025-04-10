use tracing_subscriber;
use crafting_interpreters::lox::Lox;
use tracing::{error, info};

fn main() {
    // Initialize the tracing subscriber
    tracing_subscriber::fmt::init();

    tracing::info!("Starting program");

    // Parse arguments from CLI
    //let args: Vec<&str> = std::env::args().collect::<Vec<String>>().iter().map(|s| s.as_str()).collect();
    let args = vec!["a"];

    match Lox::new(args) {
        Ok(_) => info!("Lox interpreter initialized successfully"),
        Err(e) => error!("Failed to initialize Lox interpreter: {:?}", e),
    }
}

