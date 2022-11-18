create table problem_tags (
    id integer primary key not null,
    problem_id integer not null,

    name text not null,

    foreign key (problem_id) references problems(id)
);

alter table problems add difficulty text not null default "Easy";
