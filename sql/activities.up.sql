create table activities (
    id integer primary key not null,
    meeting_id integer not null,

    title text not null,
    description text not null,

    -- SOLO: A solo competition e.g. ICPC
    -- PAIR: A group competition e.g. weekly
    -- LECT: Some type of guest lecture
    activity_type text not null,

    foreign key (meeting_id) references meetings(id)
);
