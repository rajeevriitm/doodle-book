ARG TYPE=prod

FROM node as base
RUN curl https://sh.rustup.rs -sSf |  sh -s -- -y
ENV PATH "$PATH:/root/.cargo/bin"
RUN  curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
WORKDIR /app
COPY Cargo.toml Cargo.lock  ./
RUN mkdir ./src && echo 'fn main() { panic!("Dummy Image Called!")}' > ./src/main.rs
RUN cargo build  && cargo build  --release
WORKDIR /app/doodle-wasm
COPY doodle-wasm/Cargo.toml doodle-wasm/Cargo.lock ./
RUN mkdir ./src && echo 'fn main() { panic!("Dummy Image Called!")}' > ./src/lib.rs
RUN wasm-pack build && wasm-pack build --release
COPY doodle-wasm/src src/
RUN touch src/main.rs
WORKDIR /app
COPY src src/
COPY migrations migrations/
RUN touch src/main.rs
COPY assets/*.js assets/*.json assets/
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
RUN cargo build  --release
RUN mv ./target/release/doodle-book ./doodle-book
WORKDIR /app/assets
RUN  npm run build-prod

FROM ${TYPE}-build as source

FROM debian:bullseye-slim as runner
COPY --from=source /app/doodle-book /usr/local/bin/doodle-book
COPY --from=source /app/assets/static /app/assets/static
RUN apt-get update && apt-get -y install libpq5 && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY ./Rocket.toml ./
COPY templates templates/
COPY ./assets/static/style.css  /app/assets/static/style.css
EXPOSE 8000
CMD [ "doodle-book" ]
