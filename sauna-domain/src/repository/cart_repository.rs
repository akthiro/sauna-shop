use crate::entity::cart::Cart;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait CartRepository: Send + Sync + 'static {
    async fn find_by_user_id(&self, user_id: String) -> Result<Cart>;
    async fn save(&self, cart: &Cart) -> Result<()>;
}
