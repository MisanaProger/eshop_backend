-- Your SQL goes here
ALTER TABLE users ADD constraint telegram_id UNIQUE (telegram_id);


 alter table default_orders drop column customer;
 alter table custom_orders DROP column customer;
 alter table default_orders ADD column customer bigint REFERENCES users(telegram_id) not null ;
 alter table custom_orders ADD column customer bigint REFERENCES users(telegram_id) not null ;

 alter table users DROP column id;
 ALTER TABLE users ADD Primary key (telegram_id);