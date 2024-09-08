FROM scratch
COPY target/x86_64-unknown-linux-musl/release/image-color-service /
ENTRYPOINT ["/image-color-service"]