FROM --platform=$BUILDPLATFORM alpine:3.18.4 AS builderBackend

# docker buildx args automatically available
ARG BUILDPLATFORM
ARG TARGETPLATFORM
ARG TARGETOS
ARG TARGETARCH

RUN echo "I'm building on $BUILDPLATFORM for $TARGETOS/$TARGETARCH"

WORKDIR /work

# Just a workaround to get an empty dir and be able to copy it over to scratch with
# the correct access rights
RUN mkdir data

FROM --platform=$TARGETPLATFORM scratch

# docker buildx args automatically available
ARG BUILDPLATFORM
ARG TARGETPLATFORM
ARG TARGETOS
ARG TARGETARCH

# for selecting the correct pre-built binary
ARG DB

USER 10001:10001

WORKDIR /app

COPY --chown=10001:10001 /out/rauthy-"$DB"-"$TARGETARCH" ./rauthy
COPY --chown=10001:10001 --from=builderBackend /work/data ./data

COPY --chown=10001:10001 tls/ca-chain.pem ./tls/ca-chain.pem
COPY --chown=10001:10001 tls/cert-chain.pem ./tls/cert-chain.pem
COPY --chown=10001:10001 tls/key.pem ./tls/key.pem

COPY --chown=10001:10001 rauthy.deploy.cfg ./rauthy.cfg

CMD ["/app/rauthy"]
