use std::path::Path;

use tempfile::NamedTempFile;

use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::data_objects::sentence::Sentence;

pub fn create_tmp_file() -> NamedTempFile {
    let temp_file = NamedTempFile::new();
    if temp_file.is_err() {
        tracing::error!("Could not create temporary file");
        panic!()
    }
    temp_file.unwrap()
}

/// Writes the [`sentence`] into [`tmp_file`], called generally before [`open_file_in_editor`]
pub async fn write_in_tmp_file(
    tmp_file: &NamedTempFile,
    sentence: &Sentence,
) -> Result<(), std::io::Error> {
    let mut writer = {
        let f = tokio::fs::File::from_std(tmp_file.reopen().unwrap());
        tokio::io::BufWriter::new(f)
    };
    let r = writer.write(sentence.data().as_bytes()).await;
    if let Err(e) = r {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
    }
    // TODO : Find why I do not have anything in vim when I write in the file here
    tracing::info!("Number of bytes written : {}", r.unwrap());
    writer.flush().await.expect("Flushing failed");
    Ok(())
}

/// Opens the file whose path is given in parameter into a CLI editor (nvim, nano...), so that they
/// can edit it
pub async fn open_file_in_editor(editor: &str, file_path: &Path) -> Result<(), std::io::Error> {
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

/// Read the content of a temporary file
/// Mostly used to read files where user just wrote to
///
/// # TODO
/// Async or not async ?
/// Ensure that the file is closed when entering the function
pub async fn read_file_content(file: &NamedTempFile) -> Result<String, std::io::Error> {
    let mut buf = String::default();
    let async_file = tokio::fs::File::from_std(file.reopen().unwrap());
    let mut reader = tokio::io::BufReader::new(async_file);
    let r = reader.read_to_string(&mut buf).await;
    r?;
    Ok(buf.trim().to_owned())
}

#[cfg(test)]
mod tests {
    use std::io::Seek;

    use super::*;

    #[tokio::test]
    pub async fn test_write_in_tmp_file() {
        let s = String::from("toto");
        let sentence: Sentence = s.clone().into();
        let mut tmp_file = NamedTempFile::new().unwrap();
        let r = write_in_tmp_file(&tmp_file, &sentence).await;
        assert!(r.is_ok());
        tmp_file.as_file_mut().rewind().unwrap();
        let content = read_file_content(&tmp_file).await.unwrap();
        assert_eq!(s, content);
    }
}
