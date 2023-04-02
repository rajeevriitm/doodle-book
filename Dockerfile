FROM rust:1.68 as base
RUN apt-get update && apt-get -y install npm nodejs
RUN  curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
WORKDIR /app
COPY . .
WORKDIR /app/assets
RUN npm install


FROM base as dev-build
WORKDIR /app
RUN cargo build
WORKDIR /app/assets
RUN  npm run build-dev

FROM base as prod-build
RUN cargo build --release

FROM debian:bullseye-slim as runner
RUN apt-get update && apt-get -y install libpq-dev
COPY --from=builder /app/doodle-book/target/release/doodle-book /usr/local/bin/doodle-book
COPY --from=builder /app/doodle-book/target/debug/doodle-book /usr/local/bin/doodle-book-debug
CMD [ "doodle-book-debug" ]