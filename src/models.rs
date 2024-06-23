use crate::schema::{discounts, orders, products, users};
use chrono::NaiveDateTime;
use diesel::{prelude::*, Selectable};
use serde_json::Value;
use serde::{Serialize, Deserialize};

#[derive(Insertable, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct UserModel {
    pub name: String,
    pub telegram_id: i64,
    pub phone_number: String,
}

#[derive(Insertable, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = products)]
pub struct ProductModel {
    pub model_id: String,
    pub price: f64,
    pub name: String,
    pub description: String,
    pub photo_ref_links: Vec<Option<String>>,
    pub characteristics: Option<Value>,
}

#[derive(Insertable, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(belongs_to(ProductModel))]
#[diesel(table_name = discounts)]
pub struct DiscountModel {
    pub id: i32,
    pub model: String,
    pub discount_in_percents: i16,
    pub starts_at: NaiveDateTime,
    pub ends_at: NaiveDateTime,
}

#[derive(Insertable, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(belongs_to(UserModel))]
#[diesel(table_name = orders)]
pub struct OrderModel {
    pub id: i64,
    pub customer: i64,
    pub ordered_at: NaiveDateTime,
}
