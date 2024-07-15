#build from latst rust version
FROM rust:latest as build

# install libpq, libsqlite and create new empty binary project
RUN apt-get update; \
    apt-get install --no-install-recommends -y libpq-dev; \
    USER=root cargo new --bin app
WORKDIR /app

# copy manifests
COPY ./Cargo.toml ./Cargo.toml

# build this project to cache dependencies
RUN cargo build; \
    rm src/*.rs

# copy project source and necessary files
COPY ./src ./src
COPY ./migrations ./migrations
COPY ./diesel.toml .
COPY ./.env .

# rebuild app with project source
RUN rm ./target/debug/deps/rest_jwt_rust*; \
    cargo build --release

# deploy stage
FROM debian:buster-slim

# install libpq and libsqlite
RUN apt-get update; \
    apt-get install --no-install-recommends -y libpq5; \
    rm -rf /var/lib/apt/lists/*

# copy binary and configuration files
COPY --from=build /app/target/release/rest-jwt-rust .
COPY --from=build /app/.env .
COPY --from=build /app/diesel.toml .
COPY ./wait-for-it.sh .

# expose port
EXPOSE 8080
# run the binary
CMD ["/app/target/release/rest-jwt-rust"]