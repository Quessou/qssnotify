use crate::errors::initialization_error::InitializationError;
use async_trait::async_trait;

#[async_trait]
pub trait Initializer {
    async fn initialize(&self) -> Result<(), InitializationError>;
}
