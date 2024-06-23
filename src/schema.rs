// @generated automatically by Diesel CLI.

diesel::table! {
    admins (telegram_id) {
        telegram_id -> Int8,
        username -> Text,
        password -> Text,
        token -> Uuid,
    }
}

diesel::table! {
    discounts (id) {
        id -> Int4,
        model -> Text,
        discount_in_percents -> Int2,
        starts_at -> Timestamp,
        ends_at -> Timestamp,
    }
}

diesel::table! {
    ordered_custom_products (id) {
        id -> Int8,
        refs_to_photos -> Array<Nullable<Text>>,
        characteristics -> Json,
        price -> Float8,
        order_id -> Int8,
    }
}

diesel::table! {
    ordered_default_products (id) {
        id -> Int8,
        model -> Text,
        characteristics -> Json,
        price_on_order -> Float8,
        order_id -> Int8,
    }
}

diesel::table! {
    orders (id) {
        id -> Int8,
        customer -> Int8,
        ordered_at -> Timestamp,
    }
}

diesel::table! {
    products (model_id) {
        model_id -> Text,
        price -> Float8,
        name -> Text,
        description -> Text,
        photo_ref_links -> Array<Nullable<Text>>,
        characteristics -> Nullable<Json>,
    }
}

diesel::table! {
    secrets (key) {
        #[max_length = 16]
        key -> Varchar,
    }
}

diesel::table! {
    users (telegram_id) {
        name -> Text,
        telegram_id -> Int8,
        #[max_length = 9]
        phone_number -> Varchar,
        activated -> Bool,
    }
}

diesel::joinable!(discounts -> products (model));
diesel::joinable!(ordered_custom_products -> orders (order_id));
diesel::joinable!(ordered_default_products -> orders (order_id));
diesel::joinable!(ordered_default_products -> products (model));
diesel::joinable!(orders -> users (customer));

diesel::allow_tables_to_appear_in_same_query!(
    admins,
    discounts,
    ordered_custom_products,
    ordered_default_products,
    orders,
    products,
    secrets,
    users,
);
