use crate::data_objects::sentence::Sentence;
use crate::traits::notifier::Notifier;

pub struct OsNotifier {}

impl Notifier for OsNotifier {
    fn notify(&self, sentence: Sentence) -> Result<(), std::io::Error> {
        match notify_rust::Notification::new()
            .summary("Read this")
            .body(sentence.data())
            .appname("qssnotify")
            .show()
        {
            Ok(_) => Ok(()),
            Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
        }
    }
}

impl Default for OsNotifier {
    fn default() -> Self {
        Self {}
    }
}
