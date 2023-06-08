use tokio;

mod data_objects;
mod errors;
mod filesystem;
mod traits;
use home::home_dir;
use tracing::Level;
use tracing_subscriber;
use traits::initializer::Initializer;

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // builds the subscriber.
        .finish();
    let subscriber_guard = tracing::subscriber::set_default(subscriber);

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
