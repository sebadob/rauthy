# Rauthy + Vault config

Support for loading the config from a Vault source was added in v0.31.0. To enable it, 
set the environment variable `USE_VAULT_CONFIG=true` and include a [vault.toml](../../vault.toml) file, 
that specifies the Vault connection details and how configuration values can be overridden using environment variables.
If you are testing with an insecure connection (e.g., http://), also set `DANGER_VAULT_INSECURE=true` — use this only for development/testing, never in production.

## Load config from Vault

1) Put your config into Vault (KV)
   Store the Rauthy config values under a KV path (most examples assume KV v2).
   - Mount: secret
   - Path: rauthy
   - Version: 2

   So the effective secret would be at something like secret/data/rauthy (KV v2 layout).


2) Provide Vault connection info to Rauthy via environment variables
   When Vault token is available, Rauthy loads its config from Vault.

   Set:

   ```toml
   VAULT_ADDR="http://127.0.0.1:8201"
   VAULT_TOKEN="hvs...."
   VAULT_MOUNT="secret"
   VAULT_PATH="rauthy"
   VAULT_VERSION="2"
   ```

3) Start Rauthy normally
Run Rauthy as usual; it will decide whether to load config locally vs remotely based on the Vault env setup (e.g., if `USE_VAULT_CONFIG=true` is set, it uses the Vault path).

## Example ENV vars for Docker

```
-e VAULT_ADDR=https://myvault.example.com:8200 \
-e VAULT_TOKEN=hvs.... \
-e VAULT_MOUNT=secret \
-e VAULT_PATH=rauthy_config \
-e VAULT_CONFIG_KEY=config.toml \
-e VAULT_KV_VERSION=2 \
```

## Local Vault Example using Docker

Warning: These examples are dangerous as written! They include secrets (e.g., in `vault kv put ...`) only for automated demonstration purposes, and it would never be included in such a file for production.

Example docker compose file using a predefined root token:

IMPORTANT: make sure to run `export DOCKER_MACHINE_IP=<YOUR IP>` first, if using this example or edit the compose file, else you will see something like this error in the logs:
`http://:8202/v1/auth/token/lookup-self": dial tcp :8202: connect: connection refused`

```yaml
# Example using a manually defined  root token
# export DOCKER_MACHINE_IP=$(docker-machine ip)
# or fixed ip:
# export DOCKER_MACHINE_IP=10.0.0.100
# docker compose up

version: "3"

networks:
  rauthy-test:
    driver: bridge

services:
  dev-vault-svc:
    image: hashicorp/vault:1.20.0
    container_name: dev-vault
    cap_add:
      - IPC_LOCK  # Lock memory to prevent sensitive data from swapping to disk
    environment:
      - VAULT_DEV_ROOT_TOKEN_ID=unsafe
      - VAULT_DEV_LISTEN_ADDRESS=0.0.0.0:8200
      - VAULT_ADDR=http://${DOCKER_MACHINE_IP}  # Set the Vault address
    ports:
      - "8202:8200"  # Expose Vault port 8200 on host port 8202
    command: server -dev  # Start Vault in development mode
    healthcheck:
      test: ["CMD", "vault", "status"]
      interval: 30s
      timeout: 10s
      retries: 5

  vault-setup-svc:
    image: hashicorp/vault:1.20.0
    environment:
      - VAULT_ADDR=http://${DOCKER_MACHINE_IP}:8202
    depends_on:
      - dev-vault-svc
    restart: "no"
    entrypoint: [ "sh", "-c", "sleep 5 && vault login unsafe && vault kv put /secret/rauthy_config config.toml='[auth_headers]\nenable = true\n[cluster]\nnode_id = 1\nnodes = [\"1 localhost:8100 localhost:8200\"]\n# 123SuperMegaSafe\npassword_dashboard = \"JGFyZ29uMmlkJHY9MTkkbT0zMix0PTIscD0xJE9FbFZURnAwU0V0bFJ6ZFBlSEZDT0EkTklCN0txTy8vanB4WFE5bUdCaVM2SlhraEpwaWVYOFRUNW5qdG9wcXkzQQ==\"\nsecret_raft = \"SuperSecureSecret1337\"\nsecret_api = \"SuperSecureSecret1337\"\n[database]\npg_host = \"localhost\"\npg_user = \"rauthy\"\npg_password = \"123SuperSafe\"\n[dynamic_clients]\nenable = true\n[email]\nrauthy_admin_email = \"admin@localhost\"\n[encryption]\nkeys = [\"bVCyTsGaggVy5yqQ/UzluN29DZW41M3hTSkx6Y3NtZmRuQkR2TnJxUTYzcjQ=\"]\nkey_active = \"bVCyTsGaggVy5yqQ\"\n[ephemeral_clients]\nenable = true\nenable_web_id = true\nenable_solid_aud = true\nallowed_flows = [\"authorization_code\", \"refresh_token\"]\n[http_client]\ndanger_unencrypted = true\ndanger_insecure = true\n[mfa]\nadmin_force_mfa = false\n[pow]\ndifficulty = 10\n[server]\nport_http = 8080\nport_https = 8443\nscheme = \"https\"\npub_url = \"localhost:8443\"\nhttp_workers = 1\nmetrics_enable = false\nswagger_ui_enable = true\n[tls]\ncert_path = \"tls/cert-chain.pem\"\nkey_path = \"tls/key.pem\"\n[user_registration]\nenable = true\n[webauthn]\nrp_id = \"localhost\"\nrp_origin = \"http://localhost:8080\"'"]
    healthcheck:
        test: ["CMD", "vault", "kv", "get", "rauthy_config/config.toml"]
        interval: 30s
        timeout: 10s
        retries: 5

  mailcrab-svc:
    image: marlonb/mailcrab:latest
    ports:
      - "1025:1025"
      - "1080:1080"
    networks:
      - rauthy-test

  rauthy-svc:
    container_name: rauthy-test
    image: ghcr.io/sebadob/rauthy:0.36.1
    environment:
      - DANGER_VAULT_INSECURE=true
      - PUB_URL=${DOCKER_MACHINE_IP}:8443
      #- TLS_CERT=tls/cert-chain.pem
      #- TLS_KEY=tls/key.pem
      - USE_VAULT_CONFIG=true
      - VAULT_ADDR=http://${DOCKER_MACHINE_IP}:8202
      - VAULT_TOKEN=unsafe
      - VAULT_MOUNT=secret
      - VAULT_PATH=rauthy_config
      - VAULT_CONFIG_KEY=config.toml
      - VAULT_KV_VERSION=2
      - SMTP_URL=${DOCKER_MACHINE_IP}
      - SMTP_PORT=1025
      - SMTP_DANGER_INSECURE=true
    ports:
      - "8443:8443"
    depends_on:
      mailcrab-svc:
        condition: service_started
      dev-vault-svc:
        condition: service_started
      vault-setup-svc:
        condition: service_completed_successfully
    networks:
      - rauthy-test
```

Example docker compose file for one time use token:

```yaml
# Example for a one-time-use token using a shared .env file to transfer the token from vault-setup to rauthy
# first create the .env file in the current dir where docker compose up is run from, next to the docker-copose file and set HOSTIP env var
# touch .env
# export DOCKER_MACHINE_IP=$(docker-machine ip)
# or fixed ip:
# export DOCKER_MACHINE_IP=10.0.0.100
# docker compose up
# now a restart of the rauthy-test container is expected to fail, as the one-time-use token is now expired/invalid

version: "3"

networks:
  rauthy-test:
    driver: bridge

services:
  dev-vault-svc:
    image: hashicorp/vault:1.20.0
    container_name: dev-vault
    cap_add:
      - IPC_LOCK  # Lock memory to prevent sensitive data from swapping to disk
    environment:
      - VAULT_DEV_ROOT_TOKEN_ID=unsafe
      - VAULT_DEV_LISTEN_ADDRESS=0.0.0.0:8200
      - VAULT_ADDR=http://${DOCKER_MACHINE_IP}  # Set the Vault address
    ports:
      - "8202:8200"  # Expose Vault port 8200 on host port 8202
    command: server -dev  # Start Vault in development mode
    healthcheck:
      test: ["CMD", "vault", "status"]
      interval: 30s
      timeout: 10s
      retries: 5

  vault-setup-svc:
    image: hashicorp/vault:1.20.0
    environment:
      - VAULT_ADDR=http://${DOCKER_MACHINE_IP}:8202
    depends_on:
      - dev-vault-svc
    restart: "no"
    volumes:
      - type: bind
        source: ./.env
        target: /vault/tokens/.env
    entrypoint: [ "sh", "-c", "sleep 5 && vault login unsafe && vault kv put /secret/rauthy_config config.toml='[auth_headers]\nenable = true\n[cluster]\nnode_id = 1\nnodes = [\"1 localhost:8100 localhost:8200\"]\n# 123SuperMegaSafe\npassword_dashboard = \"JGFyZ29uMmlkJHY9MTkkbT0zMix0PTIscD0xJE9FbFZURnAwU0V0bFJ6ZFBlSEZDT0EkTklCN0txTy8vanB4WFE5bUdCaVM2SlhraEpwaWVYOFRUNW5qdG9wcXkzQQ==\"\nsecret_raft = \"SuperSecureSecret1337\"\nsecret_api = \"SuperSecureSecret1337\"\n[database]\npg_host = \"localhost\"\npg_user = \"rauthy\"\npg_password = \"123SuperSafe\"\n[dynamic_clients]\nenable = true\n[email]\nrauthy_admin_email = \"admin@localhost\"\n[encryption]\nkeys = [\"bVCyTsGaggVy5yqQ/UzluN29DZW41M3hTSkx6Y3NtZmRuQkR2TnJxUTYzcjQ=\"]\nkey_active = \"bVCyTsGaggVy5yqQ\"\n[ephemeral_clients]\nenable = true\nenable_web_id = true\nenable_solid_aud = true\nallowed_flows = [\"authorization_code\", \"refresh_token\"]\n[http_client]\ndanger_unencrypted = true\ndanger_insecure = true\n[mfa]\nadmin_force_mfa = false\n[pow]\ndifficulty = 10\n[server]\nport_http = 8080\nport_https = 8443\nscheme = \"https\"\npub_url = \"localhost:8443\"\nhttp_workers = 1\nmetrics_enable = false\nswagger_ui_enable = true\n[tls]\ncert_path = \"tls/cert-chain.pem\"\nkey_path = \"tls/key.pem\"\n[user_registration]\nenable = true\n[webauthn]\nrp_id = \"localhost\"\nrp_origin = \"http://localhost:8080\"' && echo 'path \"secret/data/rauthy_config\" {capabilities = [\"read\"]}' | vault policy write rauthy_config_acl_policy -  && export ONETIME_TOKEN=$$(vault token create -policy=rauthy_config_acl_policy -use-limit=1 -orphan=true -renewable=false -ttl=10m | grep -m 1 token | awk '{print $$2}') && echo \"VAULT_TOKEN=$$ONETIME_TOKEN\" > /vault/tokens/.env && cat /vault/tokens/.env"]
    healthcheck:
        test: ["CMD", "vault", "kv", "get", "rauthy_config/config.toml"]
        interval: 30s
        timeout: 10s
        retries: 5

  mailcrab-svc:
    image: marlonb/mailcrab:latest
    ports:
      - "1025:1025"
      - "1080:1080"
    networks:
      - rauthy-test

  rauthy-svc:
    container_name: rauthy-test
    image: ghcr.io/sebadob/rauthy:0.36.1
    environment:
      - DANGER_VAULT_INSECURE=true
      - PUB_URL=${DOCKER_MACHINE_IP}:8443
      #- TLS_CERT=tls/cert-chain.pem
      #- TLS_KEY=tls/key.pem
      - USE_VAULT_CONFIG=true
      - VAULT_ADDR=http://${DOCKER_MACHINE_IP}:8202
      #- VAULT_TOKEN=unsafe
      - VAULT_MOUNT=secret
      - VAULT_PATH=rauthy_config
      - VAULT_CONFIG_KEY=config.toml
      - VAULT_KV_VERSION=2
      - SMTP_URL=${DOCKER_MACHINE_IP}
      - SMTP_PORT=1025
      - SMTP_DANGER_INSECURE=true
    ports:
      - "8443:8443"
    volumes:
      - type: bind
        source: ./.env
        target: /app/.env
    depends_on:
      mailcrab-svc:
        condition: service_started
      dev-vault-svc:
        condition: service_started
      vault-setup-svc:
        condition: service_completed_successfully
    networks:
      - rauthy-test
```
