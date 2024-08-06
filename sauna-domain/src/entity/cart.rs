use anyhow::{anyhow, bail, Result};
use uuid::Uuid;

#[derive(Debug, PartialEq, Clone)]
pub struct CartProduct {
    product_id: String,
    quantity: usize,
}

impl CartProduct {
    pub fn new(product_id: String, quantity: usize) -> Result<Self> {
        if Uuid::try_parse(&product_id).is_err() {
            bail!("商品IDの値が不正です。");
        }

        if quantity < 1 {
            bail!("購入数の値が不正です。");
        }

        Ok(Self {
            product_id,
            quantity,
        })
    }

    pub fn product_id(&self) -> String {
        self.product_id.clone()
    }

    pub fn quantity(&self) -> usize {
        self.quantity
    }
}

#[derive(Debug, PartialEq)]
pub struct Cart {
    user_id: String,
    products: Vec<CartProduct>,
}

impl Cart {
    pub fn new(user_id: String) -> Result<Self> {
        if Uuid::try_parse(&user_id).is_err() {
            bail!("ユーザーIDの値が不正です。");
        }

        Ok(Self {
            user_id,
            products: vec![],
        })
    }

    pub fn user_id(&self) -> String {
        self.user_id.clone()
    }

    pub fn products(&self) -> Vec<CartProduct> {
        self.products.clone()
    }

    pub fn product_ids(&self) -> Vec<String> {
        self.products
            .iter()
            .map(|product| product.product_id())
            .collect()
    }

    pub fn quantity_by_product_id(&self, product_id: String) -> Result<usize> {
        self.products
            .iter()
            .find(|product| product.product_id == product_id)
            .map(|product| product.quantity)
            .ok_or_else(|| anyhow!("カートの商品が見つかりません。"))
    }

    pub fn add_products(&mut self, product_id: String, quantity: usize) -> Result<()> {
        if let Some(product) = self
            .products
            .iter_mut()
            .find(|product| product.product_id == product_id)
        {
            product.quantity = quantity;
        } else {
            self.products.push(CartProduct::new(product_id, quantity)?);
        }

        Ok(())
    }

    pub fn remove_product(&mut self, product_id: String) -> Result<()> {
        if let Some(position) = self
            .products
            .iter()
            .position(|product| product.product_id == product_id)
        {
            self.products.remove(position);
        }

        Ok(())
    }
}
