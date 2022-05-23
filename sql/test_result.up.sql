create table test_results (
    id integer primary key not null,
    submission_id integer not null,
    test_id integer not null,

    runtime integer not null,
    expected_output text not null,
    success boolean not null,

    foreign key (submission_id) references submissions(id),
    foreign key (test_id) references tests(id)
);
