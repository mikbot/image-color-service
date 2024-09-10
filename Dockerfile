FROM --platform=$TARGETOS/$TARGETARCH scratch
ARG TARGETARCH
COPY --chmod=755 $TARGETARCH/*-linux-musl/release/image-color-service /
ENTRYPOINT ["/image-color-service"]
