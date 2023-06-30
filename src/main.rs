// External imports
use clap::{arg, ArgGroup, Command};
use tracing::{subscriber::DefaultGuard, Level};

mod actions;
mod core;
mod data_objects;
mod errors;
mod filesystem;
mod os_notifier;
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

#[tokio::main]
async fn main() {
    let command = Command::new("qssnotify").about("Allows to have notifications displayed regularly")
        .arg(arg!(--add "Opens a text editor to write a sentence that will be saved"))
        .arg(arg!(--edit <hash> "Opens a text editor and allows to edit the sentence whose hash is given in parameter"))
        .arg(arg!(--delete <hash> "Deletes the sentence whose hash is given in parameter"))
        .arg(arg!(--get [hash] "Returns the sentence whose hash is given in parameter if a hash is specified, or a random sentence otherwise"))
        .arg(arg!(--list "Lists all registered sentences with the associated hash"))
        .arg(arg!(--daemon "Launch the app in daemon mode"))
        .group(ArgGroup::new("subcommands").args(["add", "edit", "delete", "get", "list", "daemon"]).required(true));

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
    let notifier = if arguments.get_one::<bool>("daemon").as_ref().unwrap() == &&true {
        Some(os_notifier::OsNotifier {})
    } else {
        None
    };
    core::run(arguments, settings, notifier).await;
}
