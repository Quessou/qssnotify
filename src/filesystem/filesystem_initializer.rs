use std::fs;

use async_trait::async_trait;
use tracing;

use super::constants;
use crate::errors::initialization_error::InitializationError;
use crate::traits::initializer::Initializer;

static FILES: [&str; 2] = [constants::CONFIG_FILE_NAME, constants::DATA_FILE_NAME];

pub struct FilesystemInitializer {
    dir: std::path::PathBuf,
}

impl FilesystemInitializer {
    pub fn new(dir: &str) -> FilesystemInitializer {
        FilesystemInitializer {
            dir: std::path::PathBuf::from(dir),
        }
    }

    pub fn is_consistent(&self) -> bool {
        !self.dir.exists() || self.is_initialized()
    }

    pub fn is_initialized(&self) -> bool {
        self.dir.exists() && FILES.iter().all(|f| self.dir.join(f).exists())
    }

    pub fn delete_files(&self) -> Result<(), std::io::Error> {
        fs::remove_dir_all(&self.dir)
    }
}

#[async_trait]
impl Initializer for FilesystemInitializer {
    #[tracing::instrument(name = "Initialize filesystem", skip(self))]
    async fn initialize(&self) -> Result<(), InitializationError> {
        if self.is_initialized() {
            tracing::info!("Filesystem already initialized");
            return Err(InitializationError::AlreadyInitialized);
        } else if !self.is_consistent() {
            // TODO : Do we really want to delete everything ?
            tracing::warn!("Filesystem inconsistent, deleting everything");
            self.delete_files()?;
        }
        tracing::info!("Initializing filesystem");
        fs::create_dir(&self.dir)?;
        let _ = fs::File::create(self.dir.join(constants::DATA_FILE_NAME))?;
        let _ = fs::File::create(self.dir.join(constants::CONFIG_FILE_NAME))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_initializer(path: &str) -> FilesystemInitializer {
        FilesystemInitializer::new(path)
    }

    #[tokio::test]
    async fn test_initialize() {
        let initializer = get_test_initializer("/tmp/test_initialize");
        initializer
            .initialize()
            .await
            .expect("Initialization failed");
        assert!(initializer.is_initialized());
        initializer.delete_files().expect("Cleaning failed");
    }
    #[tokio::test]
    async fn test_already_initialized() {
        let initializer = get_test_initializer("/tmp/test_already_initialized");
        initializer
            .initialize()
            .await
            .expect("Initialization failed");
        let r = initializer.initialize().await;
        assert!(r.is_err() && r.unwrap_err() == InitializationError::AlreadyInitialized);
        initializer.delete_files().expect("Cleaning failed");
    }
}
