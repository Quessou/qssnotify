use async_trait::async_trait;
use chrono::DateTime;

use crate::data_objects::sentence::Sentence;
use crate::filesystem::paths;
use crate::filesystem::{read, write};
use crate::traits::storage::Storage as StorageTrait;

pub struct Storage {}

#[async_trait]
impl StorageTrait for Storage {
    async fn get_all(&self) -> Result<Vec<Sentence>, std::io::Error> {
        read::read_data_file(&paths::get_data_file_path()).await
    }
    async fn save_sentence(&self, sentence: Sentence) -> Result<(), std::io::Error> {
        write::append_to_data_file(&paths::get_data_file_path(), sentence).await
    }
    async fn write_all(&self, sentences: Vec<Sentence>) -> Result<(), std::io::Error> {
        write::write_data_file(&paths::get_data_file_path(), sentences).await
    }
    async fn get_last_edition_time(
        &self,
    ) -> Result<DateTime<chrono::offset::Local>, std::io::Error> {
        let toto = tokio::fs::metadata(paths::get_data_file_path())
            .await
            .unwrap();
        Ok(chrono::DateTime::from(toto.modified().unwrap()))
    }
}
impl Default for Storage {
    fn default() -> Self {
        Self {}
    }
}
