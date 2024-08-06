use anyhow::{bail, Result};
use uuid::Uuid;

const NAME_LENGTH_MAX: usize = 100;
const NAME_LENGTH_MIN: usize = 1;

const DESCRIPTION_LENGTH_MAX: usize = 1000;
const DESCRIPTION_LENGTH_MIN: usize = 1;

#[derive(Debug, PartialEq, Clone)]
pub struct Product {
    id: String,
    owner_id: String,
    name: String,
    description: String,
    price: usize,
    stock: usize,
}

impl Product {
    pub fn new(
        owner_id: String,
        name: String,
        description: String,
        price: usize,
        stock: usize,
    ) -> Result<Self> {
        Self::new_with_id(
            Uuid::new_v4().to_string(),
            owner_id,
            name,
            description,
            price,
            stock,
        )
    }

    pub fn new_with_id(
        id: String,
        owner_id: String,
        name: String,
        description: String,
        price: usize,
        stock: usize,
    ) -> Result<Self> {
        if Uuid::try_parse(&id).is_err() {
            bail!("オーナーIDの値が不正です。");
        }

        if !(NAME_LENGTH_MIN..=NAME_LENGTH_MAX).contains(&name.chars().count()) {
            bail!("商品名の値が不正です。");
        }

        if !(DESCRIPTION_LENGTH_MIN..=DESCRIPTION_LENGTH_MAX).contains(&description.chars().count())
        {
            bail!("商品説明の値が不正です。");
        }

        if price < 1 {
            bail!("価格の値が不正です。");
        }

        Ok(Self {
            id,
            owner_id,
            name,
            description,
            price,
            stock,
        })
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn owner_id(&self) -> String {
        self.owner_id.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }

    pub fn price(&self) -> usize {
        self.price
    }

    pub fn stock(&self) -> usize {
        self.stock
    }

    pub fn consume(&mut self, quantity: usize) -> Result<()> {
        if self.stock < quantity {
            bail!("在庫数が不足しています。");
        }

        self.stock -= quantity;

        Ok(())
    }
}
