use crate::schema::{
    discounts, ordered_custom_products, ordered_default_products, orders, products, users,
};
use chrono::NaiveDateTime;
use diesel::{prelude::*, Selectable};
use serde::{Deserialize, Serialize};
use serde_json::Value;

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

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(belongs_to(UserModel))]
#[diesel(table_name = orders)]
pub struct OrderModel {
    pub id: i64,
    pub customer: i64,
    pub ordered_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(belongs_to(UserModel))]
#[diesel(table_name = orders)]
pub struct CreateOrderModel {
    pub customer: i64,
}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(belongs_to(OrderModel))]
#[diesel(table_name = ordered_custom_products)]
pub struct OrderedCustomProductModel {
    pub id: i64,
    pub refs_to_photos: Vec<String>,
    pub characteristics: Value,
    pub price: f64,
    pub order_id: i64,
}

#[derive(Insertable)]
#[diesel(belongs_to(OrderModel))]
#[diesel(table_name = ordered_custom_products)]
pub struct CreateOrderedCustomProductModel {
    pub refs_to_photos: Vec<String>,
    pub characteristics: Value,
    pub price: f64,
    pub order_id: i64,
}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(belongs_to(OrderModel))]
#[diesel(table_name = ordered_default_products)]
pub struct OrderedDefaultProductModel {
    pub id: i64,
    pub model: String,
    pub characteristics: Value,
    pub price_on_order: f64,
    pub order_id: i64,
}

#[derive(Insertable)]
#[diesel(belongs_to(OrderModel))]
#[diesel(table_name = ordered_default_products)]
pub struct CreateOrderedDefaultProductModel {
    pub model: String,
    pub characteristics: Value,
    pub price_on_order: f64,
    pub order_id: i64,
}
