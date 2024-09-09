FROM --platform=$TARGETOS/$TARGETARCH scratch
ARG TARGETARCH
COPY $TARGETARCH/image-color-service /
ENTRYPOINT ["/image-color-service"]