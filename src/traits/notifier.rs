use crate::data_objects::sentence::Sentence;

pub trait Notifier {
    fn notify(&self, sentence: Sentence) -> Result<(), std::io::Error>;
}
