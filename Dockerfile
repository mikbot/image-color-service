FROM --platform=$TARGETOS/$TARGETARCH scratch
COPY $TARGETARCH/*-linux-musl/release/image-color-service /
ENTRYPOINT ["/image-color-service"]
