alter table tests add hidden boolean not null default false;

-- make all tests which are 10kb > hidden as a sane default
update tests set hidden = true where length(input) > 10000;
