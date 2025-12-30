FROM --platform=$TARGETPLATFORM gcr.io/distroless/cc-debian12:nonroot

# docker buildx args automatically available
ARG BUILDPLATFORM
ARG TARGETPLATFORM
ARG TARGETOS
ARG TARGETARCH

ARG TARGET_USER="10001:10001"
USER $TARGET_USER

# default jemalloc tuning suitable for most deployments
ENV MALLOC_CONF="abort_conf:true,narenas:8,tcache_max:4096,dirty_decay_ms:5000,muzzy_decay_ms:5000"

WORKDIR /app

COPY --chown=$TARGET_USER ./out/rauthy_$TARGETARCH ./rauthy
COPY --chown=$TARGET_USER ./config-local-test.toml ./config-local-test.toml

# we are copying the empty dirs for proper access rights upfront
COPY --chown=$TARGET_USER ./out/empty/ ./data
COPY --chown=$TARGET_USER ./out/empty/ ./tls


CMD ["/app/rauthy"]
