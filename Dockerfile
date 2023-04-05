ARG TYPE=dev

FROM node as base
RUN curl https://sh.rustup.rs -sSf |  sh -s -- -y
ENV PATH "$PATH:/root/.cargo/bin"
RUN  curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
WORKDIR /app
COPY . .
WORKDIR /app/assets
RUN npm install


FROM base as dev-build
WORKDIR /app
RUN cargo build
RUN mv ./target/debug/doodle-book ./doodle-book
WORKDIR /app/assets
RUN  npm run build-dev

FROM base as prod-build
WORKDIR /app
RUN cargo build --release
RUN mv ./target/release/doodle-book ./doodle-book
WORKDIR /app/assets
RUN  npm run build-prod

FROM ${TYPE}-build as source

FROM debian:bullseye-slim as runner
RUN apt-get update && apt-get -y install libpq-dev
WORKDIR /app
COPY ./Rocket.toml ./
COPY --from=source /app/doodle-book /usr/local/bin/doodle-book
COPY --from=source /app/assets/static ./assets/static
EXPOSE 8000
CMD [ "doodle-book" ]

