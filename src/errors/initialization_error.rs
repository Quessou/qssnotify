use thiserror::Error;

#[derive(Error, Debug)]
pub enum InitializationError {
    #[error("Already initialized")]
    AlreadyInitialized,
    #[error("Could not initialize filesystem")]
    Filesystem {
        #[from]
        source: std::io::Error,
    },
    #[error("Could not reach server")]
    _ServerCommunication,
}

impl PartialEq for InitializationError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Filesystem { source: _ }, Self::Filesystem { source: _ }) => true,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}
