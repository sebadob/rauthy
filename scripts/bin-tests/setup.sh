#!/usr/bin/env bash
# Stands up a live rauthy instance for the `src/bin/tests` integration suite,
# runs the suite, then tears the instance down.
#
# Why this exists / what the tests require (all discovered empirically):
# - DEBUG build: release rejects the tests' empty User-Agent ("Empty User-Agent not
#   allowed") - the tests' reqwest client sends none.
# - ADMIN_FORCE_MFA=false: the default enforces admin WebAuthn MFA, so full-admin
#   calls from the bootstrap admin session fail with 406.
# - The bootstrap admin must be `init_admin@localhost` / `123SuperSafe` and the
#   `init_client` client must pre-exist with the EXACT secret the tests hardcode
#   (clients.json supports {"Plain": "..."}).
# - Port 8081 (hardcoded in tests/common.rs get_backend_url()).
#
# Known limitation: under a full `--no-fail-fast` run, the login-delay escalation
# (src/service/src/login_delay.rs: >=5 failed logins -> sleep, >=7 -> 60s IP
# blacklist) ratchets because some tests intentionally perform failed logins
# (e.g. test_client_secret). Each test passes in isolation; the full-suite cascade
# is a maintainer-tuning item, not a code defect.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
WORK="${BIN_TEST_WORK_DIR:-/tmp/rauthy-bin-tests}"
PORT="${BIN_TEST_PORT:-8081}"
TARGET="${CARGO_TARGET_DIR:-$ROOT/target}"
case "$TARGET" in /*) ;; *) TARGET="$ROOT/$TARGET" ;; esac
BIN="$TARGET/debug/rauthy"

# --- generated secrets (stable defaults; override via env) -------------------
SECRETS="${SECRETS:-$(openssl rand -base64 32)}"
K1="${BIN_TEST_K1:-$(openssl rand -base64 32)}"
K2="${BIN_TEST_K2:-$(openssl rand -base64 32)}"
ENC_ACTIVE="${BIN_TEST_ENC_ACTIVE:-testkey2}"

# --- bootstrap files ----------------------------------------------------------
mkdir -p "$WORK/bootstrap"
python3 - "$WORK/bootstrap" "$PORT" <<'PYEOF'
import json, sys
out, port = sys.argv[1], sys.argv[2]
clients = [{
    "id": "init_client",
    "name": "Init Client",
    "secret": {"Plain": "LjERi0WSEz1E9OY9KFJaMjlwV1Uf3nuIuOUnJnoJQNm2i7YMjTDMy4PbAKnYRgFy"},
    "enabled": True,
    "confidential": True,
    "flows_enabled": ["authorization_code", "refresh_token", "password", "client_credentials"],
    "redirect_uris": [
        f"http://localhost:{port}/auth/v1/oidc/callback",
        f"http://localhost:{port}/auth/v1/dev/backchannel_logout",
    ],
    "backchannel_logout_uri": f"http://localhost:{port}/auth/v1/dev/backchannel_logout",
    "access_token_alg": "EdDSA",
    "id_token_alg": "EdDSA",
    "auth_code_lifetime": 60,
    "access_token_lifetime": 3600,
    "scopes": ["openid", "profile", "email"],
    "default_scopes": ["email", "openid", "profile"],
    "force_mfa": False,
}]
open(f"{out}/clients.json", "w").write(json.dumps(clients, indent=2))
PYEOF

# --- start rauthy (debug) ------------------------------------------------------
cd "$ROOT"
rm -rf "$WORK/data" && mkdir -p "$WORK/data"
(cd "$WORK/data" && env \
  ADMIN_FORCE_MFA=false \
  LISTEN_SCHEME=http \
  LISTEN_ADDRESS=127.0.0.1 \
  LISTEN_PORT_HTTP="$PORT" \
  PUB_URL="localhost:$PORT" \
  RP_ID=localhost \
  RP_ORIGIN="http://localhost:$PORT" \
  HQL_NODE_ID=1 \
  SECRETS="$SECRETS" \
  ENC_KEYS="testkey1/$K1
$ENC_ACTIVE/$K2" \
  ENC_KEY_ACTIVE="$ENC_ACTIVE" \
  HQL_SECRET_RAFT="RaftSecret-1337-ABCD-xyz" \
  HQL_SECRET_API="ApiSecret-2468-WXYZ-abc" \
  BOOTSTRAP_ADMIN_EMAIL="init_admin@localhost" \
  BOOTSTRAP_ADMIN_PASSWORD_PLAIN="123SuperSafe" \
  BOOTSTRAP_DIR="$WORK/bootstrap" \
  RUST_LOG=info \
  "$BIN" serve > "$WORK/rauthy.log" 2>&1 & echo $! > "$WORK/rauthy.pid")

cleanup() { kill "$(cat "$WORK/rauthy.pid")" 2>/dev/null || true; }
trap cleanup EXIT

for _ in $(seq 1 80); do
  if curl -sf "http://127.0.0.1:$PORT/auth/v1/health" > /dev/null 2>&1; then
    break
  fi
  sleep 3
done
if ! curl -sf "http://127.0.0.1:$PORT/auth/v1/health" > /dev/null 2>&1; then
  echo "rauthy did not become healthy - see $WORK/rauthy.log" >&2
  exit 1
fi
echo "instance healthy on :$PORT - running the bin integration suite"

# --- run the suite -------------------------------------------------------------
(cd "$ROOT" && CARGO_TARGET_DIR="$TARGET" \
  cargo test -p rauthy --no-fail-fast "$@")
