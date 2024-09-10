FROM --platform=$TARGETOS/$TARGETARCH scratch
ARG TARGETARCH
COPY $TARGETARCH/*-linux-musl/release/image-color-service /
ENTRYPOINT ["/image-color-service"]
