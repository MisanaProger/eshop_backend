-- Your SQL goes here
drop table custom_orders;
drop table default_orders;

create table orders(
    id bigserial primary key,
    customer serial references users(telegram_id) not null,
    ordered_at timestamp not null 
);

create table ordered_default_products(
    id bigserial primary key,
    order_id serial references orders(id),
    model name references products(model_id) not null,
    characteristics json not null,
    price float not null
);

create table ordered_custom_products(
    id bigserial primary key,
    order_id serial references orders(id),
    refs_to_photos text[] not null,
    characteristics json not null,
    price float not null
);
