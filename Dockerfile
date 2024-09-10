FROM --platform=$TARGETOS/$TARGETARCH scratch
COPY $TARGETARCH/image-color-service /
ENTRYPOINT ["/image-color-service"]
