FROM rust:1.84-alpine AS build
WORKDIR /app

RUN apk add --no-cache clang lld musl-dev pkgconf openssl-dev
ENV OPENSSL_DIR=/usr

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs \
    && RUSTFLAGS='-C target-feature=-crt-static' cargo build --release \
    && rm -rf src target/release/deps/crypto*
COPY src/ src/
RUN RUSTFLAGS='-C target-feature=-crt-static' cargo build --release

FROM alpine:3.18

RUN apk add --no-cache libgcc wget
WORKDIR /app
COPY --from=build /app/target/release/crypto-assets ./server
COPY assets/ assets/
COPY templates/ templates/
EXPOSE 3999
ENV RUST_LOG=info
HEALTHCHECK --interval=30s --timeout=3s --retries=3 CMD wget -qO- http://127.0.0.1:3999/ping || exit 1
CMD ["./server"]
