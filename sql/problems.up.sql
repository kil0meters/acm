create table problems (
    id integer primary key not null,

    title text unique not null,
    description text not null,
    runner text not null,
    template text not null,
    visible boolean not null,

    create_dt datetime not null default (datetime('now', 'localtime')),
    update_dt datetime not null default (datetime('now', 'localtime'))
);
