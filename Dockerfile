FROM rust:1.70-alpine3.18 AS builderBackend

WORKDIR /work

COPY migrations/ ./migrations/
COPY rauthy-common/ ./rauthy-common/
COPY rauthy-handlers/ ./rauthy-handlers/
COPY rauthy-main/ ./rauthy-main/
COPY rauthy-models/ ./rauthy-models/
COPY rauthy-service/ ./rauthy-service/
COPY static ./static/
COPY templates ./templates/
COPY Cargo.* ./

RUN mkdir data

#RUN cargo build --release --target x86_64-unknown-linux-musl
#RUN cargo build --release

#RUN chown 10001:10001 -R /work

#FROM node:alpine as builderFrontend
#
#WORKDIR /work
#
#COPY frontend/package* ./
#RUN npm install
#
#COPY frontend/ ./
#RUN ./node_modules/.bin/next build && \
#    ./node_modules/.bin/next export

FROM alpine:3.18.2
#FROM gcr.io/distroless/cc:debug
#FROM gcr.io/distroless/cc

#RUN apt update && apt install -y gcc ca-certificates && \
#    ln -fs /usr/share/zoneinfo/Europe/Zurich /etc/localtime && dpkg-reconfigure -f noninteractive tzdata && \
#    apt -y clean

USER 10001:10001

#COPY --from=builderBackend /etc/passwd /etc/passwd
#COPY --from=builderBackend /etc/group /etc/group
#COPY --from=builderBackend /usr/local/ssl/bin/c_rehash /usr/local/bin/
#COPY --from=builderBackend /usr/local/ssl/bin/openssl /usr/local/bin/
#COPY --from=builderBackend /usr/local/ssl/lib64/libssl.so /usr/lib/libssl.so.3
#COPY --from=builderBackend /usr/local/ssl/lib64/libcrypto.so /usr/lib/libcrypto.so.3

WORKDIR /app

#RUN mkdir -p static/v1
#
#RUN mkdir migrations
#COPY migrations/ ./migrations/
#COPY --chown=10001:10001 --from=builderBackend /work/target/release/rauthy .
COPY --chown=10001:10001 /out/rauthy .
COPY --chown=10001:10001 --from=builderBackend /work/data ./data

COPY --chown=10001:10001 tls/ ./tls/
#COPY --chown=10001:10001 static/ ./static/
#COPY migrations/ ./migrations/
COPY --chown=10001:10001 rauthy.deploy.cfg ./rauthy.cfg

#RUN chown 10001:10001 -R /app

#ENTRYPOINT ["/app/rauthy"]
#ENTRYPOINT [""]
CMD ["/app/rauthy"]
