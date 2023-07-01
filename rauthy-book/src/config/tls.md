# TLS 

If you do have TLS certificates from another source already, skip directly to [Config](#config).

## Generate Certificates

```admonish hint
The tools provided in the rauthy repository are very basic and have a terrible user experience.  
They should only be used, if you do not have an already existing TLS setup or workflow.  
A project specifically tailored to TLS CA and certificates is in the making.
```

As mentioned, the tools are very basic. If you for instance type in a bad password during CA / intermediate generation,
they will just throw an error, and you need to clean up again. They should only get you started and be used for testing.  
There are a lot of good tools out there which can get you started with TLS and there is no real benefit in creating 
just another one that does the same stuff.

The scripts can be found here (TODO add link to tools).  
You need to have `openssl` and a BASH shell available on your system. They have not been tested with Windows.

The cache layer does validate the CA for mTLS connections, which is why you can generate a full set of certificates.

**1. Certificate Authority (CA)**  
- Execute
```
./build_ca.sh
```
and enter an at least 4 character password for the private key file for the CA 3 times.

**2. Intermediate CA**
- Execute
```
./build_intermediate.sh
```
  and enter an at least 4 character password for the private key file for the CA 3 times.
- The 4. password needs to be the one from the CA private key file from step 1.
- Type `y` 2 times to verify the signing of the new certificate
- On success, you will see `intermediate/certs/intermediate.cert.pem: OK` as the last line

**3. End Entity Certificates**  
These are the certificates used by the cache, rauthy itself, or any other server / client.
- The end entity script needs the common name for the certificate as the 1. option when you execute it:
```
./build_end_entity.sh redhac.local
```
- Enter the password for the Intermediate CA private key from step 2 and verify the signing.  
- On success, you will see `intermediate/certs/redhac.local.cert.pem: OK` as the last line 
- Generate another set of certificates for rauthy itself and add your domain name as argument
```
./build_end_entity.sh auth.example.com
```
- You should have 5 files in the `../` folder:
```
auth.example.com.cert.pem
auth.example.com.key.pem
ca-chain.pem
redhac.local.cert.pem
redhac.local.key.pem
```

```admonish info
This is not a tutorial about TLS certificates.  
As mentioned above already, another dedicated TLS project is in the making.
```

## Config

The [reference config](../config/config.html) contains a `TLS` section with all the values you can set.

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
CACHE_TLS_SERVER_CERT=tls/redhac.local.cert.pem
# The path to the server TLS key PEM file (default: tls/redhac.local.key.pem)
CACHE_TLS_SERVER_KEY=tls/redhac.local.key.pem
# If not empty, the PEM file from the specified location will be added as the CA certificate chain for validating
# the servers TLS certificate (default: tls/ca-chain.cert.pem)
CACHE_TLS_CA_SERVER=tls/ca-chain.cert.pem

# The path to the client mTLS certificate PEM file (default: tls/redhac.local.cert.pem)
CACHE_TLS_CLIENT_CERT=tls/redhac.local.cert.pem
# The path to the client mTLS key PEM file (default: tls/redhac.local.key.pem)
CACHE_TLS_CLIENT_KEY=tls/redhac.local.key.pem
# If not empty, the PEM file from the specified location will be added as the CA certificate chain for validating
# the clients mTLS certificate (default: tls/ca-chain.cert.pem)
CACHE_TLS_CA_CLIENT=tls/ca-chain.cert.pem

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
kubectl -n rauthy create secret tls rauthy-tls --key="../auth.example.com.key.pem" --cert="../auth.example.com.cert.pem" && \
``` 

**Secrets - `redhac` cache**

```
kubectl -n rauthy create secret tls redhac-tls-server --key="../redhac.local.key.pem" --cert="../redhac.local.cert.pem" && \
kubectl -n rauthy create secret generic redhac-server-ca --from-file ../ca-chain.pem
``` 

#### Config Adjustements - REST API

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
