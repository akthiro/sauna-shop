use crate::entity::user::User;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn save(&self, user: &User) -> Result<()>;
    async fn find_by_id(&self, id: String) -> Result<User>;
    async fn find_all(&self) -> Result<Vec<User>>;
}
