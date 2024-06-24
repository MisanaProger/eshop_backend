use crate::models::{
    CreateOrderModel, CreateOrderedCustomProductModel, CreateOrderedDefaultProductModel,
    OrderModel, OrderedCustomProductModel, OrderedDefaultProductModel, ProductModel,
};
use chrono::NaiveDateTime;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
    result::Error,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub enum OrderedProduct {
    Default {
        id: i64,
        model: String,
        characteristics: Value,
        price_on_order: f64,
    },
    Custom {
        id: i64,
        refs_to_photos: Vec<String>,
        characteristics: Value,
        price: f64,
    },
}

impl From<OrderedDefaultProductModel> for OrderedProduct {
    fn from(value: OrderedDefaultProductModel) -> Self {
        OrderedProduct::Default {
            id: value.id,
            model: value.model,
            characteristics: value.characteristics,
            price_on_order: value.price_on_order,
        }
    }
}

impl From<OrderedCustomProductModel> for OrderedProduct {
    fn from(value: OrderedCustomProductModel) -> Self {
        OrderedProduct::Custom {
            id: value.id,
            refs_to_photos: value.refs_to_photos,
            characteristics: value.characteristics,
            price: value.price,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Order {
    id: i64,
    customer_telegram_id: i64,
    ordered_at: NaiveDateTime,
    products: OrderedProduct,
}

#[derive(Serialize, Deserialize)]
pub struct OrderDTO {
    pub customer_telegram_id: i64,
    pub products: Vec<OrderedProductDTO>,
}

#[derive(Serialize, Deserialize)]
pub enum OrderedProductDTO {
    Default {
        model: String,
        characteristics: Value,
    },
    Custom {
        refs_to_photos: Vec<String>,
        characteristics: Value,
        price: f64,
    },
}

impl OrderDTO {
    pub fn store(
        self,
        connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    ) -> Result<OrderModel, Error> {
        use crate::schema::orders::dsl::*;
        let inserted_order = diesel::insert_into(orders)
            .values(CreateOrderModel {
                customer: self.customer_telegram_id,
            })
            .get_result::<OrderModel>(connection)?;
        for product in self.products {
            product.store(connection, inserted_order.id)?;
        }

        Ok(inserted_order)
    }
}

impl OrderedProductDTO {
    pub fn store(
        self,
        connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
        order_id: i64,
    ) -> Result<usize, Error> {
        use crate::schema::ordered_custom_products::dsl::ordered_custom_products;
        use crate::schema::ordered_default_products::dsl::ordered_default_products;
        match self {
            OrderedProductDTO::Default {
                ref model,
                ref characteristics,
            } => diesel::insert_into(ordered_default_products)
                .values(CreateOrderedDefaultProductModel {
                    model: model.clone(),
                    characteristics: characteristics.clone(),
                    price_on_order: ProductModel::from_model_id(model.clone(), connection)?
                        .current_price(connection),
                    order_id,
                })
                .execute(connection),
            OrderedProductDTO::Custom {
                refs_to_photos,
                characteristics,
                price,
            } => diesel::insert_into(ordered_custom_products)
                .values(CreateOrderedCustomProductModel {
                    refs_to_photos: refs_to_photos.clone(),
                    characteristics: characteristics.clone(),
                    price,
                    order_id,
                })
                .execute(connection),
        }
    }
}
