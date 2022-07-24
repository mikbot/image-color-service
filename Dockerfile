FROM rust:1.62.1 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM scratch
COPY --from=builder /usr/src/app/target/release/image-color-service /app
ENTRYPOINT ["/app"]
