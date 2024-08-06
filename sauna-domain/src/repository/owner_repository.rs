use crate::entity::owner::Owner;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait OwnerRepository: Send + Sync + 'static {
    async fn save(&self) -> Result<()>;
    async fn find_by_id(&self, id: String) -> Result<Owner>;
}
