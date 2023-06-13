// External imports
use clap::{arg, ArgGroup, Command};
use home::home_dir;
use tokio;
use tracing::{subscriber::DefaultGuard, Level};
use tracing_subscriber;

mod data_objects;
mod errors;
mod filesystem;
mod traits;

// Project imports
use traits::initializer::Initializer;

use crate::data_objects::sentence::Sentence;

fn initialize_subscriber() -> DefaultGuard {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_default(subscriber)
}

#[tokio::main]
async fn main() {
    let command = Command::new("qssnotify").about("Allows to have notifications displayed regularly")
        .arg(arg!(--add "Opens a text editor to write a sentence that will be saved"))
        .arg(arg!(--edit <hash> "Opens a text editor and allows to edit the sentence whose hash is given in parameter"))
        .arg(arg!(--delete <hash> "Deletes the sentence whose hash is given in parameter"))
        .arg(arg!(--get [hash] "Returns the sentence whose hash is given in parameter if a hash is specified, or a random sentence otherwise"))
        .arg(arg!(--list "Lists all registered sentences with the associated hash"))
        .group(ArgGroup::new("subcommands").args(["add", "edit", "delete", "get", "list"]).required(true));

    let arguments = command.get_matches();

    // Kinda ugly actually
    if let None = arguments.get_one::<String>("get") {
        let get_arguments = arguments.get_many::<String>("get");
        if get_arguments.is_some() && get_arguments.unwrap().len() == 0 {
            println!("get without an arg !!!");
        }
    }
    let _guard = initialize_subscriber();

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

    if let Some(true) = arguments.get_one::<bool>("list") {
        println!("list !!");
    }
    if let Some(c) = arguments.get_one::<String>("edit") {
        println!("edit !! {c}");
    }
    if let Some(true) = arguments.get_one::<bool>("add") {
        println!("add !!");

        filesystem::write::write_data_file(
            &home_dir()
                .unwrap()
                .join(".qssnotify")
                .join(filesystem::constants::DATA_FILE_NAME),
            vec![Sentence::new("mdr xd".to_owned())],
        )
        .await
        .expect("blbl");
        return;
    }
    if let Some(s) = arguments.get_one::<String>("delete") {
        println!("delete !! {s}");
    }
    if let Some(a) = arguments.get_one::<String>("get") {
        println!("get with an arg !!! {:?}", a);
    }

    let sentences = filesystem::read::read_data_file(
        &home_dir()
            .unwrap()
            .join(".qssnotify")
            .join(filesystem::constants::DATA_FILE_NAME),
    )
    .await;

    println!("SENTENCES : {:?}", sentences);
}
