ARG IMAGE
ARG IMAGE_DATE

#FROM --platform=$BUILDPLATFORM $IMAGE:$TARGETARCH-$IMAGE_DATE AS builder
FROM --platform=$BUILDPLATFORM $IMAGE:$TARGETARCH-$IMAGE_DATE AS builder

# docker buildx args automatically available
ARG BUILDPLATFORM
ARG TARGETPLATFORM
ARG TARGETOS
ARG TARGETARCH

ARG FEATURES="default"
ARG MODE="release"

RUN echo "Building on $BUILDPLATFORM for $TARGETOS/$TARGETARCH in $MODE mode with target $(target)" && sleep 3

RUN mkdir -p out && mkdir empty

COPY . .

#RUN --mount=type=cache,target=/usr/local/cargo/registry <<EOF

RUN <<EOF
#!/bin/bash
set -e

if [ "release" = "$MODE" ]; then
  cargo build --release --features $FEATURES --target $(target)
else
  cargo build --features $FEATURES --target $(target)
fi

mv target/$(target)/$MODE/rauthy out/rauthy
EOF

FROM --platform=$BUILDPLATFORM scratch

ARG TARGET_USER="10001:10001"

USER $TARGET_USER

WORKDIR /app

COPY --from=builder --chown=$TARGET_USER /work/out/rauthy ./rauthy
COPY --from=builder --chown=$TARGET_USER /work/empty ./data

COPY --from=builder --chown=$TARGET_USER /work/tls/ca-chain.pem ./tls/ca-chain.pem
COPY --from=builder --chown=$TARGET_USER /work/tls/cert-chain.pem ./tls/cert-chain.pem
COPY --from=builder --chown=$TARGET_USER /work/tls/key.pem ./tls/key.pem

COPY --from=builder --chown=$TARGET_USER /work/rauthy.deploy.cfg ./rauthy.cfg

CMD ["/app/rauthy"]
