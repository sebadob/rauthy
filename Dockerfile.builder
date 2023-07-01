FROM rust:1.65-alpine3.16

ENV USER=user
ENV UID=10001

RUN apk update && apk add --no-cache tzdata build-base g++ openssl openssl-dev clang jq ca-certificates bash linux-headers

#WORKDIR /usr/local/src
#COPY openssl/openssl-3.0.3.tar.gz ./openssl-3.0.3.tar.gz
#RUN tar -xvf openssl-3.0.3.tar.gz

#WORKDIR /usr/local/src/openssl-3.0.3
#RUN ./config --prefix=/usr/local/ssl --openssldir=/usr/local/ssl shared zlib && \
#    make && \
#    make install && \
#    echo "/usr/local/ssl/lib" > /etc/ld.so.conf.d/openssl-3.0.3.conf && \
#    ldconfig -v && \
#    rm /usr/bin/c_rehash /usr/bin/openssl && \
#    cp /usr/local/ssl/bin/* /usr/local/bin/ && \
#    ln -s /usr/local/ssl/lib64/libssl.so /usr/lib/libssl.so.3 && \
#    ln -s /usr/local/ssl/lib64/libcrypto.so /usr/lib/libcrypto.so.3

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

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

RUN cargo build --release --target x86_64-unknown-linux-musl
