use crate::{
    actions::utils::{open_file_in_editor, read_file_content},
    settings::Settings,
};

use super::utils;

pub async fn add_sentence(settings: &Settings) -> Result<(), std::io::Error> {
    let temp_file = utils::create_tmp_file();

    open_file_in_editor(&settings.editor, temp_file.path()).await?;
    let content = read_file_content(&temp_file).await?.trim().to_owned();
    tracing::info!("Sentence to add : {}", content);

    utils::append_sentence_to_data_file(content.into()).await
}
