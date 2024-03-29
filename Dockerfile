FROM rust:1.67.0-alpine as builder
WORKDIR /usr/src/app
COPY . .
RUN apk add --no-cache musl-dev && cargo build --release

FROM scratch
COPY --from=builder /usr/src/app/target/release/image-color-service /
ENTRYPOINT ["/image-color-service"]
