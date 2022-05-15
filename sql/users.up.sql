create table users (
    id integer primary key not null,

    -- ADMIN: Admin/full priviledges
    -- OFFICER: Officer/create new problems
    -- MEMBER: Normal user
    auth text not null,
    name text not null,
    username text unique not null,

    star_count integer default 0 not null,
    password char(65) not null,
    create_dt datetime not null default (datetime('now', 'localtime')),
    update_dt datetime not null default (datetime('now', 'localtime'))
);
