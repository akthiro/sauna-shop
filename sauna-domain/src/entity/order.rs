use anyhow::{bail, Result};
use chrono::{DateTime, Local};
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub struct Order {
    id: String,
    user_id: String,
    total_amount: usize,
    products: Vec<OrderProduct>,
    ordered_at: DateTime<Local>,
}

impl Order {
    pub fn new(
        user_id: String,
        total_amount: usize,
        products: Vec<OrderProduct>,
        ordered_at: DateTime<Local>,
    ) -> Result<Self> {
        Self::new_with_id(
            Uuid::new_v4().to_string(),
            user_id,
            total_amount,
            products,
            ordered_at,
        )
    }

    pub fn new_with_id(
        id: String,
        user_id: String,
        total_amount: usize,
        products: Vec<OrderProduct>,
        ordered_at: DateTime<Local>,
    ) -> Result<Self> {
        if Uuid::try_parse(&user_id).is_err() {
            bail!("ユーザーIDの値が不正です。");
        }

        if products.is_empty() {
            bail!("購入商品がありません。");
        }

        Ok(Self {
            id,
            user_id,
            total_amount,
            products,
            ordered_at,
        })
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn user_id(&self) -> String {
        self.user_id.clone()
    }

    pub fn total_amount(&self) -> usize {
        self.total_amount
    }

    pub fn products(&self) -> Vec<OrderProduct> {
        self.products.clone()
    }

    pub fn ordered_at(&self) -> DateTime<Local> {
        self.ordered_at
    }

    pub fn product_ids(&self) -> Vec<String> {
        self.products
            .iter()
            .map(|product| product.product_id())
            .collect()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct OrderProduct {
    product_id: String,
    price: usize,
    quantity: usize,
}

impl OrderProduct {
    pub fn new(product_id: String, price: usize, quantity: usize) -> Result<Self> {
        if Uuid::try_parse(&product_id).is_err() {
            bail!("商品IDの値が不正です。");
        }

        if quantity < 1 {
            bail!("購入数の値が不正です。");
        }

        Ok(Self {
            product_id,
            price,
            quantity,
        })
    }

    pub fn product_id(&self) -> String {
        self.product_id.clone()
    }

    pub fn price(&self) -> usize {
        self.price
    }

    pub fn quantity(&self) -> usize {
        self.quantity
    }
}

pub fn calc_total_amount(products: &[OrderProduct]) -> usize {
    products.iter().map(|product| product.price()).sum()
}
