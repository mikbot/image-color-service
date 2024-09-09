FROM --platform=$TARGETOS/$TARGETARCH rust:alpine as builder
WORKDIR /usr/src/app
COPY . .
RUN apk add --no-cache musl-dev && cargo build --release

FROM --platform=$TARGETOS/$TARGETARCH scratch
COPY --from=builder /usr/src/app/target/release/image-color-service /
ENTRYPOINT ["/image-color-service"]