# TLS 

If you do have TLS certificates from another source already, skip directly to [Config](#config).

## Generating Certificates

```admonish hint
We are using another project of mine called [Nioca](https://github.com/sebadob/nioca) for an easy creation of a
fully functioning and production ready private Root Certificate Authority (CA).
```

I suggest to use `docker` for this task. Otherwise, you can use the `nioca` binary directly on any linux machine.
If you want a permanent way of generating certificates for yourself, take a look at Rauthy's `justfile` and copy
and adjust the recipes `create-root-ca` and `create-end-entity-tls` to your liking.  
If you just want to get everything started quickly, follow these steps:

### Create an alias for the `docker` command
```
alias nioca='docker run --rm -it -v ./:/ca -u $(id -u ${USER}):$(id -g ${USER}) ghcr.io/sebadob/nioca'
```

To see the full feature set for more customization than mentioned below:
```
nioca x509 -h
```

### Generate full certificate chain

To make your browser happy, your need to have at least one `--alt-name-dns` for the URL of your application.
You can define as many of them as you like.

```
nioca x509 \
    --cn 'Rauthy Default' \
    --o 'Rauthy OIDC' \
    --alt-name-dns localhost \
    --alt-name-dns redhac.local \
    --usages-ext server-auth \
    --usages-ext client-auth \
    --stage full \
    --clean
```

You will be asked 6 times (yes, 6) for an at least 16 character password:
- The first 3 times, you need to provide the encryption password for your Root CA
- The last 3 times, you should provide a different password for your Intermediate CA

When everything was successful, you will have a new folder named `x509` with sub folders `root`, `intermediate`
and `end_entity` in your current one.

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

The [reference config](../config/config.html) contains a `TLS` section with all the values you can set.

For this example, we will be using the same certificates for both the internal cache mTLS connections and the
public facing HTTPS server.

### Cache

The cache layer (optionally) creates an mTLS connection and validates client certificates, if they are configured.  
To enable TLS at all, set

```
CACHE_TLS=true
```

By default, `redhac` expects certificates to be in `/app/tls/` with the common name / SNI `redhac.local`.  
The certificates need to be in the PEM format and you can provide different certificates for the server and client part,
if you like.

If this differs from your setup, you can set the following config variables:

```
# The path to the server TLS certificate PEM file (default: tls/redhac.local.cert.pem)
CACHE_TLS_SERVER_CERT=tls/cert-chain.pem
# The path to the server TLS key PEM file (default: tls/redhac.local.key.pem)
CACHE_TLS_SERVER_KEY=tls/key.pem
# If not empty, the PEM file from the specified location will be added as the CA certificate chain for validating
# the servers TLS certificate (default: tls/ca-chain.cert.pem)
CACHE_TLS_CA_SERVER=tls/ca-chain.pem

# The path to the client mTLS certificate PEM file (default: tls/redhac.local.cert.pem)
CACHE_TLS_CLIENT_CERT=tls/cert-chain.pem
# The path to the client mTLS key PEM file (default: tls/redhac.local.key.pem)
CACHE_TLS_CLIENT_KEY=tls/key.pem
# If not empty, the PEM file from the specified location will be added as the CA certificate chain for validating
# the clients mTLS certificate (default: tls/ca-chain.cert.pem)
CACHE_TLS_CA_CLIENT=tls/ca-chain.pem

# The domain / CN the client should validate the certificate against. This domain MUST be inside the
# 'X509v3 Subject Alternative Name' when you take a look at the servers certificate with the openssl tool.
# default: redhac.local
CACHE_TLS_CLIENT_VALIDATE_DOMAIN=redhac.local

# Can be used, if you need to overwrite the SNI when the client connects to the server, for instance if you are behind
# a loadbalancer which combines multiple certificates. (default: "")
#CACHE_TLS_SNI_OVERWRITE=
```

### Server 

The TLS configuration for the REST API is much simpler.  
By default, rauthy will expect a certificate and a key file in `/app/tls/tls.key` and `/app/tls/tls.crt`, which is the 
default naming for a Kubernetes TLS secret. The expected format is PEM, but you could provide the key in DER format too,
if you rename the file-ending to `*.der`.

You can change the default path for the files with the config variables `TLS_CERT` and `TLS_KEY`.

### Kubernetes

If you did not follow the above procedure to generate the CA and certificates, you may need to rename the files in the
following command, to create the Kubernetes secrets.

**Secrets - REST API**

```
kubectl -n rauthy create secret tls rauthy-tls --key="key.pem" --cert="cert-chain.pem"
``` 

**Secrets - `redhac` cache**

```
kubectl -n rauthy create secret tls redhac-tls-server --key="key.pem" --cert="cert-chain.pem" && \
kubectl -n rauthy create secret generic redhac-server-ca --from-file ca-chain.pem
``` 

```admonish notice
We create the `redhac-tls-server` here with the exact same values. If you really want to harden your setup in production,
you should provide a different set of certificates for the internal mTLS connection.
```

#### Config Adjustments - REST API

We need to configure the newly created Kubernetes secrets in the `std.yaml` from the
[Kubernetes](../getting_started/k8s.md#create-and-apply-the-stateful-set) setup.

1. In the `spec.template.spec.volumes` section, we need to mount the volumes from secrets:

**REST API**:
```yaml
- name: rauthy-tls
  secret:
    secretName: rauthy-tls
```

**`redhac` cache**:
```yaml
- name: redhac-tls-server
  secret:
    secretName: redhac-tls-server
- name: redhac-server-ca
  secret:
    secretName: redhac-server-ca
```

2. In the `spec.template.spec.containers.[rauthy].volumeMounts` section, add::

**REST API**:
```yaml
- mountPath: /app/tls/
  name: rauthy-tls
  readOnly: true
```

**`redhac` cache**:
```yaml
- mountPath: /app/tls/redhac/certs
  name: redhac-tls-server
  readOnly: true
- mountPath: /app/tls/redhac/ca
  name: redhac-server-ca
  readOnly: true
```

After having modified the config from above and the `sts.yaml` now, just apply both:

```
kubectl apply -f config.yaml
kubectl apply -f sts.yaml
```

The `rauthy` pods should restart now and TLS is configured.
