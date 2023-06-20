use ron;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

use super::paths;
use super::read;
use crate::data_objects::sentence::Sentence;

async fn write_data_in_writer(
    data: &Vec<Sentence>,
    writer: &mut tokio::io::BufWriter<impl tokio::io::AsyncWrite + std::marker::Unpin>,
) -> Result<(), std::io::Error> {
    let s = ron::to_string(data).expect("Big issue??");
    let _len_written = writer.write(s.as_bytes()).await?;
    writer.flush().await.unwrap();
    Ok(())
}

#[tracing::instrument(name = "Writing data in file")]
pub async fn write_data_file(
    path: &std::path::Path,
    data: Vec<Sentence>,
) -> Result<(), std::io::Error> {
    let file = tokio::fs::File::create(path).await?;
    let mut writer = tokio::io::BufWriter::new(file);
    tracing::info!("Data writing successful");
    write_data_in_writer(&data, &mut writer).await
}

pub async fn append_to_data_file(
    path: &std::path::Path,
    data: Sentence,
) -> Result<(), std::io::Error> {
    //let mut file = OpenOptions::new().read(true).open(path).await?;
    //let mut writer = tokio::io::BufWriter::new(&mut file);
    let mut sentences = read::read_data_file(&paths::get_data_file_path()).await?;
    sentences.push(data);
    write_data_file(path, sentences).await
}

#[cfg(test)]
mod tests {

    use tokio::io::AsyncReadExt;

    use super::*;

    #[tokio::test]
    async fn test_write_data_in_writer() {
        let toto = String::from("toto");
        let input_data = vec![Sentence::new(toto)];
        let mut buf = &mut std::io::Cursor::<Vec<u8>>::default();
        let mut writer = tokio::io::BufWriter::new(&mut buf);
        write_data_in_writer(&input_data, &mut writer)
            .await
            .expect("Writing failed");
        buf.set_position(0);

        // Retrieving back data that was written in the Cursor
        let mut reader = tokio::io::BufReader::new(std::io::Write::by_ref(&mut buf));
        let mut read_data: Vec<u8> = vec![];
        let _ = reader.read_to_end(&mut read_data).await;
        let read_data: String = String::from_utf8(read_data).unwrap();
        let read_data: Vec<Sentence> = ron::from_str(&read_data).unwrap();

        assert_eq!(input_data, read_data);
    }
}
