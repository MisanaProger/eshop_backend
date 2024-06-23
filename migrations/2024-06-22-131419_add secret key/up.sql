create table admins(
    telegram_id bigint primary key,
    username text not null,
    password text not null,
    token uuid not null
);

