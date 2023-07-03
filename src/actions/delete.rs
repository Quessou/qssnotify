use crate::traits::storage::Storage;

pub async fn delete_sentence(storage: &impl Storage, hash: u64) -> Result<(), std::io::Error> {
    let sentences = storage.get_all().await?;
    let sentences = sentences
        .into_iter()
        .filter(|s| s.hash() != hash)
        .collect::<Vec<_>>();
    storage.write_all(sentences).await?;

    Ok(())
}
