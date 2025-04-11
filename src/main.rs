use crafting_interpreters::lox::Lox;
use tracing::{
    debug, error, info,
    instrument::WithSubscriber,
    level_filters::{self, LevelFilter},
};
use tracing_subscriber;

fn main() {
    // Initialize the tracing subscriber
    tracing_subscriber::fmt()
        //.with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .init();

    info!("Starting interpreter");

    // Parse arguments from CLI
    let binding = std::env::args().collect::<Vec<String>>();
    // TODO : improve and use a proper functionnal function
    let mut i = 0;
    let mut args: Vec<&str> = binding
        .iter()
        .map(|s| {
            let arg = s.as_str();
            debug!("arg: {:?} -> {:?}", i, arg);
            i += 1;
            arg
        })
        .collect();

    // Default argument is "target/debug/crafting-interpreters" so we slice the vector
    args.remove(0);

    match Lox::new(args) {
        Ok(_) => info!("Lox interpreter initialized successfully"),
        Err(e) => error!("Failed to initialize Lox interpreter: {:?}", e),
    }
}
