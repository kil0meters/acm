create table users (
    id integer primary key not null,

    -- ADMIN: Admin/full priviledges
    -- OFFICER: Officer/create new problems
    -- MEMBER: Normal user
    auth text not null default "MEMBER",
    name text not null,
    username text unique not null,
    discord_id text not null,

    create_dt datetime not null default (datetime('now', 'localtime')),
    update_dt datetime not null default (datetime('now', 'localtime'))
);
