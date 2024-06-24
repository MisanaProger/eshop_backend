use crate::models::DiscountModel;
use crate::{models::ProductModel, DBConnection};
use chrono::Utc;
use diesel::prelude::*;
use diesel::{result::Error, QueryDsl, SelectableHelper};

impl ProductModel {
    pub fn from_model_id(
        model_id: impl ToString,
        connection: &mut DBConnection,
    ) -> Result<Self, Error> {
        let model_id = model_id.to_string();
        use crate::schema::products::dsl::products;
        products
            .find(model_id)
            .select(ProductModel::as_select())
            .first(connection)
    }

    pub fn discount_in_percents(&self, connection: &mut DBConnection) -> f64 {
        use crate::schema::discounts::dsl::*;

        discounts
            .filter(model.eq(self.model_id.clone()))
            .filter(starts_at.gt(Utc::now().naive_local()))
            .filter(ends_at.lt(Utc::now().naive_local()))
            .select(DiscountModel::as_select())
            .first(connection)
            .unwrap()
            .discount_in_percents
            .into()
    }

    pub fn default_price(&self, connection: &mut DBConnection) -> f64 {
        use crate::schema::products::dsl::*;
        products
            .find(&self.model_id)
            .select(ProductModel::as_select())
            .first(connection)
            .unwrap()
            .price
    }

    pub fn current_price(&self, connection: &mut DBConnection) -> f64 {
        (1. - self.discount_in_percents(connection) / 100.) * self.default_price(connection)
    }
}
