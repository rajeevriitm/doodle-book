FROM rust:1.68 as builder
WORKDIR /app
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim as runner
RUN apt-get update && apt-get -y install libpq-dev
COPY --from=builder /usr/local/cargo/bin/doodle-book /usr/local/bin/doodle-book
CMD [ "doodle-book" ]