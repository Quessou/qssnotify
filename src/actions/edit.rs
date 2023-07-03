use crate::settings::Settings;
use crate::{data_objects::sentence::Sentence, traits::storage::Storage};

use super::utils::{self, open_file_in_editor, read_file_content, write_in_tmp_file};

pub async fn edit_sentence(
    storage: &impl Storage,
    hash: u64,
    settings: &Settings,
) -> Result<(), std::io::Error> {
    let mut sentences: Vec<Sentence> = storage.get_all().await?;
    let edited_sentence_index = sentences
        .iter()
        .position(|s| s.hash() == hash)
        .expect("Could not find hash");
    let sentence_to_edit = sentences.remove(edited_sentence_index);

    let temp_file = utils::create_tmp_file();
    let temp_file_path = temp_file.path();

    write_in_tmp_file(&temp_file, &sentence_to_edit)
        .await
        .expect("Writing to temporary file failed");
    open_file_in_editor(&settings.editor, temp_file_path).await?;
    let sentence: Sentence = read_file_content(&temp_file).await.unwrap().into();
    sentences.push(sentence);
    //write_data_file(&paths::get_data_file_path(), sentences).await
    storage.write_all(sentences).await
}
