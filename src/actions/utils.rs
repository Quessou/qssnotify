use tempfile::NamedTempFile;
use tokio::io::AsyncReadExt;

use crate::data_objects::sentence::Sentence;
use crate::filesystem::paths;
use crate::filesystem::write;

pub fn create_tmp_file() -> NamedTempFile {
    let temp_file = NamedTempFile::new();
    if temp_file.is_err() {
        tracing::error!("Could not create temporary file");
        panic!()
    }
    temp_file.unwrap()
}

pub fn open_file_in_editor(editor: &str, file_path: &str) -> Result<(), std::io::Error> {
    let mut command = std::process::Command::new(editor);
    let subprocess = command.arg(file_path);
    let r = subprocess.spawn();
    if let Err(e) = r {
        tracing::error!("Could not launch editor : {}", editor);
        return Err(e);
    };

    let s = r.unwrap().wait();
    match s {
        Ok(_) => Ok(()),
        Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
    }
}

pub async fn read_file_content(file_path: &str) -> Result<String, std::io::Error> {
    let async_file = tokio::fs::File::open(file_path).await.unwrap();
    let mut buf = String::default();
    let mut reader = tokio::io::BufReader::new(async_file);
    reader.read_to_string(&mut buf).await?;

    Ok(buf)
}

pub async fn append_sentence_to_data_file(sentence: Sentence) -> Result<(), std::io::Error> {
    write::append_to_data_file(&paths::get_data_file_path(), sentence).await
}
