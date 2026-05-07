use async_trait::async_trait;
use crate::rxtx::AppError;
#[async_trait]
trait Job
{
    async fn execute(self) -> Result<String, AppError>;
}