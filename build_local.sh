#!/bin/bash

TAG=$(cat rauthy-main/Cargo.toml | grep '^version =' | cut -d " " -f3 | xargs)

cargo build --release --target x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/release/rauthy out/

# registry.netitservices.com is an internal development registry
docker build --no-cache -t registry.netitservices.com/sd/rauthy:$TAG .
docker push registry.netitservices.com/sd/rauthy:$TAG
docker build --no-cache -f Dockerfile.debug -t registry.netitservices.com/sd/rauthy:$TAG-debug .
docker push registry.netitservices.com/sd/rauthy:$TAG-debug

## tagged public release
#docker tag registry.netitservices.com/sd/rauthy:$TAG sdobedev/rauthy:$TAG
#docker push sdobedev/rauthy:$TAG
#docker tag registry.netitservices.com/sd/rauthy:$TAG-debug sdobedev/rauthy:$TAG-debug
#docker push sdobedev/rauthy:$TAG-debug
#
## latest public release
#docker tag registry.netitservices.com/sd/rauthy:$TAG sdobedev/rauthy:latest
#docker push sdobedev/rauthy:latest
