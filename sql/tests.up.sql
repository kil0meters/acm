create table tests (
    id integer primary key not null,
    problem_id integer not null,
    test_number integer not null,
    input text not null,
    output text not null,

    foreign key (problem_id) references problems(id)
);

