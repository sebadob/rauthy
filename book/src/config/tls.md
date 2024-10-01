# TLS

If you do have TLS certificates from another source already, skip directly to [Config](#config).

## Generating Certificates

```admonish hint
We are using another project of mine called [Nioca](https://github.com/sebadob/nioca) for an easy creation of a
fully functioning and production ready private Root Certificate Authority (CA).
```

I suggest to use `docker` for this task. Otherwise, you can use the `nioca` binary directly on any linux machine.
If you want a permanent way of generating certificates for yourself, take a look at Rauthys `justfile` and copy
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

The cache layer (optionally) uses TLS, if you provide certificates. Simply provide the following values from the `TLS`
section in the reference config:

```
# If given, these keys / certificates will be used to establish
# TLS connections between nodes.
HQL_TLS_RAFT_KEY=tls/hiqlite/tls.key
HQL_TLS_RAFT_CERT=tls/hiqlite/tls.crt
#HQL_TLS_RAFT_DANGER_TLS_NO_VERIFY=false

HQL_TLS_API_KEY=tls/hiqlite/tls.key
HQL_TLS_API_CERT=tls/hiqlite/tls.crt
#HQL_TLS_API_DANGER_TLS_NO_VERIFY=false
```

There is no problem using the same certificates for both networks, but you can optionally even separate them if you need
to. You could even re-use the Server TLS, if your DNS setup allows for this.

```admonish note
At the time of writing, it does not accept a custom Root CA yet. In this case you have to set the 
`*_DANGER_TLS_NO_VERIFY` to `true`
```

### Rauthy Server / API

By default, rauthy will expect a certificate and a key file in `/app/tls/tls.key` and `/app/tls/tls.crt`, which is the
default naming for a Kubernetes TLS secret. The expected format is PEM, but you could provide the key in DER format too,
if you rename the file-ending to `*.der`.

You can change the default path for the files with the config variables `TLS_CERT` and `TLS_KEY`.

### Kubernetes

If you did not follow the above procedure to generate the CA and certificates, you may need to rename the files in the
following command, to create the Kubernetes secrets.

**Secrets - Rauthy Server / API**

```
kubectl -n rauthy create secret tls rauthy-tls --key="key.pem" --cert="cert-chain.pem"
``` 

**Secrets - `hiqlite` cache**

```
kubectl -n rauthy create secret tls hiqlite-tls --key="key.pem" --cert="cert-chain.pem"
``` 

```admonish notice
We create the `hiqlite-tls` here with the exact same values. You could of course either use different certificates, or
not create a separate secret at all and just re-use the Rauthy TLS certificates, if you DNS setup allows for proper 
validation in this case.
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
