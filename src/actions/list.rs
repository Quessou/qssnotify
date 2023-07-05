use crate::data_objects::sentence::Sentence;
use crate::traits::displayer::Displayer;
use crate::traits::storage::Storage;

// TODO : Remove me / move me into Displayer ?
pub async fn list_sentences(
    storage: &impl Storage,
    displayer: &impl Displayer<Sentence>,
) -> Result<(), std::io::Error> {
    let sentences = storage.get_all().await?;
    displayer.display_vec(sentences).await;
    Ok(())
}
