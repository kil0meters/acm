-- Add migration script here
create table jobs (
    id integer primary key not null,
    submission_id integer,

    -- PENDING: The job is in queue
    -- IN_PROGRESS: The job is currently running
    -- COMPLETE: The job is done
    status text not null default "PENDING",

    foreign key (submission_id) references submissions(id)
);
