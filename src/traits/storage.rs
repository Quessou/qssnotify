use async_trait::async_trait;

use chrono::DateTime;

use crate::data_objects::sentence::Sentence;

#[async_trait]
pub trait Storage {
    async fn get_all(&self) -> Result<Vec<Sentence>, std::io::Error>;
    async fn save_sentence(&self, sentence: Sentence) -> Result<(), std::io::Error>;
    async fn write_all(&self, sentences: Vec<Sentence>) -> Result<(), std::io::Error>;
    async fn get_last_edition_time(
        &self,
    ) -> Result<DateTime<chrono::offset::Local>, std::io::Error>;
}
