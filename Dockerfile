#build from latst rust version
FROM rust:1.79.0-slim-bullseye as build

# install libpq, libsqlite and create new empty binary project
RUN apt-get update; \
    apt-get install -y --no-install-recommends postgresql-common libpq5 libpq-dev libsqlite3-dev; \
    rm -rf /var/lib/apt/lists/* ; \
    USER=root cargo new --bin app

WORKDIR /app

# copy manifests
COPY ./Cargo.toml ./Cargo.toml

# copy project source and necessary files
COPY ./src ./src
COPY ./migrations ./migrations
COPY ./diesel.toml .
COPY ./.env .

# Set the RUST_LOG environment variable
ENV RUST_LOG=info

# rebuild app with project source
RUN cargo build --release

# deploy stage
FROM debian:bullseye-slim

# create app directory
RUN mkdir app
WORKDIR /app

# install libpq, libsqlite and create new empty binary project
RUN apt-get update; \
    apt-get install -y --no-install-recommends postgresql-common libpq5 libpq-dev libsqlite3-dev; \
    rm -rf /var/lib/apt/lists/*


# copy the built binary from the build stage
COPY --from=build /app/target/release/rest-jwt-rust .
COPY --from=build /app/.env .
COPY --from=build /app/diesel.toml .

# expose port
EXPOSE 8080

#run
CMD ["/app/rest-jwt-rust"]