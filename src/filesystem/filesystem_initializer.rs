use tokio::fs;

use async_trait::async_trait;
use ron;
use tokio::io::AsyncWriteExt;
use tracing;

use super::constants;
use crate::errors::initialization_error::InitializationError;
use crate::settings::Settings;
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

    pub async fn delete_files(&self) -> Result<(), std::io::Error> {
        fs::remove_dir_all(&self.dir).await
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
            self.delete_files().await?;
        }
        tracing::info!("Initializing filesystem");
        fs::create_dir(&self.dir).await?;
        let config_file_path = self.dir.join(constants::CONFIG_FILE_NAME);
        let _ = fs::File::create(self.dir.join(constants::DATA_FILE_NAME)).await?;
        let _ = fs::File::create(&config_file_path).await?;

        let default_settings = Settings::default();
        let settings_file = fs::File::create(config_file_path)
            .await
            .expect("Opening of settings file failed");
        let mut writer: tokio::io::BufWriter<_> = tokio::io::BufWriter::new(settings_file);
        let r = writer
            .write_all(ron::to_string(&default_settings).unwrap().as_bytes())
            .await;
        match r {
            Ok(_) => tracing::trace!("Writing of default settings on filesystem successful"),
            Err(_) => tracing::error!("Writing of default settings on filesystem failed"),
        }
        let r = writer.flush().await;
        match r {
            Ok(_) => tracing::trace!("Flushing of default settings successful"),
            Err(_) => tracing::error!("Flushing of default settings failed"),
        }

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
        initializer.delete_files().await.expect("Cleaning failed");
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
        initializer.delete_files().await.expect("Cleaning failed");
    }
}
