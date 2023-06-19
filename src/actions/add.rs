use crate::{
    actions::utils::{open_file_in_editor, read_file_content},
    settings::Settings,
};

use super::utils;

pub async fn add_sentence(settings: &Settings) -> Result<(), std::io::Error> {
    let temp_file = utils::create_tmp_file(); //tempfile::NamedTempFile::new();

    open_file_in_editor(&settings.editor, temp_file.path().to_str().unwrap())?;
    let content = read_file_content(temp_file.path().to_str().unwrap()).await?;
    println!("Sentence : {}", content);

    // TODO : Save the sentence somewhere
    Ok(())
}
