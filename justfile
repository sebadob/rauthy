set dotenv-load
set positional-arguments

export TAG := `cat Cargo.toml | grep '^version =' | cut -d " " -f3 | xargs`
export TODAY := `date +%Y%m%d`
export DEV_HOST := `echo $PUB_URL | cut -d':' -f1`
export USER :=  `echo "$(id -u):$(id -g)"`

docker := `echo ${DOCKER:-docker}`
map_docker_user := if docker == "podman" { "" } else { "-u $USER" }
cargo_home := `echo ${CARGO_HOME:-$HOME/.cargo}`

arch := if arch() == "x86_64" { "amd64" } else { "arm64" }

builder_image := "ghcr.io/sebadob/rauthy-builder"
builder_tag_date := "20240830"

container_mailcrab := "rauthy-mailcrab"
container_postgres := "rauthy-db-postgres"
container_test_backend := "rauthy-test-backend"
container_cargo_registry := "/home/.cargo/registry"

db_url_sqlite := "sqlite:data/rauthy.db"
db_url_postgres := "postgresql://rauthy:123SuperSafe@$DEV_HOST:5432/rauthy"

## All scripts in the file with a leading `_` are meant for internal use only.
## The ones with 2 `__` in front are meant to be executed inside a dev / builder container only,
## as they require specific tooling and environmental setup.

[private]
default:
    @just --list

_run +args:
    @{{docker}} run --rm -it \
      -v {{cargo_home}}/registry:{{container_cargo_registry}} \
      -v {{invocation_directory()}}/:/work/ \
      {{map_docker_user}} \
      -e DATABASE_URL={{db_url_sqlite}} \
      --net host \
      --name rauthy \
      {{builder_image}}:{{arch}}-{{builder_tag_date}} {{args}}

_run-pg +args:
    @{{docker}} run --rm -it \
      -v {{cargo_home}}/registry:{{container_cargo_registry}} \
      -v {{invocation_directory()}}/:/work/ \
      {{map_docker_user}} \
      -e DATABASE_URL={{db_url_postgres}} \
      --net host \
      --name rauthy-postgres \
      {{builder_image}}:{{arch}}-{{builder_tag_date}} {{args}}

_run-ui +args:
    @{{docker}} run --rm -it \
      -v {{cargo_home}}/registry:{{container_cargo_registry}} \
      -v {{invocation_directory()}}/:/work/ \
      -e npm_config_cache=/work/.npm_cache \
      {{map_docker_user}} \
      --net host \
      -w/work/frontend \
      --name rauthy-ui \
      {{builder_image}}:{{arch}}-{{builder_tag_date}} {{args}}

_run-bg +args:
    @{{docker}} run --rm -d \
      -v {{cargo_home}}/registry:{{container_cargo_registry}} \
      -v {{invocation_directory()}}/:/work/ \
      {{map_docker_user}} \
      -e DATABASE_URL={{db_url_sqlite}} \
      --net host \
      --name {{container_test_backend}} \
      {{builder_image}}:{{arch}}-{{builder_tag_date}} {{args}}

_run-bg-pg +args:
    @{{docker}} run --rm -d \
      -v {{cargo_home}}/registry:{{container_cargo_registry}} \
      -v {{invocation_directory()}}/:/work/ \
      {{map_docker_user}} \
      -e DATABASE_URL={{db_url_postgres}} \
      --net host \
      --name {{container_test_backend}} \
      {{builder_image}}:{{arch}}-{{builder_tag_date}} {{args}}

# start the backend containers for local dev
@backend:
    just mailcrab-start || echo ">>> Mailcrab is already running - nothing to do"
    just postgres-start || echo ">>> Postgres is already running - nothing to do"
    just prepare

# stop mailcrab and postgres docker containers
@backend-stop:
    just postgres-stop || echo ">>> Postgres is not running - nothing to do"
    just mailcrab-stop || echo ">>> Mailcrab is not running - nothing to do"
    echo "Trying to cleanup orphaned containers"
    {{docker}} rm container rauthy || echo ">>> No orphaned 'rauthy' container found"

@clean:
    just _run cargo clean

# Creates a new Root + Intermediate CA for development and testing TLS certificates
create-root-ca:
    # Password for both root and intermediate dev CA is always: 123SuperMegaSafe

    # The nioca container runs with 10001:10001 uid:gid
    #chmod 0766 tls/ca

    # Root CA
    {{docker}} run --rm -it -v ./tls/ca:/ca -u $(id -u ${USER}):$(id -g ${USER}) \
          ghcr.io/sebadob/nioca \
          x509 \
          --stage root \
          --clean

    # Intermediate CA
    {{docker}} run --rm -it -v ./tls/ca:/ca -u $(id -u ${USER}):$(id -g ${USER}) \
          ghcr.io/sebadob/nioca \
          x509 \
          --stage intermediate

    cp tls/ca/x509/intermediate/ca-chain.pem tls/ca-chain.pem


# Create a new End Entity TLS certificate for development and testing
# Intermediate CA DEV password: 123SuperMegaSafe
create-end-entity-tls:
    # create the new certificate
    {{docker}} run --rm -it -v ./tls/ca:/ca {{map_docker_user}} \
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
    {{docker}} buildx create --name rauthy_builder --bootstrap --use
    {{docker}} buildx inspect rauthy_builder


# Execute an `npm install` for the frontend inside the container. Only needed for the first setup or after an update.
npm-install:
    just _run-ui npm install

# Starts mailcrab
mailcrab-start:
    {{docker}} run -d \
      --net host \
      --name {{container_mailcrab}} \
      --restart unless-stopped \
      docker.io/marlonb/mailcrab

# Stops mailcrab
mailcrab-stop:
    {{docker}} stop {{container_mailcrab}}
    {{docker}} rm {{container_mailcrab}}


# Starts mailcrab
postgres-start:
    {{docker}} run -d \
      -v {{invocation_directory()}}/postgres/init-script:/docker-entrypoint-initdb.d \
      -v {{invocation_directory()}}/postgres/sql-scripts:/scripts/ \
      -e POSTGRES_PASSWORD=123SuperSafe \
      --net host \
      --name {{container_postgres}} \
      --restart unless-stopped \
      docker.io/library/postgres:16.2-alpine

    sleep 3
    just migrate-postgres

# Stops mailcrab
postgres-stop:
    {{docker}} stop {{container_postgres}}
    {{docker}} rm {{container_postgres}}


# Just uses `cargo fmt --all`
fmt:
    @just _run cargo fmt --all


# clippy with sqlite features
clippy:
    #!/usr/bin/env bash
    set -euxo pipefail
    clear
    just _run cargo clippy


# clippy with postgres features
clippy-postgres:
    #!/usr/bin/env bash
    set -euxo pipefail
    clear
    just _run-pg cargo clippy --features postgres


# re-create and migrate the sqlite database with sqlx
migrate:
    #!/usr/bin/env bash
    set -euxo pipefail

    just _run mkdir -p data/
    just _run rm -f data/rauthy.db*
    just _run sqlx database create
    just _run sqlx migrate run --source migrations/sqlite


# migrate the postgres database with sqlx
migrate-postgres:
    @just _run-pg sqlx migrate run --source migrations/postgres


# runs any of: none (sqlite), postgres, ui
run ty="sqlite":
    #!/usr/bin/env bash
    set -euxo pipefail
    clear

    if [[ {{ty}} == "postgres" ]]; then
      just _run-pg cargo run --features postgres
    elif [[ {{ty}} == "ui" ]]; then
      just _run-ui npm run dev -- --host=0.0.0.0
    elif [[ {{ty}} == "sqlite" ]]; then
      just _run cargo run
    fi


# prints out the currently set version
version:
    #!/usr/bin/env bash
    echo "v$TAG"


# prepare DB migrations for SQLite for compile-time checked queries
prepare: migrate
    just _run cargo sqlx prepare --workspace


# run `sqlx prepare` locally to get rid of `sqlx::query!()` warnings
prepare-local: migrate
    cargo sqlx prepare --workspace


# prepare DB migrations for Postgres for compile-time checked queries
prepare-postgres: migrate-postgres
    just _run-pg cargo sqlx prepare --workspace -- --features postgres


# only starts the backend in test mode with sqlite database for easier test debugging
test-backend: test-backend-stop migrate prepare
    #!/usr/bin/env bash
    set -euxo pipefail
    clear

    just _run cargo build
    {{docker}} run --rm -it \
      -v {{cargo_home}}/registry:{{container_cargo_registry}} \
      -v {{invocation_directory()}}/:/work/ \
      {{map_docker_user}} \
      -e DATABASE_URL={{db_url_sqlite}} \
      --net host \
      --name {{container_test_backend}} \
      {{builder_image}}:{{arch}}-{{builder_tag_date}} cargo run test


# stops a possibly running test backend that may have spawned in the background for integration tests
test-backend-stop:
    {{docker}} stop {{container_test_backend}} || exit 0

# runs a single test with sqlite - needs the backend being started manually
test *test:
    #!/usr/bin/env bash
    set -euxo pipefail
    clear

    just _run cargo test {{test}}


# runs the full set of tests with in-memory sqlite
test-sqlite *test: test-backend-stop migrate prepare
    #!/usr/bin/env bash
    set -euxo pipefail
    clear

    just _run cargo build
    just _run-bg ./target/debug/rauthy test

    just _run cargo test {{test}}
    just test-backend-stop


# runs the full set of tests with postgres
test-postgres test="": test-backend-stop postgres-stop postgres-start prepare-postgres
    #!/usr/bin/env bash
    set -euxo pipefail
    clear

    just _run-pg cargo build --features postgres
    just _run-bg-pg ./target/debug/rauthy test

    just _run-pg cargo test --features postgres {{test}}
    just test-backend-stop


# builds the frontend and exports to static html
build-ui:
    #!/usr/bin/env bash
    set -euxo pipefail
    just _run-ui just __build-ui

# this recipe will only work inside the builder container
__build-ui:
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
        done;
    done


# builds the rauthy book
build-docs:
    #!/usr/bin/env bash
    set -euxo pipefail
    just _run just __build-docs
    git add docs


# builds the rauthy book
__build-docs:
    #!/usr/bin/env bash
    set -euxo pipefail
    cd book
    mdbook build -d ../docs
    cd ..


# mode = release or debug / no-test = no-test or do-test / image = name for the final image
build mode="release" no-test="test" image="ghcr.io/sebadob/rauthy": build-ui
    #!/usr/bin/env bash
    set -euxo pipefail

    # sqlite
    #if [ {{no-test}} != "no-test" ]; then
    #    echo "make sure clippy is fine with sqlite"
    #    just _run cargo clippy -- -D warnings
    #    echo "run tests against sqlite"
    #    just test-sqlite
    #else
    #    just prepare
    #fi

    # make sure any big testing sqlite backups are cleaned up to speed up docker build
    rm -rf data/backup

    echo "build sqlite release"
    {{docker}} buildx build \
        -t {{image}}:$TAG-lite \
        --platform linux/amd64,linux/arm64 \
        --build-arg="IMAGE={{builder_image}}" \
        --build-arg="IMAGE_DATE={{builder_tag_date}}" \
        --build-arg="DATABASE_URL={{db_url_sqlite}}" \
        --build-arg="FEATURES=default" \
        --build-arg="MODE={{mode}}" \
        --push \
        .

    # postgres
    if [ {{no-test}} != "no-test" ]; then
        # restart postgres to clean it up for the tests
        just postgres-stop || echo ">>> Postgres is not running - nothing to do"
        just postgres-start || echo ">>> Postgres is already running - nothing to do"

        echo "make sure clippy is fine with postgres"
        just _run-pg cargo clippy --features postgres -- -D warnings
        echo "run tests against postgres"
        just test-postgres
    else
        just prepare-postgres
    fi

    echo "build postgres release"
    {{docker}} buildx build \
        -t {{image}}:$TAG \
        --platform linux/amd64,linux/arm64 \
        --build-arg="IMAGE={{builder_image}}" \
        --build-arg="IMAGE_DATE={{builder_tag_date}}" \
        --build-arg="DATABASE_URL={{db_url_postgres}}" \
        --build-arg="FEATURES=postgres" \
        --build-arg="MODE={{mode}}" \
        --push \
        .


# specify a custom image for building locally and change `push` to `load` to not push but only load it into your local docker context
build-builder image="ghcr.io/sebadob/rauthy-builder" push="push":
    #!/usr/bin/env bash
    set -euxo pipefail

    {{docker}} pull ghcr.io/cross-rs/x86_64-unknown-linux-musl:edge
    {{docker}} buildx build \
          -t {{image}}:amd64-$TODAY \
          -f Dockerfile_builder \
          --platform linux/amd64 \
          --build-arg="IMAGE=ghcr.io/cross-rs/x86_64-unknown-linux-musl:edge" \
          --no-cache \
          --{{push}} \
          .

    {{docker}} pull ghcr.io/cross-rs/aarch64-unknown-linux-musl:edge
    {{docker}} buildx build \
          -t {{image}}:arm64-$TODAY \
          -f Dockerfile_builder \
          --platform linux/arm64 \
          --build-arg="IMAGE=ghcr.io/cross-rs/aarch64-unknown-linux-musl:edge" \
          --no-cache \
          --{{push}} \
          .


# makes sure everything is fine
is-clean:
    #!/usr/bin/env bash
    set -euxo pipefail

    # exit early if clippy emits warnings
    just _run cargo clippy -- -D warnings
    just _run-pg cargo clippy --features postgres -- -D warnings

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


# publishes the application images - full pipeline incl clippy and testing  you can provide a custom image name as variable
publish: build-docs fmt build
    #!/usr/bin/env bash
    set -euxo pipefail


# publishes the application images - full pipeline incl clippy and testing
publish-latest:
    #!/usr/bin/env bash
    set -euxo pipefail

    # the `latest` image will always point to the postgres x86 version, which is used the most (probably)
    {{docker}} pull ghcr.io/sebadob/rauthy:$TAG
    {{docker}} tag ghcr.io/sebadob/rauthy:$TAG ghcr.io/sebadob/rauthy:latest
    {{docker}} push ghcr.io/sebadob/rauthy:latest


# should be run before submitting a PR to make sure everything is fine
pre-pr-checks: build-ui fmt test-sqlite test-postgres clippy clippy-postgres
    #!/usr/bin/env bash
    set -euxo pipefail

# does a `cargo update` + `npm update` for the UI
update-deps:
    just _run cargo update
    just _run-ui npm update
