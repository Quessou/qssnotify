use actions::get::get;
// External imports
use clap::{arg, ArgGroup, Command};
use tracing::{subscriber::DefaultGuard, Level};

mod actions;
mod data_objects;
mod errors;
mod filesystem;
mod settings;
mod traits;

// Project imports
use traits::initializer::Initializer;

fn initialize_subscriber() -> DefaultGuard {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_default(subscriber)
}

fn str_to_hash(s: &str) -> Result<u64, std::num::ParseIntError> {
    u64::from_str_radix(s, 16).into()
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

    let _guard = initialize_subscriber();

    let initializer = filesystem::filesystem_initializer::FilesystemInitializer::new(
        filesystem::paths::get_app_directory_path()
            .to_str()
            .unwrap(),
    );

    if let Err(e) = initializer.initialize().await {
        if e != errors::initialization_error::InitializationError::AlreadyInitialized {
            tracing::info!(
                "Filesystem issue when trying to create the data directory : {}",
                e
            );
        }
    }

    let settings: settings::Settings =
        settings::read_settings(&filesystem::paths::get_config_file_path())
            .await
            .unwrap();

    if let Some(true) = arguments.get_one::<bool>("list") {
        tracing::trace!("listing registered sentences");
        actions::list::list_sentences().await.unwrap();
    }
    if let Some(hash) = arguments.get_one::<String>("edit") {
        tracing::trace!("Editing sentence {hash}");
        let hash = str_to_hash(hash).expect("Hash parsing failed");
        actions::edit::edit_sentence(hash, &settings)
            .await
            .expect("Could not edit sentence");
    }
    if let Some(true) = arguments.get_one::<bool>("add") {
        tracing::trace!("Adding a sentence");
        actions::add::add_sentence(&settings)
            .await
            .expect("Sentence addition failed");
    }
    if let Some(hash) = arguments.get_one::<String>("delete") {
        let hash = str_to_hash(hash).expect("Hash parsing failed");
        actions::delete::delete_sentence(hash)
            .await
            .expect("Sentence deletion failed");
    }
    if let Some(str_hash) = arguments.get_one::<String>("get") {
        let hash = str_to_hash(str_hash).expect("Hash parsing failed");
        let s = actions::get::get(hash)
            .await
            .expect("Sentence retrieving failed")
            .expect(&format!("Could not find sentence for hash {}", str_hash));
        println!("{}", s);
    }
    if arguments.get_one::<String>("get").is_none() {
        let get_arguments = arguments.get_many::<String>("get");
        if get_arguments.is_some() && get_arguments.unwrap().len() == 0 {
            println!("get without an arg !!!");
            let s = actions::get::get_random()
                .await
                .expect("Sentence retrieving failed")
                .expect("Could not find a random sentence (storage is empty ?)");
            println!("{}", s);
        }
    }
}
