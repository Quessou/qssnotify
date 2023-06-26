use crate::data_objects::sentence::Sentence;
use crate::filesystem::{paths, read};
use rand::prelude::*;

pub async fn get_random() -> Result<Option<Sentence>, std::io::Error> {
    let index: usize = rand::thread_rng().gen();
    let sentences = read::read_data_file(&paths::get_data_file_path()).await?;
    if sentences.is_empty() {
        return Ok(None);
    }
    let sentence = sentences[index % sentences.len()].clone();
    Ok(Some(sentence))
}
pub async fn get(hash: u64) -> Result<Option<Sentence>, std::io::Error> {
    let sentences = read::read_data_file(&paths::get_data_file_path()).await?;
    if sentences.is_empty() {
        return Ok(None);
    }
    let sentence = sentences.into_iter().find(|s| s.hash() == hash);
    if sentence.is_none() {
        return Ok(None);
    }
    Ok(sentence)
}
