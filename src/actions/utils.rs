use tempfile::NamedTempFile;
use tokio::io::AsyncReadExt;

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
