use anyhow::{bail, Result};
use email_address::EmailAddress;
use uuid::Uuid;

const NAME_LENGTH_MAX: usize = 255;
const NAME_LENGTH_MIN: usize = 1;

#[derive(Debug, PartialEq)]
pub struct Owner {
    id: String,
    name: String,
    email: String,
}

impl Owner {
    pub fn new(name: String, email: String) -> Result<Self> {
        Self::new_with_id(Uuid::new_v4().to_string(), name, email)
    }

    pub fn new_with_id(id: String, name: String, email: String) -> Result<Self> {
        if !(NAME_LENGTH_MIN..=NAME_LENGTH_MAX).contains(&name.chars().count()) {
            bail!("名前の値が不正です。");
        }

        if !EmailAddress::is_valid(&email) {
            bail!("メールアドレスの値が不正です。");
        }

        Ok(Self { id, name, email })
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn email(&self) -> String {
        self.email.clone()
    }
}
