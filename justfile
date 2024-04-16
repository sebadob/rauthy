set shell := ["bash", "-uc"]

export TAG := `cat Cargo.toml | grep '^version =' | cut -d " " -f3 | xargs`

db_url_sqlite := "sqlite:data/rauthy.db"
db_url_sqlite_mem := "sqlite::memory"
db_url_postgres := "postgresql://rauthy:123SuperSafe@localhost:5432/rauthy"

test_pid_file := ".test_pid"


# Creates a new Root + Intermediate CA for development and testing TLS certificates
create-root-ca:
    # Password for both root and intermediate dev CA is always: 123SuperMegaSafe

    # The nioca container runs with 10001:10001 uid:gid
    #chmod 0766 tls/ca

    # Root CA
    docker run --rm -it -v ./tls/ca:/ca -u $(id -u ${USER}):$(id -g ${USER}) \
          ghcr.io/sebadob/nioca \
          x509 \
          --stage root \
          --clean

    # Intermediate CA
    docker run --rm -it -v ./tls/ca:/ca -u $(id -u ${USER}):$(id -g ${USER}) \
          ghcr.io/sebadob/nioca \
          x509 \
          --stage intermediate

    cp tls/ca/x509/intermediate/ca-chain.pem tls/ca-chain.pem


# Create a new End Entity TLS certificate for development and testing
# Intermediate CA DEV password: 123SuperMegaSafe
create-end-entity-tls:
    # create the new certificate
    docker run --rm -it -v ./tls/ca:/ca -u $(id -u ${USER}):$(id -g ${USER}) \
          ghcr.io/sebadob/nioca \
          x509 \
          --cn 'localhost' \
          --alt-name-dns 'localhost' \
          --alt-name-dns 'redhac.local' \
          --alt-name-dns 'rauthy.local' \
          --alt-name-ip '127.0.0.1' \
          --alt-name-uri 'localhost:8080' \
          --alt-name-uri 'localhost:8443' \
          --usages-ext server-auth \
          --usages-ext client-auth \
          --o 'Rauthy OIDC' \
          --stage end-entity

    # copy it in the correct place
    cp tls/ca/x509/end_entity/$(cat tls/ca/x509/end_entity/serial)/cert-chain.pem tls/cert-chain.pem
    cp tls/ca/x509/end_entity/$(cat tls/ca/x509/end_entity/serial)/key.pem tls/key.pem


# This may be executed if you don't have a local `docker buildx` setup
docker-buildx-setup:
    #!/usr/bin/env bash
    set -euxo pipefail
    clear

    # create 'rauthy_builder' buildx instance
    docker buildx create --name rauthy_builder --bootstrap --use
    docker buildx inspect rauthy_builder


# Starts maildev (https://github.com/maildev/maildev) on your localhost for E-Mail testing
mailcrab-start:
    #!/usr/bin/env bash
    docker run -d -p 1080:1080 -p 1025:1025 --name mailcrab --restart unless-stopped marlonb/mailcrab


# Stops maildev
mailcrab-stop:
    #!/usr/bin/env bash
    docker stop mailcrab
    docker rm mailcrab


# Just uses `cargo fmt --all`
fmt:
    #!/usr/bin/env bash
    cargo fmt --all


# Force-pulls the latest `cross` docker image for cross-compilation
pull-latest-cross:
    #!/usr/bin/env bash
    docker pull ghcr.io/cross-rs/aarch64-unknown-linux-musl:main
    docker pull ghcr.io/cross-rs/x86_64-unknown-linux-musl:main


# clippy with sqlite features
clippy:
    clear
    DATABASE_URL={{db_url_sqlite}} cargo clippy --features sqlite


# clippy with postgres features
clippy-postgres:
    clear
    DATABASE_URL={{db_url_postgres}} cargo clippy


# re-create and migrate the sqlite database with sqlx
migrate:
    mkdir -p data/
    rm -f data/rauthy.db*
    DATABASE_URL={{db_url_sqlite}} sqlx database create
    DATABASE_URL={{db_url_sqlite}} sqlx migrate run --source migrations/sqlite


# migrate the postgres database with sqlx
migrate-postgres:
    DATABASE_URL={{db_url_postgres}} sqlx migrate run --source migrations/postgres


# runs the application with sqlite feature
run:
    DATABASE_URL={{db_url_sqlite}} cargo run --features sqlite


# runs the application with postgres feature
run-postgres:
    DATABASE_URL={{db_url_postgres}} cargo run


# runs the UI in development mode
run-ui:
    #!/usr/bin/env bash
    cd frontend
    npm run dev -- --host


# prints out the currently set version
version:
    #!/usr/bin/env bash
    echo "v$TAG"


# prepare DB migrations for SQLite for compile-time checked queries
prepare: migrate
    DATABASE_URL={{db_url_sqlite}} cargo sqlx prepare --workspace -- --features sqlite


# prepare DB migrations for Postgres for compile-time checked queries
prepare-postgres: migrate-postgres
    DATABASE_URL={{db_url_postgres}} cargo sqlx prepare --workspace


# only starts the backend in test mode with sqlite database for easier test debugging
test-backend: migrate prepare
    #!/usr/bin/env bash
    set -euxo pipefail
    clear

    DATABASE_URL={{db_url_sqlite}} cargo build --features sqlite
    DATABASE_URL={{db_url_sqlite}} ./target/debug/rauthy test


# stops a possibly running test backend that may have spawned in the background for integration tests
test-backend-stop:
    #!/usr/bin/env bash
    set -euxo pipefail

    if [ -f {{test_pid_file}} ]; then
      PID=$(cat .test_pid)
      echo "Stopping background process with PID $PID"
      kill "$PID" || echo "Could not stop the process"
      rm {{test_pid_file}}
    else
      echo "No background test process running"
    fi

# runs a single test with sqlite - needs the backend being started manually
test test="":
    #!/usr/bin/env bash
    set -euxo pipefail
    clear

    DATABASE_URL={{db_url_sqlite}} cargo test --features sqlite {{test}}


# runs the full set of tests with in-memory sqlite
test-full test="": test-backend-stop migrate prepare
    #!/usr/bin/env bash
    set -euxo pipefail
    clear

    DATABASE_URL={{db_url_sqlite}} cargo build --features sqlite
    DATABASE_URL={{db_url_sqlite}} ./target/debug/rauthy test &
    #just migrate-sqlite && DATABASE_URL=sqlite:data/rauthy.db cargo run --features sqlite test
    #cargo test --features sqlite test_userinfo
    sleep 1
    PID=$(echo "$!")
    echo $PID > {{test_pid_file}}
    echo "PID: $PID"

    DATABASE_URL={{db_url_sqlite}} cargo test --features sqlite {{test}}
    kill "$PID"
    echo All tests successful


# runs the full set of tests with postgres
test-postgres test="": test-backend-stop migrate-postgres prepare-postgres
    #!/usr/bin/env bash
    set -euxo pipefail
    clear

    DATABASE_URL={{db_url_postgres}} cargo build
    DATABASE_URL={{db_url_postgres}} ./target/debug/rauthy test &
    sleep 1
    PID=$(echo "$!")
    echo $PID > {{test_pid_file}}
    echo "PID: $PID"

    DATABASE_URL={{db_url_postgres}} cargo test
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
          mkdir -p "$folder"
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
    "templates/html/error/*.html"
    "templates/html/error/error/*.html"
    "templates/html/error/error/error/*.html"
    "templates/html/oidc/*.html"
    "templates/html/users/*.html"
    "templates/html/users/{id}/reset/*.html"
    "templates/html/users/{id}/email_confirm/*.html"
    )
    for folder in "${PAGES[@]}"; do
        for html in $folder; do
          # set correct document language
          sd 'lang="en"' 'lang="{{{{ lang }}"' $html
          # for pre-rendering colors
          sd '#6b3d99;' '{{{{ col_act1 }};' $html
          sd '#714d99;' '{{{{ col_act1a }};' "$html"
          sd '#388c51;' '{{{{ col_act2 }};' "$html"
          sd '#4d8c62;' '{{{{ col_act2a }};' "$html"
          sd '#3d5d99;' '{{{{ col_acnt }};' "$html"
          sd '#36486b;' '{{{{ col_acnta }};' "$html"
          sd '#43993d;' '{{{{ col_ok }};' "$html"
          sd '#993d49;' '{{{{ col_err }};' "$html"
          sd '#545454;' '{{{{ col_glow }};' "$html"
          sd '#b2b2b2;' '{{{{ col_gmid }};' "$html"
          sd '#f2f2f2;' '{{{{ col_ghigh }};' "$html"
          sd '#383838;' '{{{{ col_text }};' "$html"
          sd '#f7f7f7;' '{{{{ col_bg }};' "$html"
          # for the nonce in the CSP for script files
          #sed -i 's/<link /<link nonce="{{{{ nonce }}" /g' "$html"
          #sed -i 's/<script>/<script nonce="{{{{ nonce }}">/g' "$html"
        done;
    done


# builds the rauthy book
build-docs:
    #!/usr/bin/env bash
    set -euxo pipefail
    cd rauthy-book
    mdbook build -d ../docs


# builds the whole application in release mode
build: test-full
    #!/usr/bin/env bash
    set -euxo pipefail

    cargo clean

    cargo clippy --features sqlite -- -D warnings
    cross build --release --target x86_64-unknown-linux-musl --features sqlite
    cp target/x86_64-unknown-linux-musl/release/rauthy out/rauthy-sqlite-amd64

    cargo clean
    cross build --release --target aarch64-unknown-linux-musl --features sqlite
    cp target/aarch64-unknown-linux-musl/release/rauthy out/rauthy-sqlite-arm64


# builds the whole application in release mode
build-postgres: test-postgres
    #!/usr/bin/env bash
    set -euxo pipefail

    cargo clean

    cargo clippy -- -D warnings
    cross build --release --target x86_64-unknown-linux-musl
    cp target/x86_64-unknown-linux-musl/release/rauthy out/rauthy-postgres-amd64

    cargo clean
    cross build --release --target aarch64-unknown-linux-musl
    cp target/aarch64-unknown-linux-musl/release/rauthy out/rauthy-postgres-arm64


# makes sure everything is fine
is-clean:
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

    # TODO the check-in sqlx preparations seem to bug this
    # make sure git is clean
    #git diff --quiet || exit 1

    git tag "v$TAG"
    git push origin "v$TAG"


# publishes the application images - full pipeline incl clippy and testing
publish: pull-latest-cross build-docs build-ui fmt build build-postgres
    #!/usr/bin/env bash
    set -euxo pipefail

    # build and push sqlite version
    docker buildx build \
          -t ghcr.io/sebadob/rauthy:$TAG-lite \
           --platform linux/amd64,linux/arm64 \
           --build-arg="DB=sqlite" \
           --no-cache \
           --push \
           .

    # build and push postgres version
    docker buildx build \
          -t ghcr.io/sebadob/rauthy:$TAG \
          --platform linux/amd64,linux/arm64 \
          --build-arg="DB=postgres" \
          --no-cache \
          --push \
          .


# publishes the application images - full pipeline incl clippy and testing
publish-latest:
    #!/usr/bin/env bash
    set -euxo pipefail

    # the `latest` image will always point to the postgres x86 version, which is used the most (probably)
    docker pull ghcr.io/sebadob/rauthy:$TAG
    docker tag ghcr.io/sebadob/rauthy:$TAG ghcr.io/sebadob/rauthy:latest
    docker push ghcr.io/sebadob/rauthy:latest


# should be run before submitting a PR to make sure everything is fine
verify: build-ui fmt test-full
    #!/usr/bin/env bash
    set -euxo pipefail
    cargo clippy --features sqlite -- -D warnings

    echo "all good"
