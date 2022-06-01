create table problems (
    id integer primary key not null,
    activity_id integer,

    title text unique not null,
    description text not null,
    runner text not null,
    reference text not null,
    template text not null,

    visible boolean not null default true,

    create_dt datetime not null default (datetime('now', 'localtime')),
    update_dt datetime not null default (datetime('now', 'localtime')),

    foreign key (activity_id) references activities(id)
);
