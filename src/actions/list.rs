use crate::filesystem::paths;
use crate::filesystem::read;

pub async fn list_sentences() -> Result<(), std::io::Error> {
    let sentences = read::read_data_file(&paths::get_data_file_path()).await?;
    for s in sentences {
        println!("{}", s);
    }
    Ok(())
}
