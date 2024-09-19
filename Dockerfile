FROM --platform=$TARGETPLATFORM gcr.io/distroless/cc-debian12:nonroot

# docker buildx args automatically available
ARG BUILDPLATFORM
ARG TARGETPLATFORM
ARG TARGETOS
ARG TARGETARCH

WORKDIR /app

COPY ./out/rauthy_$TARGETARCH ./rauthy
COPY ./out/empty/ ./data

COPY ./tls/ca-chain.pem ./tls/ca-chain.pem
COPY ./tls/cert-chain.pem ./tls/cert-chain.pem
COPY ./tls/key.pem ./tls/key.pem

COPY ./rauthy.deploy.cfg ./rauthy.cfg

CMD ["/app/rauthy"]
