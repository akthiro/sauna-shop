use crate::entity::order::Order;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait OrderRepository: Send + Sync + 'static {
    async fn save(&self, order: &Order) -> Result<()>;
}
