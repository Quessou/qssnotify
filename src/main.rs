// External imports
use clap::{arg, builder::Arg, ArgGroup, Command};
use home::home_dir;
use tokio;
use tracing::Level;
use tracing_subscriber;

mod data_objects;
mod errors;
mod filesystem;
mod traits;

// Project imports
use traits::initializer::Initializer;

#[tokio::main]
async fn main() {
    let command = Command::new("qssnotify").about("Allows to have notifications displayed regularly")
        .arg(arg!(--add "Opens a text editor to write a sentence that will be saved"))
        .arg(arg!(--edit <hash> "Opens a text editor and allows to edit the sentence whose hash is given in parameter"))
        .arg(arg!(--delete <hash> "Deletes the sentence whose hash is given in parameter"))
        .arg(arg!(--get [hash] "Returns the sentence whose hash is given in parameter if a hash is specified, or a random sentence otherwise"))
        .arg(arg!(--list "Lists all registered sentences with the associated hash"))
        .group(ArgGroup::new("subcommands").args(["add", "edit", "delete", "get", "list"/*, "help"*/]).required(true));

    let arguments = command.get_matches();

    if let Some(true) = arguments.get_one::<bool>("list") {
        println!("list !!");
    }
    if let Some(c) = arguments.get_one::<String>("edit") {
        println!("edit !! {c}");
    }
    if let Some(true) = arguments.get_one::<bool>("add") {
        println!("add !!");
    }
    if let Some(s) = arguments.get_one::<String>("delete") {
        println!("delete !! {s}");
    }
    if let Some(a) = arguments.get_one::<String>("get") {
        println!("get with an arg !!! {:?}", a);
    }
    // Kinda ugly actually
    if let None = arguments.get_one::<String>("get") {
        let get_arguments = arguments.get_many::<String>("get");
        if get_arguments.is_some() && get_arguments.unwrap().len() == 0 {
            println!("get without an arg !!!");
        }
    }

    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // builds the subscriber.
        .finish();
    let _subscriber_guard = tracing::subscriber::set_default(subscriber);

    let initializer = filesystem::filesystem_initializer::FilesystemInitializer::new(
        home_dir().unwrap().join(".qssnotify").to_str().unwrap(),
    );
    if let Err(e) = initializer.initialize().await {
        if e != errors::initialization_error::InitializationError::AlreadyInitialized {
            tracing::info!(
                "Filesystem issue when trying to create the data directory : {}",
                e
            );
        }
    }
}
