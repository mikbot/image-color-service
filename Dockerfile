FROM --platform=$TARGETOS/$TARGETARCH scratch
COPY *-linux-musl/release/image-color-service /
ENTRYPOINT ["/image-color-service"]
