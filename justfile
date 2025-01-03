set dotenv-load := true
set positional-arguments := true

export TAG := `cat Cargo.toml | grep '^version =' | cut -d " " -f3 | xargs`
export TODAY := `date +%Y%m%d`
export DEV_HOST := `echo ${PUB_URL:-localhost:8080} | cut -d':' -f1`
export USER := `echo "$(id -u):$(id -g)"`
arch := if arch() == "x86_64" { "amd64" } else { "arm64" }
docker := `echo ${DOCKER:-docker}`
map_docker_user := if docker == "podman" { "" } else { "-u $USER" }
npm := `echo ${NPM:-npm}`
cargo_home := `echo ${CARGO_HOME:-$HOME/.cargo}`
builder_image := "ghcr.io/sebadob/rauthy-builder"
builder_tag_date := "20241118"
container_network := "rauthy-dev"
container_mailcrab := "rauthy-mailcrab"
container_postgres := "rauthy-db-postgres"
container_cargo_registry := "/usr/local/cargo/registry"
file_test_pid := ".test_pid"
postgres := "HIQLITE=false"

[private]
default:
    @just --list

# Execute after a fresh clone of the repo. It will fully set up your dev env.
setup:
    #!/usr/bin/env bash
    set -euxo pipefail
    clear

    echo "Installing necessary tools: just, sd, mdbook, mdbook-admonish, sqlx-cli"
    readarray -t cargopkgs < <(cargo install --list | cut -d' ' -f1 | sort | uniq)
    for pkg in just sd mdbook mdbook-admonish; do
        if command -v "$pkg" &>/dev/null; then
            if printf '%s\0' "${cargopkgs[@]}" | grep -qw "$pkg"; then
                cargo install "$pkg"
            fi
        else
            cargo install "$pkg"
        fi
    done
    if command -v sqlx &>/dev/null; then
        if printf '%s\0' "${cargopkgs[@]}" | grep -qw sqlx-cli; then
            cargo install sqlx-cli --no-default-features --features rustls,sqlite,postgres
        fi
    else
        cargo install sqlx-cli --no-default-features --features rustls,sqlite,postgres
    fi

    echo "npm install to set up the frontend"
    cd frontend/
    {{ npm }} install
    cd ..

    if ! {{ docker }} network inspect rauthy-dev; then
      echo "Creating rauthy-dev container network"
      {{ docker }} network create rauthy-dev
    fi

    echo "Building the UI and static HTML"
    just build-ui

    echo "Starting Postgres and Mailcrab containers"
    just backend-start

    cargo build

# start the backend containers for local dev
@backend-start:
    just mailcrab-start || echo ">>> Mailcrab is already running - nothing to do"
    just postgres-start || echo ">>> Postgres is already running - nothing to do"

# stop mailcrab and postgres docker containers
@backend-stop:
    just postgres-stop || echo ">>> Postgres is not running - nothing to do"
    just mailcrab-stop || echo ">>> Mailcrab is not running - nothing to do"
    echo "Trying to cleanup orphaned containers"
    {{ docker }} rm container rauthy || echo ">>> No orphaned 'rauthy' container found"

# Creates a new Root + Intermediate CA for development and testing TLS certificates
create-root-ca:
    # Password for both root and intermediate dev CA is always: 123SuperMegaSafe

    # The nioca container runs with 10001:10001 uid:gid
    #chmod 0766 tls/ca

    # Root CA
    {{ docker }} run --rm -it -v ./tls/ca:/ca -u $(id -u ${USER}):$(id -g ${USER}) \
          ghcr.io/sebadob/nioca \
          x509 \
          --stage root \
          --clean

    # Intermediate CA
    {{ docker }} run --rm -it -v ./tls/ca:/ca -u $(id -u ${USER}):$(id -g ${USER}) \
          ghcr.io/sebadob/nioca \
          x509 \
          --stage intermediate

    cp tls/ca/x509/intermediate/ca-chain.pem tls/ca-chain.pem

# Create a new End Entity TLS certificate for development and testing

# Intermediate CA DEV password: 123SuperMegaSafe
create-end-entity-tls:
    # create the new certificate
    {{ docker }} run --rm -it -v ./tls/ca:/ca {{ map_docker_user }} \
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

# This may be executed if you don't have a local `docker buildx` setup and want to create a release container build
docker-buildx-setup:
    #!/usr/bin/env bash
    set -euxo pipefail
    clear

    if ! {{ docker }} buildx inspect rauthy_builder; then
        # create 'rauthy_builder' buildx instance
        {{ docker }} buildx create --name rauthy_builder --bootstrap --use
        {{ docker }} buildx inspect rauthy_builder
    fi

# Starts mailcrab
mailcrab-start:
    {{ docker }} run -d \
      --net {{ container_network }} \
      -p 1025:1025 \
      -p 1080:1080 \
      --name {{ container_mailcrab }} \
      --restart unless-stopped \
      docker.io/marlonb/mailcrab

# Stops mailcrab
mailcrab-stop:
    {{ docker }} stop {{ container_mailcrab }}
    {{ docker }} rm {{ container_mailcrab }}

# Starts mailcrab
postgres-start:
    {{ docker }} run -d \
      -e POSTGRES_USER=rauthy \
      -e POSTGRES_PASSWORD=123SuperSafe \
      -e POSTGRES_DB=rauthy \
      --net {{ container_network }} \
      -p 5432:5432 \
      --name {{ container_postgres }} \
      --restart unless-stopped \
      docker.io/library/postgres:17.2-alpine

    sleep 3
    just migrate-postgres

# Stops mailcrab
postgres-stop:
    {{ docker }} stop {{ container_postgres }}
    {{ docker }} rm {{ container_postgres }}

# Just uses `cargo fmt --all`
fmt:
    cargo fmt --all

clippy:
    #!/usr/bin/env bash
    set -euxo pipefail
    clear
    cargo clippy

# delete the local hiqlite database
delete-hiqlite:
    #!/usr/bin/env bash
    set -euxo pipefail

    mkdir -p data/
    rm -rf data/logs
    rm -rf data/state_machine

# migrate the postgres database with sqlx
migrate-postgres:
    sqlx migrate run --source migrations/postgres

# runs any of: none (hiqlite), postgres, ui
run ty="hiqlite":
    #!/usr/bin/env bash
    set -euxo pipefail
    clear

    if [[ {{ ty }} == "postgres" ]]; then
      {{ postgres }} cargo run
    elif [[ {{ ty }} == "ui" ]]; then
      cd frontend
      {{ npm }} run dev -- --host=0.0.0.0
    elif [[ {{ ty }} == "hiqlite" ]]; then
      cargo run
    fi

# prints out the currently set version
version:
    #!/usr/bin/env bash
    echo "v$TAG"

# only starts the backend in test mode with hiqlite database for easier test debugging
test-backend: test-backend-stop delete-hiqlite
    #!/usr/bin/env bash
    set -euxo pipefail
    clear
    cargo run test

# stops a possibly running test backend that may have spawned in the background for integration tests
test-backend-stop:
    #!/usr/bin/env bash
    if [ -f {{ file_test_pid }} ]; then
      kill $(cat {{ file_test_pid }})
      rm {{ file_test_pid }}
    fi

    # we need to sleep 5 seconds because the lockfiles will take 4.5 seconds to be deleted
    sleep 5

# runs a single test with hiqlite - needs the backend being started manually
test *test:
    #!/usr/bin/env bash
    set -euxo pipefail
    clear
    cargo test {{ test }}

# runs the full set of tests with sqlite
test-hiqlite *test: test-backend-stop delete-hiqlite
    #!/usr/bin/env bash
    clear

    cargo build
    ./target/debug/rauthy test &
    echo $! > {{ file_test_pid }}

    # a fresh Hiqlite instance needs ~1 - 1.5 seconds for the raft initialization
    sleep 3

    if cargo test {{ test }}; then
      echo "All SQLite tests successful"
      just test-backend-stop
    else
      echo "Failed Tests"
      just test-backend-stop
      exit 1
    fi

# runs the full set of tests with postgres
test-postgres test="": test-backend-stop postgres-stop postgres-start
    #!/usr/bin/env bash
    clear

    cargo build
    {{ postgres }} ./target/debug/rauthy test &
    echo $! > {{ file_test_pid }}

    sleep 1

    if {{ postgres }} cargo test {{ test }}; then
      echo "All SQLite tests successful"
      just test-backend-stop
    else
      echo "Failed Tests"
      just test-backend-stop
      exit 1
    fi

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
    {{ npm }} run build
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
          #sd 'lang="en"' 'lang="{{{{ lang }}"' $html
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
        done;
    done

# builds the rauthy book
build-docs:
    #!/usr/bin/env bash
    set -euxo pipefail
    cd book
    mdbook build -d ../docs
    git add ../docs/*

# Build the final container image.
build image="ghcr.io/sebadob/rauthy": build-ui
    #!/usr/bin/env bash
    set -euxo pipefail

    # make sure base image is up to date
    docker pull gcr.io/distroless/cc-debian12:nonroot

    mkdir -p out/empty

    # IMPORTANT: We can't use `cross` for the x86 build because it uses a way too old
    # `gcc`which has a known `memcmp` issue:
    # https://github.com/cross-rs/cross/security/advisories/GHSA-2r9g-5qvw-fgmf
    # https://gcc.gnu.org/bugzilla/show_bug.cgi?id=95189
    {{ docker }} run \
      -v {{ cargo_home }}/registry:{{ container_cargo_registry }} \
      -v {{ invocation_directory() }}/:/work/ \
      -w /work \
      {{ map_docker_user }} \
      --net {{ container_network }} \
      -e DATABASE_URL=postgresql://rauthy:123SuperSafe@rauthy-db-postgres:5432/rauthy \
      {{ builder_image }}:{{ builder_tag_date }} \
      cargo build --release --target x86_64-unknown-linux-gnu
    cp target/x86_64-unknown-linux-gnu/release/rauthy out/rauthy_amd64

    # TODO here is potential to unify both images into a `dockerx` build which could
    # potentially speed up the build process in exchange for more complex images.
    # Depending on the target arch, the `--target` would be added dynamically inside
    # the container.
    {{ docker }} run \
      -v {{ cargo_home }}/registry:{{ container_cargo_registry }} \
      -v {{ invocation_directory() }}/:/work/ \
      -w /work \
      {{ map_docker_user }} \
      --net {{ container_network }} \
      -e DATABASE_URL=postgresql://rauthy:123SuperSafe@rauthy-db-postgres:5432/rauthy \
      {{ builder_image }}:{{ builder_tag_date }} \
      cargo build --release --target aarch64-unknown-linux-gnu
    cp target/aarch64-unknown-linux-gnu/release/rauthy out/rauthy_arm64

    {{ docker }} buildx build \
        -t {{ image }}:$TAG \
        --platform linux/amd64,linux/arm64 \
        --push \
        .

    rm -rf out/

# specify a custom image for building locally and change `push` to `load` to not push but only load it into your local docker context
build-builder image="ghcr.io/sebadob/rauthy-builder" push="push":
    #!/usr/bin/env bash
    set -euxo pipefail

    {{ docker }} build \
          -t {{ image }}:$TODAY \
          -f Dockerfile_builder \
          --build-arg="IMAGE=rust:1.82-bookworm" \
          .
    {{ docker }} push {{ image }}:$TODAY

# makes sure everything is fine
is-clean:
    #!/usr/bin/env bash
    set -euxo pipefail

    # exit early if clippy emits warnings
    cargo clippy postgres -- -D warnings

    # make sure everything has been committed
    git diff --exit-code

    echo all good

# sets a new git tag and pushes it
release:
    #!/usr/bin/env bash
    set -euxo pipefail

    # make sure git is clean
    git diff --quiet || exit 1

    git tag "v$TAG"
    git push origin "v$TAG"

# publishes the application images - full pipeline incl clippy and testing you can provide a custom image name as variable
publish: build-docs fmt test-hiqlite test-postgres build
    #!/usr/bin/env bash
    set -euxo pipefail

# publishes the application images - full pipeline incl clippy and testing
publish-latest:
    #!/usr/bin/env bash
    set -euxo pipefail

    # the `latest` image will always point to the postgres x86 version, which is used the most (probably)
    {{ docker }} pull ghcr.io/sebadob/rauthy:$TAG
    {{ docker }} tag ghcr.io/sebadob/rauthy:$TAG ghcr.io/sebadob/rauthy:latest
    {{ docker }} push ghcr.io/sebadob/rauthy:latest

# should be run before submitting a PR to make sure everything is fine
pre-pr-checks: build-ui fmt test-hiqlite test-postgres clippy
    #!/usr/bin/env bash
    set -euxo pipefail

# does a `cargo update` + `npm update` for the UI
update-deps:
    #!/usr/bin/env bash
    cargo update
    cd frontend
    {{ npm }} update
