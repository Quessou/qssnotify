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
    ServerCommunication,
}
/*
impl PartialEq for InitializationError {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::AlreadyInitialized, Self::AlreadyInitialized)
                | (Self::Filesystem(), Self::Filesystem(_))
                | (Self::ServerCommunication, Self::ServerCommunication)
        )
    }
}*/

impl PartialEq for InitializationError {
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }

    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Filesystem { source: l_source }, Self::Filesystem { source: r_source }) => true,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}
