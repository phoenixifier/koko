-- Your SQL goes here
create table users (
    id int primary key,
    lang varchar not null default 'en',
    admin boolean not null default false
)