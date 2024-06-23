-- Your SQL goes here
ALTER TABLE ordered_default_products
drop COLUMN order_id;

ALTER TABLE ordered_custom_products
drop COLUMN order_id;

alter table ordered_custom_products add column order_id bigserial references orders(id) not null;
alter table ordered_default_products add column order_id bigserial references orders(id) not null;

ALTER TABLE ordered_default_products RENAME COLUMN price TO price_on_order;