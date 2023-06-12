use ron;
use tokio::io::{AsyncRead, AsyncReadExt, BufReader};

use crate::data_objects::sentence::Sentence;

// TODO : Return Result<Vec<Sentence>, std::io::Error> instead
pub async fn reader_to_data(
    mut reader: BufReader<impl AsyncRead + std::marker::Unpin>,
) -> Result<Vec<Sentence>, tokio::io::Error> {
    let mut data: Vec<u8> = vec![];
    let read_data_len = reader.read_to_end(&mut data).await;

    let read_data: String = String::from_utf8(data).unwrap();

    match read_data_len {
        Ok(0) => Ok(vec![]),
        Ok(_) => Ok(ron::from_str(&read_data).expect("Parsing failed")),
        Err(e) => Err(e),
    }
}

// TODO : Return Result<Vec<Sentence>, std::io::Error> instead
pub async fn read_data_file(path: &std::path::Path) -> Result<Vec<Sentence>, tokio::io::Error> {
    let f = tokio::fs::File::open(path)
        .await
        .expect("Opening file failed");
    let reader = BufReader::new(f);
    reader_to_data(reader).await
}

#[cfg(test)]
mod tests {

    use std::io::Read;
    use tokio::io::AsyncWriteExt;

    use super::*;

    #[tokio::test]
    pub async fn test_reader_to_data() {
        let toto = String::from("toto");
        let input: Vec<Sentence> = vec![Sentence::new(toto.clone()), Sentence::new(toto)];
        let mut bytes = std::io::Cursor::<Vec<u8>>::default();
        let mut writer = tokio::io::BufWriter::new(&mut bytes);
        let string_input = ron::to_string(&input).unwrap();
        writer.write_all(string_input.as_bytes()).await.unwrap();
        writer.flush().await.unwrap();
        bytes.set_position(0);

        let deserialized_data = reader_to_data(BufReader::new(Read::by_ref(&mut bytes))).await;
        assert_eq!(input, deserialized_data.unwrap());
    }
}
