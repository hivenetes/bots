FROM --platform=$BUILDPLATFORM golang:1.18-alpine AS builder
ARG TARGETOS
ARG TARGETARCH
ARG VERSION
RUN mkdir /src
ADD . /src/
WORKDIR /src
RUN CGO_ENABLED=0 GOOS=${TARGETOS} GOARCH=${TARGETARCH} go build -ldflags "-X main.Version=$VERSION" -a -o cats main.go
FROM alpine
COPY --from=builder /src/static /app/static
COPY --from=builder /src/cats /app/cats
WORKDIR /app
EXPOSE 8080
ENTRYPOINT ["/app/cats"]