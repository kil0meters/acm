create table submissions (
    id integer primary key not null,
    problem_id integer not null,
    user_id integer not null,

    success boolean not null,
    runtime integer not null,
    error text,
    code text not null,

    time datetime not null default (datetime('now', 'localtime')),

    foreign key (problem_id) references problems(id),
    foreign key (user_id) references users(id)
);
