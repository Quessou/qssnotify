use crate::data_objects::sentence::Sentence;
use rand::prelude::*;

use crate::traits::storage::Storage;

/// Reads the entire data file and returns a sentence if able
/// Kinda useless actually, mostly here for testing purposes
pub async fn get_random(storage: &impl Storage) -> Result<Option<Sentence>, std::io::Error> {
    let index: usize = rand::thread_rng().gen();
    //let sentences = read::read_data_file(&paths::get_data_file_path()).await?;
    let sentences = storage.get_all().await?;
    if sentences.is_empty() {
        return Ok(None);
    }
    let sentence = sentences[index % sentences.len()].clone();
    Ok(Some(sentence))
}

pub async fn get(storage: &impl Storage, hash: u64) -> Result<Option<Sentence>, std::io::Error> {
    let sentences = storage.get_all().await?;
    if sentences.is_empty() {
        return Ok(None);
    }
    let sentence = sentences.into_iter().find(|s| s.hash() == hash);
    if sentence.is_none() {
        return Ok(None);
    }
    Ok(sentence)
}
