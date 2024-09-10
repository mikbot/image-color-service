FROM --platform=$TARGETOS/$TARGETARCH scratch
COPY --from=builder $TARGETARCH/image-color-service /
ENTRYPOINT ["/image-color-service"]
