# Docker actix postgres redis todo example

This is a Dockerized version of the
[actix todo example](https://github.com/actix/examples/tree/master/todo)
in
[Rust](https://www.rust-lang.org/).
It uses
[Docker](https://www.docker.com/),
[actix](https://actix.rs/),
[postgres](https://www.postgresql.org/),
[redis](https://redis.io/),
and [diesel](https://diesel.rs/).


To run it, do

```
docker-compose up
```

You should pass in environment variables, e.g. through a `.env` file,
something like as follows:

```
POSTGRES_USER=foobar
POSTGRES_PASSWORD=ram-crud-headache-airplane
POSTGRES_DB=todo
POSTGRES_PORT=5432
REDIS_HOST=redis
REDIS_PORT=6379
TODO_PORT=9003
DEVELOPMENT=true
```

In development mode, the app will auto-reload on code change
using [cargo-watch](https://github.com/passcod/cargo-watch)
and [listenfd](https://github.com/mitsuhiko/rust-listenfd).

To view the app, visit [http://localhost:8000](http://localhost:8000).

## Dependencies

This is up-to-date as of the following releases. See `Cargo.toml`

```
actix-web = "2.0.0-rc"
actix-files = "0.2.0"
actix-session = "0.3.0"
actix-redis = {version = "0.8.0", features = ["web"]}
actix-rt = "1.0.0"
```

## License

Code by Kyle Jensen in this repo is available under the 
[Unlicense](https://unlicense.org/). The code by the Actix
authors is under an Apache license [here](https://github.com/actix/examples/blob/master/LICENSE).
