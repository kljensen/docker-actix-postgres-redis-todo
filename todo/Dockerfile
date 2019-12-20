FROM rust:1.39.0-slim-stretch
WORKDIR /app
RUN  apt-get update \
    && apt-get install -y wget gnupg2
RUN \
    echo "deb http://apt.postgresql.org/pub/repos/apt/ stretch-pgdg main" >> /etc/apt/sources.list.d/pgdg.list \
    && wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | apt-key add - \
    && apt-get update \
    && apt-get install -y postgresql-11 libpq-dev

RUN cargo install \
    systemfd  \
    cargo-watch
RUN cargo install diesel_cli --no-default-features --features postgres