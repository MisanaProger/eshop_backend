-- Your SQL goes here
CREATE TABLE users(
    id serial primary key,
    name name not null,
    telegram_id bigint not null,
    phone_number varchar(9) not null
);

CREATE TABLE products(
    model_id name primary key not null,
    price float not null,
    name name not null,
    description text not null,
    photo_ref_links text[] not null,
    characteristics json
);

CREATE TABLE discounts(
    id serial primary key,
    model name references products(model_id) not null,
    discount_in_ercents smallint not null,
    starts_at timestamp not null,
    ends_at timestamp not null 
);

CREATE TABLE default_orders(
    id serial primary key,
    customer serial references users(id) not null,
    model name references products(model_id) not null,
    price float not null,
    ordered_at timestamp not null 
);

CREATE TABLE custom_orders(
    id serial primary key,
    customer serial references users(id) not null,
    refs_to_photos text[] not null,
    characteristics json not null,
    price float not null,
    ordered_at timestamp not null 
);