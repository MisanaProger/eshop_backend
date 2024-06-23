-- This file should undo anything in `up.sql`
alter table users drop column activated bool not null default false;