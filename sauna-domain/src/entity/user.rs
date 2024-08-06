use anyhow::{bail, Result};
use email_address::EmailAddress;
use uuid::Uuid;

const NAME_LENGTH_MAX: usize = 255;
const NAME_LENGTH_MIN: usize = 1;

const PHONE_NUMBER_DIGIT_TEN: usize = 10;
const PHONE_NUMBER_DIGIT_ELEVEN: usize = 11;

#[derive(Debug, PartialEq)]
pub struct User {
    id: String,
    email: String,
    phone_number: String,
    last_name: String,
    first_name: String,
    address: Address,
}

impl User {
    pub fn new(
        email: String,
        phone_number: String,
        last_name: String,
        first_name: String,
        prefecture: String,
        city: String,
        address_extra: String,
    ) -> Result<Self> {
        Self::new_with_id(
            Uuid::new_v4().to_string(),
            email,
            phone_number,
            last_name,
            first_name,
            prefecture,
            city,
            address_extra,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_with_id(
        id: String,
        email: String,
        phone_number: String,
        last_name: String,
        first_name: String,
        prefecture: String,
        city: String,
        address_extra: String,
    ) -> Result<Self> {
        if !(NAME_LENGTH_MIN..=NAME_LENGTH_MAX).contains(&last_name.chars().count()) {
            bail!("名前（姓）の値が不正です。");
        }

        if !(NAME_LENGTH_MIN..=NAME_LENGTH_MAX).contains(&first_name.chars().count()) {
            bail!("名前（名）の値が不正です。");
        }

        if !EmailAddress::is_valid(&email) {
            bail!("メールアドレスの値が不正です。");
        }

        let phone_number = phone_number.replace('-', "");

        if phone_number.len() != PHONE_NUMBER_DIGIT_TEN
            && phone_number.len() != PHONE_NUMBER_DIGIT_ELEVEN
        {
            bail!("電話番号の値が不正です。");
        }

        Ok(Self {
            id,
            email,
            phone_number,
            last_name,
            first_name,
            address: Address::new(prefecture, city, address_extra)?,
        })
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn email(&self) -> String {
        self.email.clone()
    }

    pub fn phone_number(&self) -> String {
        self.phone_number.clone()
    }

    pub fn last_name(&self) -> String {
        self.last_name.clone()
    }

    pub fn first_name(&self) -> String {
        self.first_name.clone()
    }

    pub fn prefecture(&self) -> String {
        self.address.prefecture.clone()
    }

    pub fn city(&self) -> String {
        self.address.city.clone()
    }

    pub fn address_extra(&self) -> String {
        self.address.extra.clone()
    }
}

#[derive(Debug, PartialEq)]
struct Address {
    prefecture: String,
    city: String,
    extra: String,
}

impl Address {
    fn new(prefecture: String, city: String, extra: String) -> Result<Self> {
        if prefecture.is_empty() || city.is_empty() || extra.is_empty() {
            bail!("住所の値が不正です。");
        }

        Ok(Self {
            prefecture,
            city,
            extra,
        })
    }
}
