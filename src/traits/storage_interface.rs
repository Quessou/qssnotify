use async_trait::async_trait;

use crate::data_objects::sentence::Sentence;

#[async_trait]
pub trait StorageInterface {
    async fn get_random(&self) -> Sentence;
    async fn get_all(&self) -> Vec<Sentence>;
}
