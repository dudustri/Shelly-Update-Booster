use async_trait::async_trait;

#[async_trait]
pub trait Storage: Send + Sync  {
    async fn store(&self, message: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}
