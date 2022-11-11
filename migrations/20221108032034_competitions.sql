create table competitions (
    id integer primary key not null,
    name text not null,

    start datetime not null,
    end datetime not null
);

alter table problems add competition_id integer references competitions(id);

create table teams (
    id integer primary key not null,
    competition_id integer not null,

    name text not null,

    foreign key (competition_id) references competitions(id)
);

create table team_members (
    id integer primary key not null,
    user_id integer not null,
    team_id integer not null,

    foreign key (user_id) references users(id),
    foreign key (team_id) references teams(id)
);
