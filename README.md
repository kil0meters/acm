# Chico ACM Website

This repository holds the code for the website that may or may not be used by
the CSU Chico chapter of the ACM.

## Project organization

Currently, the project is laid out as follows: 

- `./app` — Code for the frontend
    - `./app/styles` — Currently stores all css styling for the website
- `./server` — Code for the backend
- `./ramiel` - Service for running tests
- `./src` — Shared code between the frontend and backend, such as forms.
- `./sql` - Database schemas

## Running

First install Rust, then clone the repo.

```sh
git clone git@github.com:/kil0meters/acm.git
cd acm
```

Initialize the database:

```
echo "DATABASE_URL=sqlite://./sqlite.db" > .env
cat sql/*.sql | sqlite3 db.sqlite
```

Build the frontend automatically on changes

```sh
cd app
trunk watch
```

Then run the server:

```sh
cargo run
```

Alternatively, if you have
[`cargo-watch`](https://github.com/watchexec/cargo-watch) installed:

```sh
cargo watch -i "db.sqlite*" -x run
```

