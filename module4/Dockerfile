FROM rust:1.72 as builder

WORKDIR /usr/src/app

COPY . .

RUN rustup target install wasm32-unknown-unknown
RUN cargo install wasm-bindgen-cli
RUN cargo build --target wasm32-unknown-unknown --release
RUN wasm-bindgen --out-dir ./out --target web ./target/wasm32-unknown-unknown/release/balls.wasm
RUN cp ./static/* ./out

FROM nginx:alpine as host

COPY --from=builder /usr/src/app/out /usr/share/nginx/html

EXPOSE 80
