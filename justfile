set dotenv-load := true
set positional-arguments := true

export TAG := `cat Cargo.toml | grep '^version =' | cut -d " " -f3 | xargs`
export TODAY := `date +%Y%m%d`
export DEV_HOST := `echo ${PUB_URL:-localhost:8080} | cut -d':' -f1`
export USER := `echo "$(id -u):$(id -g)"`
arch := if arch() == "x86_64" { "amd64" } else { "arm64" }
is_mac_container := `if test -f /usr/local/bin/container; then echo true; else echo false; fi`
docker := `which docker || which podman || which container || echo 'no-container-runtime-found'`
map_docker_user := if docker == "podman" { "" } else { "-u $USER" }
npm := `echo ${NPM:-npm}`
cargo_home := `echo ${CARGO_HOME:-$HOME/.cargo}`
node_image := "node:22"
builder_image := "ghcr.io/sebadob/rauthy-builder"
builder_tag_date := "20251203"
container_mailcrab := "rauthy-mailcrab"
container_postgres := "rauthy-db-postgres"
container_cargo_registry := "/usr/local/cargo/registry"
file_test_pid := ".test_pid"
test_env_vars := "PUB_URL=localhost:8081 RP_ORIGIN=http://localhost:8081"
jemalloc_conf := "MALLOC_CONF=abort_conf:true,narenas:8,tcache_max:4096,dirty_decay_ms:5000,muzzy_decay_ms:5000"
postgres := "HIQLITE=false"

[private]
default:
    @just --list

# Execute after a fresh clone of the repo. It will fully set up your dev env.
setup:
    #!/usr/bin/env bash
    set -euxo pipefail
    clear

    echo "npm install to set up the frontend"
    cd frontend/
    {{ npm }} install
    cd ..

    echo "Installing wasm-pack"
    cargo install wasm-pack mdbook mdbook-admonish

    echo "Building WASM modules"
    just build-wasm

# start the backend containers for local dev
@dev-env-start:
    just mailcrab-start
    just postgres-start

# stop mailcrab and postgres docker containers
@dev-env-stop:
    just postgres-stop
    just mailcrab-stop

# rm mailcrab and postgres docker containers
@dev-env-rm: dev-env-stop
    just mailcrab-rm

    echo "Trying to cleanup orphaned containers"
    {{ docker }} rm container rauthy || echo ">>> No orphaned 'rauthy' container found"

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
    #!/usr/bin/env bash

    if {{ is_mac_container }} && {{ docker }} ls -a | grep "^{{ container_mailcrab }} "; then
        {{ docker }} start {{ container_mailcrab }}
    elif !{{ is_mac_container }} && {{ docker }} ps -a --format '{{{{ .Names }}' | grep -q "^{{ container_mailcrab }}$"; then
        {{ docker }} start {{ container_mailcrab }}
    else
        {{ docker }} run -d \
            -p 1025:1025 \
            -p 1080:1080 \
            --name {{ container_mailcrab }} \
            docker.io/marlonb/mailcrab || echo ">>> Mailcrab is already running - nothing to do"
    fi

# Stops mailcrab
mailcrab-stop:
    {{ docker }} stop {{ container_mailcrab }} || echo ">>> Mailcrab is not running - nothing to do"

mailcrab-rm:
    {{ docker }} rm {{ container_mailcrab }} || echo ">>> Mailcrab does not exists - nothing to do"

# Starts mailcrab
postgres-start:
    #!/usr/bin/env bash

    if {{ is_mac_container }} && {{ docker }} ls -a | grep "^{{ container_postgres }} "; then
        {{ docker }} start {{ container_postgres }}
    elif !{{ docker }} ps -a --format '{{{{ .Names }}' | grep -q "^{{ container_postgres }}$"; then
        {{ docker }} start {{ container_postgres }}
    else
        {{ docker }} run -d \
            -e POSTGRES_USER=rauthy \
            -e POSTGRES_PASSWORD=123SuperSafe \
            -e POSTGRES_DB=rauthy \
            -p 5432:5432 \
            --name {{ container_postgres }} \
            docker.io/library/postgres:17.2-alpine || echo ">>> Postgres is already running - nothing to do"
    fi

    while ! curl localhost:5432 2>&1 | grep 'Empty reply from server' ; do
        echo 'Waiting until Postgres is up and running'
        sleep 1
    done
    echo 'Postgres is ready'

# Stops mailcrab
postgres-stop:
    {{ docker }} stop {{ container_postgres }} || echo ">>> Postgres is not running - nothing to do"

postgres-rm:
    {{ docker }} rm {{ container_postgres }} || echo ">>> Postgres does not exist - nothing to do"

# Starts nginx for `/forward_auth` testing
nginx-start:
    #!/usr/bin/env bash
    set -euxo pipefail

    rm assets/nginx/access.log
    touch assets/nginx/access.log

    {{ docker }} run -it --rm \
            -v ./assets/nginx/conf.d:/etc/nginx/conf.d:ro \
            -v ./assets/nginx/access.log:/var/log/nginx/access.log \
            --network host \
            --name nginx-rauthy-test \
            nginx

# Just uses `cargo fmt --all`
fmt:
    #!/usr/bin/env bash
    set -euxo pipefail

    cargo fmt --all
    cd frontend
    npm run format

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
    rm -rf data/logs_cache
    rm -rf data/state_machine
    rm -rf data/state_machine_cache
    rm -rf data/node_*

# runs any of: none (hiqlite), postgres, ui
run ty="hiqlite" node_id="1":
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
    elif [[ {{ ty }} == "node" ]]; then
      if [[ {{ node_id }} == "1" ]]; then
        LISTEN_PORT_HTTP=8001 PUB_URL="localhost:8001" HQL_DATA_DIR=data/node_1 HQL_NODE_ID=1 cargo run -- 1
      elif [[ {{ node_id }} == "2" ]]; then
        LISTEN_PORT_HTTP=8002 PUB_URL="localhost:8002" HQL_DATA_DIR=data/node_2 HQL_NODE_ID=2 cargo run -- 2
      elif [[ {{ node_id }} == "3" ]]; then
        LISTEN_PORT_HTTP=8003 PUB_URL="localhost:8003" HQL_DATA_DIR=data/node_3 HQL_NODE_ID=3 cargo run -- 3
      else
        echo "Only node_id from 1 - 3 supported"
      fi
    fi

# runs (and stops) Postgres, Mailcrab, normal `just run` command
watch ty="hiqlite":
    #!/usr/bin/env bash

    just mailcrab-start

    # In case of Postgres, the container will not be up that quickly
    if [[ {{ ty }} == "postgres" ]]; then
      just postgres-start
      sleep 2
      command="{{ postgres }} cargo run"
    elif [[ {{ ty }} == "hiqlite" ]]; then
      command="cargo run"
    fi

    watchexec -r -w src -w templates -- $command

    just dev-env-stop

# prints out the currently set version
version:
    #!/usr/bin/env bash
    echo "v$TAG"

# only starts the backend in test mode with hiqlite database for easier test debugging
test-backend: test-backend-stop delete-hiqlite
    #!/usr/bin/env bash
    set -euxo pipefail
    clear
    {{ test_env_vars }} cargo run test

# starts the test backend with memory profiling - expects `heaptrack` to be available
test-backend-heaptrack: test-backend-stop delete-hiqlite
    #!/usr/bin/env bash
    set -euxo pipefail
    clear
    echo "Building a release build with the 'profiling' profile - this will take some time ..."
    RUSTFLAGS=-g cargo build --profile profiling
    #echo "Temporarily removing kernel hardening and elevating ptrace rights until reboot"
    #echo 0 | sudo tee /proc/sys/kernel/yama/ptrace_scope
    #echo 'grant temporary access to performance events until reboot'
    #echo '1' | sudo tee /proc/sys/kernel/perf_event_paranoid
    #echo 'if you get an mmap error, try: sudo sysctl kernel.perf_event_mlock_kb=2048'
    {{ test_env_vars }} heaptrack ./target/profiling/rauthy test

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
    {{ test_env_vars }} ./target/debug/rauthy test &
    echo $! > {{ file_test_pid }}

    while ! curl -s localhost:8081/auth/v1/ping; do
      sleep 1
      echo '>>> Waiting for test-backend to be up and running'
    done

    if cargo test {{ test }}; then
      echo "All SQLite tests successful"
      just test-backend-stop
    else
      echo "Failed Tests"
      just test-backend-stop
      exit 1
    fi

# runs the full set of tests with postgres
test-postgres test="": test-backend-stop postgres-stop postgres-rm delete-hiqlite postgres-start
    #!/usr/bin/env bash
    clear

    cargo build
    # the Hiqlite tests are disk-backed, lets check postgres in-memory to cover this as well
    {{ test_env_vars }} {{ postgres }} HQL_CACHE_STORAGE_DISK=false ./target/debug/rauthy test &
    echo $! > {{ file_test_pid }}

    while ! curl -s localhost:8081/auth/v1/ping; do
      sleep 1
      echo '>>> Waiting for test-backend to be up and running'
    done

    if {{ postgres }} cargo test {{ test }}; then
      echo "All Postgres tests successful"
      just test-backend-stop
    else
      echo "Failed Tests"
      just test-backend-stop
      exit 1
    fi

# builds the frontend and exports to static html, the option `container` will build it inside a container
build-ui where="local":
    #!/usr/bin/env bash
    set -euxo pipefail

    mkdir -p templates/html
    mkdir -p static/v1

    # cleanup old files
    rm -rf static/v1/*
    rm -rf templates/html/*

    if [[ {{ where }} == "container" ]]; then
        {{ docker }} run \
            -v {{ invocation_directory() }}/:/work/ \
            -w /work/frontend \
            {{ map_docker_user }} \
            {{ node_image }} \
            npm run build
    else
        # build the frontend
        cd frontend
        {{ npm }} run build
    fi

# archives static pre-built HTML into `assets/static_html/`
archive-ui: build-ui
    #!/usr/bin/env bash
    set -euxo pipefail

    # cleanup
    mkdir -p assets/static_html
    rm -rf assets/static_html/*

    tar -czf assets/static_html/templates_html.tar.gz templates/html
    tar -czf assets/static_html/static_v1.tar.gz static/v1

    git add assets/static_html/*

# extracts archived UI files into target folders
extract-ui-archive:
    #!/usr/bin/env bash
    set -euxo pipefail

    mkdir static

    rm -rf static/v1
    rm -rf templates/html/

    tar -xf assets/static_html/static_v1.tar.gz static/
    tar -xf assets/static_html/templates_html.tar.gz templates/

# builds the rauthy book
build-docs:
    #!/usr/bin/env bash
    set -euxo pipefail

    cd book
    mdbook build -d ../docs
    git add ../docs/*

# builds a release version with the `profiling` profile for analyzations with `heaptrack`
build-profiling:
    #!/usr/bin/env bash
    set -euxo pipefail
    clear
    echo "Building a release build with the 'profiling' profile - this will take some time ..."
    #RUSTFLAGS=-g cargo build --profile profiling
    RUSTFLAGS=-g {{ jemalloc_conf }} cargo build --profile profiling --features jemalloc
    echo "You can analyze the application via: heaptrack ./target/profiling/rauthy"

# build wasm frontend modules
build-wasm:
    #!/usr/bin/env bash

    cd src/wasm-modules
    rm -rf ../../frontend/src/wasm/*

    wasm-pack build -d ../../frontend/src/wasm/spow --no-pack --out-name spow --features spow
    wasm-pack build -d ../../frontend/src/wasm/md --no-pack --out-name md --features md

# Build the final container image.
build image="ghcr.io/sebadob/rauthy" push="push": build-wasm build-ui
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
        -e JEMALLOC_SYS_WITH_LG_PAGE=16 \
        {{ map_docker_user }} \
        {{ builder_image }}:{{ builder_tag_date }} \
        cargo build --release --target aarch64-unknown-linux-gnu
    cp target/aarch64-unknown-linux-gnu/release/rauthy out/rauthy_arm64

    if [[ {{ push }} == "push" ]]; then
        {{ docker }} buildx build \
            -t {{ image }}:$TAG \
            --platform linux/amd64,linux/arm64 \
            --push \
            .
    else
        {{ docker }} buildx build \
            -t {{ image }}:$TAG \
            --platform linux/amd64,linux/arm64 \
            --load \
            .
    fi

    rm -rf out/

# specify a custom image for building locally and change `push` to `load` to not push but only load it into your local docker context
build-builder image="ghcr.io/sebadob/rauthy-builder" push="push":
    #!/usr/bin/env bash
    set -euxo pipefail

    {{ docker }} build \
          -t {{ image }}:$TODAY \
          -f Dockerfile_builder \
          .
    {{ docker }} push {{ image }}:$TODAY

# execs into a new blank builder image for debugging with local volume mounts
exec-into-new-builder:
    #!/usr/bin/env bash
    set -euxo pipefail

    IMAGE=$(grep ^FROM Dockerfile_builder | cut -d ' ' -f2)

    {{ docker }} run -it --rm \
        -v {{ cargo_home }}/registry:{{ container_cargo_registry }} \
        -v {{ invocation_directory() }}/:/work/ \
        -w /work \
        -e DEBIAN_FRONTEND=noninteractive \
        -e CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-linux-gnu-gcc \
        -e CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc \
        -e PKG_CONFIG_ALLOW_CROSS=1 \
        $IMAGE \
        bash

# makes sure everything is fine
is-clean:
    #!/usr/bin/env bash
    set -euxo pipefail

    cd frontend
    npm run check
    cd ..

    cargo clippy -- -D warnings
    cargo fmt --check

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
publish: build-docs fmt test-hiqlite test-postgres build archive-ui

# publishes the application images - full pipeline incl clippy and testing
publish-latest:
    #!/usr/bin/env bash
    set -euxo pipefail

    {{ docker }} pull ghcr.io/sebadob/rauthy:$TAG
    {{ docker }} tag ghcr.io/sebadob/rauthy:$TAG ghcr.io/sebadob/rauthy:latest
    {{ docker }} push ghcr.io/sebadob/rauthy:latest

# should be run before submitting a PR to make sure everything is fine
pre-pr-checks: build-ui fmt is-clean test-hiqlite test-postgres
    #!/usr/bin/env bash
    set -euxo pipefail

# does a `cargo update` + `npm update` for the UI
update-deps:
    #!/usr/bin/env bash
    cargo update
    cd frontend
    {{ npm }} update
