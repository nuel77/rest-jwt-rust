#build from latst rust version
FROM rust:latest

# install libpq, libsqlite and create new empty binary project
RUN apt-get update; \
    apt-get install -y --no-install-recommends postgresql-common libpq-dev libpq5 libpq-dev libsqlite3-dev; \
    USER=root cargo new --bin app
WORKDIR /app

# copy manifests
COPY ./Cargo.toml ./Cargo.toml

# copy project source and necessary files
COPY ./src ./src
COPY ./migrations ./migrations
COPY ./diesel.toml .
COPY ./.env .

# rebuild app with project source
RUN cargo build --release

CMD ["/app/target/release/rest-jwt-rust"]