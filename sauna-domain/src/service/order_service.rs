use crate::{
    entity::{
        cart::Cart,
        order::{calc_total_amount, Order, OrderProduct},
    },
    repository::{order_repository::OrderRepository, product_repository::ProductRepository},
};
use anyhow::{Context, Result};
use chrono::{DateTime, Local};

pub struct OrderService<O, P> {
    order_repository: O,
    product_repository: P,
}

impl<O, P> OrderService<O, P>
where
    O: OrderRepository,
    P: ProductRepository,
{
    pub fn new(order_repository: O, product_repository: P) -> Self {
        Self {
            order_repository,
            product_repository,
        }
    }

    pub async fn order_product(&self, cart: &Cart, now: DateTime<Local>) -> Result<String> {
        let products = self
            .product_repository
            .find_by_ids(cart.product_ids())
            .await?;

        let mut order_products = vec![];

        for cart_product in cart.products() {
            let mut product = products
                .iter()
                .find(|product| product.id() == cart_product.product_id())
                .context("商品が見つかりません。")?
                .clone();

            order_products.push(OrderProduct::new(
                cart_product.product_id(),
                product.price(),
                cart_product.quantity(),
            )?);

            product.consume(cart_product.quantity())?;

            self.product_repository.save(&product).await?;
        }

        let order = Order::new(
            cart.user_id(),
            calc_total_amount(&order_products),
            order_products,
            now,
        )?;

        self.order_repository.save(&order).await?;

        Ok(order.id())
    }
}
