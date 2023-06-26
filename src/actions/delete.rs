use crate::filesystem::paths;
use crate::filesystem::read::read_data_file;
use crate::filesystem::write::write_data_file;

pub async fn delete_sentence(hash: u64) -> Result<(), std::io::Error> {
    let sentences = read_data_file(&paths::get_data_file_path()).await?;
    let sentences = sentences
        .into_iter()
        .filter(|s| s.hash() != hash)
        .collect::<Vec<_>>();
    write_data_file(&paths::get_data_file_path(), sentences).await?;

    Ok(())
}
