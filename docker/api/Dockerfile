FROM rust:alpine AS builder

RUN USER=root cargo new --bin graphql
WORKDIR /graphql

ENV     RUSTFLAGS="-C target-feature=-crt-static"
RUN     apk add -q --update-cache --no-cache build-base openssl-dev musl pkgconfig protobuf-dev

COPY ../graphql/Cargo.toml ./Cargo.toml
COPY ../graphql/Cargo.lock ./Cargo.lock
COPY ../graphql/api        ./api
COPY ../graphql/crawler    ./crawler
COPY ../graphql/error      ./error
COPY ../graphql/rank       ./rank
COPY ../graphql/search     ./search

RUN cargo build --release

FROM alpine:3 AS runtime

RUN apk add --no-cache libgcc

RUN addgroup -S appgroup && adduser -S rust -G appgroup
USER rust

COPY --from=builder /graphql/target/release/api /bin/graphql

CMD ["./bin/graphql"]
