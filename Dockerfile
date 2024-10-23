FROM --platform=$TARGETPLATFORM gcr.io/distroless/cc-debian12:nonroot

# docker buildx args automatically available
ARG BUILDPLATFORM
ARG TARGETPLATFORM
ARG TARGETOS
ARG TARGETARCH

ARG TARGET_USER="10001:10001"
USER $TARGET_USER

WORKDIR /app

COPY --chown=$TARGET_USER ./out/rauthy_$TARGETARCH ./rauthy
COPY --chown=$TARGET_USER ./out/empty/ ./data

COPY --chown=$TARGET_USER ./tls/ca-chain.pem ./tls/ca-chain.pem
COPY --chown=$TARGET_USER ./tls/cert-chain.pem ./tls/cert-chain.pem
COPY --chown=$TARGET_USER ./tls/key.pem ./tls/key.pem

COPY --chown=$TARGET_USER ./rauthy.deploy.cfg ./rauthy.cfg

CMD ["/app/rauthy"]
