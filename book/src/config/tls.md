# TLS

If you do have TLS certificates from another source already, skip directly to [Config](#config).

## Generating Certificates

You can either generate TLS certificates yourself and manage your own CA, or (for testing purposes)
let Rauthy generate self-signed certificates. Rauthys self-signed ones should only be used for
testing, but they are generated somewhat securely. Rauthy will create it's own CA and save it do the
database, with the private key being encrypted with the configured `encryption.key_active`. The CA
will be valid for 10 years and re-used for signing the server certs in all instances, even you do an
HA deployment.

```admonish caution
Some browsers like Firefox do not allow the registration of Passkeys when using self-signed TLS 
certificates. To be able to do this during testing, you would need to add the generated CA 
certificate to your trust store.

In such a case, you will probably see an error like `Invalid Key` during registration. This happens 
in insecure contexts.
```

### Automatic self-signed certificates

#### Rauthy Server + API

Generating self-signed TLS certificates for the Rauthy HTTPS server is pretty straight forward. Keep
in mind that these will only be generated for the HTTPS server and NOT for Hiqlite internal cluster
traffic, if you have an HA deployment. The cluster-internal auto-tls has a separate config variable.

To generate TLS certificates on startup, you only need to set `tls.generate_self_signed = true`:

```toml
[tls]
# If set to `true`, Rauthy will generate self-signed TLS certs and copy
# them into `tls/self_signed_cert.pem` and `tls/self_signed_key.pem`.
# It will also IGNORE any `cert_path` / `key_path`.
#
# CAUTION: If set to `true`, it will delete existing files:
# - `tls/self_signed_cert.pem`
# - `tls/self_signed_key.pem`
#
# This should only be used for testing and never in production!
#
# default: false
# overwritten by: TLS_GENERATE_SELF_SIGNED
generate_self_signed = true
```

If you only care about testing certificates for the HTTPS server, you don't need to configure
anything else at this point.

#### Hiqlite - Raft Cluster Traffic

By default, Raft-internal traffic is NOT encrypted. The reasons is that you may not even want this
in situations like when you e.g. have a separate physical network for nodes, or you already have a
service mesh with mTLS in place. However, if you want to follow a zero-trust philosophy, the best
way to do this is by letting Hiqlite handle everything for you automatically:

```toml
[cluster]
# The `tls_auto_certificates` will generate self-signed TLS
# certificates for internal Raft and API traffic. Clients will
# simply not validate the certificates for ease of use because
# they don't have to. They do a 3-way handshake anyway, which
# validates both client and server without the secret ever being
# sent over the network.
#
# If you specify specific certificates with either `tls_raft_*` or
# `tls_api_*`, they will be used instead.
#
# default: false
# overwritten by: HQL_TLS_AUTO_CERTS
tls_auto_certificates = true
```

### Manually managed certificates

```admonish note
We are using another project of mine called [Nioca](https://github.com/sebadob/nioca) for an easy 
creation of a fully functioning and production ready private Root Certificate Authority (CA).
```

I suggest to use `docker` for this task. Otherwise, you can use the `nioca` binary directly on any
linux machine. If you want a permanent way of generating certificates for yourself, take a look at
Rauthys `justfile` and copy and adjust the recipes `create-root-ca` and `create-end-entity-tls` to
your liking.

If you just want to get everything started quickly, follow these steps:

#### Create an alias for the `docker` command

```
alias nioca='docker run --rm -it -v ./:/ca -u $(id -u ${USER}):$(id -g ${USER}) ghcr.io/sebadob/nioca'
```

To see the full feature set for more customization than mentioned below:

```
nioca x509 -h
```

#### Generate full certificate chain

To make your browser happy, your need to have at least one `--alt-name-dns` for the URL of your
application.
You can define as many of them as you like.

```
nioca x509 \
    --cn 'Rauthy Default' \
    --o 'Rauthy OIDC' \
    --alt-name-dns localhost \
    --alt-name-dns rauthy.rauthy \
    --alt-name-dns rauthy.rauthy.svc.cluster.local \
    --usages-ext server-auth \
    --usages-ext client-auth \
    --stage full \
    --clean
```

You will be asked 6 times (yes, 6) for an at least 16 character password:

- The first 3 times, you need to provide the encryption password for your Root CA
- The last 3 times, you should provide a different password for your Intermediate CA

When everything was successful, you will have a new folder named `x509` with sub folders `root`,
`intermediate` and `end_entity` in your current one.

From these, you will need the following files:

```
cp x509/intermediate/ca-chain.pem . && \
cp x509/end_entity/$(cat x509/end_entity/serial)/cert-chain.pem . && \
cp x509/end_entity/$(cat x509/end_entity/serial)/key.pem .
```

- You should have 3 files in `ls -l`:

```
ca-chain.pem
cert-chain.pem
key.pem
```

## Config

The [reference config](../config/config.html) contains a `TLS` section with all the values you can
set.

For this example, we will be using the same certificates for both the internal cache mTLS
connections and the public-facing HTTPS server.

### Hiqlite

Hiqlite can run the whole database layer, and it will always take care of caching. It can be
configured to use TLS internally, if you provide certificates. Simply provide the following values
from the `TLS` section in the reference config:

```toml
[cluster]
# If given, these keys / certificates will be used to establish
# TLS connections between nodes.
#
# values are optional, overwritten by: HQL_TLS_{RAFT|API}_{KEY|CERT}
# overwritten by: HQL_TLS_RAFT_KEY
tls_raft_key = "tls/tls.key"
# overwritten by: HQL_TLS_RAFT_CERT
tls_raft_cert = "tls/tls.crt"
#tls_raft_danger_tls_no_verify = true

# overwritten by: HQL_TLS_API_KEY
tls_api_key = "tls/tls.key"
# overwritten by: HQL_TLS_RAFT_KEY
tls_api_cert = "tls/tls.crt"
#tls_api_danger_tls_no_verify = true
```

There is no problem using the same certificates for both networks, but you can optionally even
separate them if you need to. You could even re-use the Server TLS, if your DNS setup allows for
this.

```admonish note
At the time of writing, it does not accept a custom Root CA yet. In this case you have to set the 
`*_danger_tls_no_verify` to `true`
```

### Rauthy Server / API

By default, rauthy will expect a certificate and a key file in `/app/tls/tls.key` and
`/app/tls/tls.crt`, which is the default naming for a Kubernetes TLS secret. The expected format is
PEM, but you could provide the key in DER format too, if you rename the file-ending to `*.der`.

```toml
[tls]
## UI + API TLS

# Overwrite the path to the TLS certificate file in PEM
# format for rauthy
#
#default: 'tls/tls.crt'
# overwritten by: TLS_CERT
cert_path = 'tls/cert-chain.pem'

# Overwrite the path to the TLS private key file in PEM
# format for rauthy. If the path / filename ends with '.der',
# rauthy will parse it as DER, otherwise as PEM.
#
# default: 'tls/tls.key'
# overwritten by: TLS_KEY
key_path = 'tls/key.pem'
```

### Kubernetes

If you did not follow the above procedure to generate the CA and certificates, you may need to
rename the files in the following command, to create the Kubernetes secrets.

**Secrets - Rauthy Server / API**

```
kubectl -n rauthy create secret tls rauthy-tls --key="key.pem" --cert="cert-chain.pem"
``` 

**Secrets - `hiqlite` cache**

```
kubectl -n rauthy create secret tls hiqlite-tls --key="key.pem" --cert="cert-chain.pem"
``` 

```admonish notice
We create the `hiqlite-tls` here with the exact same values. You could of course either use 
different certificates, or not create a separate secret at all and just re-use the Rauthy TLS 
certificates, if you DNS setup allows for proper validation in this case.
```

#### Config Adjustments - REST API

We need to configure the newly created Kubernetes secrets in the `sts.yaml` from the
[Kubernetes](../getting_started/k8s.md#create-and-apply-the-stateful-set) setup.

1. In the `spec.template.spec.volumes` section, we need to mount the volumes from secrets:

**REST API**:

```yaml
- name: rauthy-tls
  secret:
    secretName: rauthy-tls
```

**`hiqlite` cache**:

```yaml
- name: hiqlite-tls
  secret:
    secretName: hiqlite-tls
```

2. In the `spec.template.spec.containers.[rauthy].volumeMounts` section, add::

**REST API**:

```yaml
- mountPath: /app/tls/
  name: rauthy-tls
  readOnly: true
```

**`hiqlite` cache**:

```yaml
- mountPath: /app/tls/hiqlite/
  name: hiqlite-tls
  readOnly: true
```

After having modified the config from above and the `sts.yaml` now, just apply both:

```
kubectl apply -f config.yaml
kubectl apply -f sts.yaml
```

The `rauthy` pods should restart now and TLS is configured.
