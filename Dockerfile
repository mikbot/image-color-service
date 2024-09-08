FROM scratch
COPY ./target/release/image-color-service /
ENTRYPOINT ["/image-color-service"]
