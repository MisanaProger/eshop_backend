-- Your SQL goes here
ALTER TABLE users
ALTER COLUMN name 
TYPE text;

ALTER TABLE products
ALTER  name 
TYPE text,
ALTER COLUMN model_id 
TYPE text;

ALTER TABLE discounts
ALTER COLUMN model
TYPE text;

ALTER TABLE default_orders
ALTER COLUMN model
TYPE text;



