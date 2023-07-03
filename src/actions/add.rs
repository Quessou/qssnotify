use crate::{
    actions::utils::{open_file_in_editor, read_file_content},
    settings::Settings,
    traits::storage::Storage,
};

use super::utils;

pub async fn add_sentence(
    storage: &impl Storage,
    settings: &Settings,
) -> Result<(), std::io::Error> {
    let temp_file = utils::create_tmp_file();

    open_file_in_editor(&settings.editor, temp_file.path()).await?;
    let content = read_file_content(&temp_file).await?.trim().to_owned();
    tracing::info!("Sentence to add : {}", content);

    storage.save_sentence(content.into()).await
}
