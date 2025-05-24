FROM rust:1.87

WORKDIR /usr/src/fe-pong
COPY . .
RUN rustup target add wasm32-unknown-unknown
RUN cargo build --bin pong --release --target wasm32-unknown-unknown --features="build-pong"
RUN mkdir static
RUN cp target/wasm32-unknown-unknown/release/pong.wasm static/
RUN cargo build --bin game_server --release --features="build-server"
ENV ROCKET_PORT=5000
ENV ROCKET_ADDRESS=0.0.0.0 
CMD ["cargo", "run", "--bin", "game_server", "--release", "--features=build-server"]
