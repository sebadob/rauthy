set shell := ["bash", "-uc"]

export TAG := `cat rauthy-main/Cargo.toml | grep '^version =' | cut -d " " -f3 | xargs`

# runs the application
run:
    cargo run --target x86_64-unknown-linux-musl


# runs the UI in development mode
run-ui:
    cd frontend
    npm run dev


# prints out the currently set version
version:
    echo $TAG


# runs the full set of tests
test:
    #!/usr/bin/env bash

    cargo build
    cargo run test &
    sleep 1
    PID=$(echo "$!")
    echo "PID: $PID"

    cargo test
    kill "$PID"
    echo All tests successful


# builds the frontend and exports to static html
build-ui:
    #!/usr/bin/env bash
    set -euxo pipefail

    # cleanup old files
    rm -rf static/v1/*
    rm -rf templates/html/*

    # make sure all output FOLDERS exist
    FOLDERS=(
    "templates/html"
    "static/v1"
    )
    for folder in "${FOLDERS[@]}"; do
      if [ ! -d "$folder" ]; then
          mkdir -p "$directory"
      fi
    done

    # build the frontend
    cd frontend
    npm run build
    cd ..

    # set correct values in html outputs for pre-rendering
    PAGES=(
    "templates/html/*.html"
    "templates/html/admin/*.html"
    "templates/html/oidc/*.html"
    "templates/html/users/*.html"
    "templates/html/users/{id}/reset/*.html"
    )
    for folder in "${PAGES[@]}"; do
        for html in $folder; do
          # for pre-rendering colors
          sed -i 's/#6b3d99;/{{{{ col_act1 }};/g' "$html"
          sed -i 's/#714d99;/{{{{ col_act1a }};/g' "$html"
          sed -i 's/#388c51;/{{{{ col_act2 }};/g' "$html"
          sed -i 's/#4d8c62;/{{{{ col_act2a }};/g' "$html"
          sed -i 's/#3d5d99;/{{{{ col_acnt }};/g' "$html"
          sed -i 's/#36486b;/{{{{ col_acnta }};/g' "$html"
          sed -i 's/#43993d;/{{{{ col_ok }};/g' "$html"
          sed -i 's/#993d49;/{{{{ col_err }};/g' "$html"
          sed -i 's/#545454;/{{{{ col_glow }};/g' "$html"
          sed -i 's/#b2b2b2;/{{{{ col_gmid }};/g' "$html"
          sed -i 's/#f2f2f2;/{{{{ col_ghigh }};/g' "$html"
          sed -i 's/#383838;/{{{{ col_text }};/g' "$html"
          sed -i 's/#f7f7f7;/{{{{ col_bg }};/g' "$html"
          # for the nonce in the CSP for script files
          sed -i 's/<link /<link nonce="{{{{ nonce }}" /g' "$html"
          sed -i 's/<script>/<script nonce="{{{{ nonce }}">/g' "$html"
        done;
    done


# builds the rauthy book
build-docs:
    #!/usr/bin/env bash
    set -euxo pipefail
    cd rauthy-book
    mdbook build -d ../docs


# builds the whole application in release mode
build: build-docs build-ui
    cargo clippy -- -D warnings
    cargo build --release --target x86_64-unknown-linux-musl
    cp target/x86_64-unknown-linux-musl/release/rauthy out/


# makes sure everything is fine
is-clean: test build
    #!/usr/bin/env bash
    set -euxo pipefail

    # exit early if clippy emits warnings
    cargo clippy -- -D warnings

    # make sure everything has been committed
    git diff --exit-code

    echo all good


# sets a new git tag and pushes it
release:
    #!/usr/bin/env bash
    set -euxo pipefail

    git tag "v$TAG"
    git push origin "v$TAG"


# publishes the application images
publish:
    docker build --no-cache -t sdobedev/rauthy:$TAG .
    docker push sdobedev/rauthy:$TAG
    docker build --no-cache -f Dockerfile.debug -t sdobedev/rauthy:$TAG-debug .
    docker push sdobedev/rauthy:$TAG-debug

    docker tag sdobedev/rauthy:$TAG sdobedev/rauthy:latest
    docker push sdobedev/rauthy:latest
