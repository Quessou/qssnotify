use serde::{Deserialize, Serialize};
use tokio::io::AsyncReadExt;

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Settings {
    pub editor: String,
}

// TODO : What to put here ?
impl Settings {}

impl Default for Settings {
    fn default() -> Self {
        Self {
            editor: "nvim".to_owned(),
        }
    }
}

#[tracing::instrument(name = "Read settings")]
pub async fn read_settings(path: &std::path::Path) -> Result<Settings, tokio::io::Error> {
    let file = tokio::fs::File::open(path)
        .await
        .expect("File opening failed");
    let mut reader = tokio::io::BufReader::new(file);
    let mut data: Vec<u8> = vec![];
    let r = reader.read_to_end(&mut data).await;

    match r {
        Ok(0) => Err(tokio::io::Error::new(
            tokio::io::ErrorKind::Other,
            "Reading nothing",
        )),
        Ok(_) => Ok(
            ron::from_str(&String::from_utf8(data).expect("Parsing of settings failed")).unwrap(),
        ),
        Err(e) => Err(tokio::io::Error::new(tokio::io::ErrorKind::Other, e)),
    }
}

#[cfg(test)]
mod tests {
    use tokio::io::AsyncWriteExt;

    use super::*;

    async fn write_dummy_settings_file(path: &std::path::Path) -> Result<(), ()> {
        let file = tokio::fs::File::create(path)
            .await
            .expect("File opening failed");
        let settings = Settings::default();
        let mut writer = tokio::io::BufWriter::new(file);
        if let Err(_) = writer
            .write_all(ron::to_string(&settings).unwrap().as_bytes())
            .await
        {
            return Err(());
        };
        if let Err(_) = writer.flush().await {
            return Err(());
        }

        Ok(())
    }

    async fn delete_dummy_settings_file(path: &std::path::Path) -> Result<(), ()> {
        match tokio::fs::remove_file(path).await {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    async fn clean_test_read_settings(path: &std::path::Path) {
        if delete_dummy_settings_file(path).await.is_err() {
            panic!("Recover of failing test failed");
        };
    }

    #[tokio::test]
    async fn test_read_settings() {
        let path = std::path::Path::new("/tmp/dummy_settings");

        // Write dummy file
        if let Err(_) = write_dummy_settings_file(&path).await {
            clean_test_read_settings(path).await;
            return;
        }

        // Read file
        let file = tokio::fs::File::open(path).await;
        let file = match file {
            Ok(f) => f,
            Err(_) => {
                clean_test_read_settings(path).await;
                return;
            }
        };

        let mut reader = tokio::io::BufReader::new(file);
        let mut data: Vec<u8> = vec![];
        if let Err(_) = reader.read_to_end(&mut data).await {
            clean_test_read_settings(path).await;
            return;
        };
        let data = String::from_utf8(data).unwrap();
        let settings: Settings = ron::from_str(&data).unwrap();

        // Assert
        assert_eq!(settings, Settings::default());

        // Delete file
        delete_dummy_settings_file(path)
            .await
            .expect("Deletion failed");
    }
}
