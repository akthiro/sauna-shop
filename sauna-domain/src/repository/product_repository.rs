use crate::entity::product::Product;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait ProductRepository: Send + Sync + 'static {
    async fn save(&self, product: &Product) -> Result<()>;
    async fn find_by_id(&self, id: String) -> Result<Product>;
    async fn find_by_ids(&self, ids: Vec<String>) -> Result<Vec<Product>>;
}
