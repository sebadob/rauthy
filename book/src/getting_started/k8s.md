# Kubernetes

At the time of writing, there is no Helm Chart or Kustomize files available yet. The whole setup is
pretty simple on purpose though, so it should not be a big deal to get it running inside Kubernetes.

## Single Instance

Since Rauthy uses pretty aggressive caching for different reasons, you cannot just have a single
deployment and scale up the replicas without a proper HA setup. How to deploy a HA version is
described below.

The steps to deploy on Kubernetes are pretty simple.

1. Create namespace
2. Create and apply the config
3. Create and apply the stateful set

### Create Namespace

For the purpose of this documentation, we assume that Rauthy will be deployed in the `rauthy`
namespace.  
If this is not the case for you, change the following commands accordingly.

```
kubectl create ns rauthy
```

### Create and apply the config

This documentation will manage the Kubernetes files in a folder called `rauthy`.

```
mkdir rauthy && cd rauthy
```

Create the config file, paste the [Reference Config](../config/config.md) and adjust it to your
needs. We are putting the complete config in a K8s secret. Rauthy's config contains quite a few
different secret values and it's just a lot simpler to maintain everything in a single secret, than
splitting it into a `ConfigMap` and overwrite each secret manually.

```
apiVersion: v1
kind: Secret
metadata:
  name: rauthy-config
  namespace: rauthy
type: Opaque
stringData:
  config.toml: |
    PASTE CONFIG HERE - WATCH THE INDENTATION'
```

Open the config with your favorite editor and paste the [Reference Config](../config/config.md) in
place.  
Make sure to watch the indentation.

```admonish note
I recommend to just always set `cluster.node_id_from = "k8s"` when deploying a StatefulSet. This 
will parse the Raft NodeID automatically from the K8s Pod / Hostname and you don't have to worry 
about the `node_id`. For instance, a Pod named `rauthy-0` will be translated to `node_id = 1` 
automatically.
```

There are some values that you need to generate on your own. These are:

- `cluster.secret_raft` + `cluster.secret_api`
- `encryption.keys` + `encryption.key_active`

The secrets for the `cluster` can be just some long random alphanumeric values. They are used for
authentication for the Hiqlite Raft + API layer. The encryption keys must be generated. More
detailed explanation is in the [Encryption](../config/encryption.md) section. The tl;dr is:

```
echo "$(openssl rand -hex 4)/$(openssl rand -base64 32)"
```

Copy the output and add it to keys. The `key_active` will be the first part of the output until the
first `/`. For instance:

```toml
[encryption]
keys = ["XLCcaQ/f2xmq/nxVFgJN0CN311miyvVlBxXOQISyw1nPEPOqiI="]
key_active = "XLCcaQ"
```

You can generate safe values for both `secret_raft` and `secret_api` in many ways. You can just
provide a random alphanumeric value, which for instance:

```
cat /dev/urandom | tr -dc 'a-zA-Z0-9' | head -c48
```

or you can use the above `openssl` command again, even though Hiqlite does not need or utilize
base64:

```
openssl rand -base64 48
```

If you plan on using S3 for backups, paste the proper values into `cluster.s3_*` values.

```admonish note
It seems that in some environments, the above `openssl` command does not output proper values, which 
will make Rauthy panic on startup, when it checks the given values. If you run into that situation, 
you can generate them without `openssl` as well, with e.g:

<pre><code>
echo "$(cat /dev/urandom | tr -dc 'a-zA-Z0-9' | head -c 6)/$(cat /dev/urandom | head -c 32 | base64)"
</code></pre>
```

### Create and apply the stateful set

```
touch sts.yaml
```

Paste the following content into the `sts.yaml` file:

```yaml
apiVersion: v1
kind: Service
metadata:
  name: rauthy
  namespace: rauthy
spec:
  selector:
    app: rauthy
    ports:
      # chose whatever fits your needs here, you usually only need either http or https
      - name: http
        port: 8080
        targetPort: 8080
      - name: https
        port: 8443
        targetPort: 8443
---
# The headless service is used for the Raft Cluster setup, so Nodes 
# can connect to each other without any load balancer in between.
apiVersion: v1
kind: Service
metadata:
  name: rauthy-headless
  namespace: rauthy
spec:
  type: ClusterIP
  clusterIP: None
  # Make sure to only publish them on the headless service 
  # and NOT the one you are using via your reverse proxy!
  publishNotReadyAddresses: true
  sessionAffinity: None
  selector:
    app: rauthy
  ports:
    - name: hiqlite-raft
      protocol: TCP
      port: 8100
      targetPort: 8100
    - name: hiqlite-api
      protocol: TCP
      port: 8200
      targetPort: 8200
---
# The PDB is only necessary for a HA deployment. You can take it out for a single instance.
apiVersion: policy/v1
kind: PodDisruptionBudget
metadata:
  name: rauthy
  namespace: rauthy
spec:
  maxUnavailable: 1
  selector:
    matchLabels:
      app: rauthy
---
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: rauthy
  namespace: rauthy
  labels:
    app: rauthy
spec:
  serviceName: rauthy-headless
  # If you start a fresh cluster without a bootstrapped Admin password, it is
  # highly suggested to start a single replica for the first setup + login.
  # It will work with 3 replicas directly, but if you are not quick enough and
  # your logs buffer size is small, you might miss the auto-generated password
  # in Pod `rauthy-0` because of many logs.
  replicas: 1
  selector:
    matchLabels:
      app: rauthy
  template:
    metadata:
      labels:
        app: rauthy
    spec:
      affinity:
        podAntiAffinity:
          requiredDuringSchedulingIgnoredDuringExecution:
            - labelSelector:
                matchExpressions:
                  - key: app
                    operator: In
                    values:
                      - rauthy
              topologyKey: "kubernetes.io/hostname"
      securityContext:
        fsGroup: 10001
      containers:
        - name: rauthy
          image: ghcr.io/sebadob/rauthy:0.34.0
          securityContext:
            capabilities:
              drop:
                - ALL
            # User ID 10001 is actually built into the container 
            # at the creation for better security
            runAsUser: 10001
            runAsGroup: 10001
            runAsNonRoot: true
            allowPrivilegeEscalation: false
          ports:
            # Hiqlite internal ports
            - containerPort: 8100
            - containerPort: 8200
            # You may need to adjust this, if you decide to start 
            # in https only mode or use another port
            - containerPort: 8080
            - containerPort: 8443
          volumeMounts:
            - name: rauthy-data
              mountPath: /app/data
              readOnly: false
            - name: rauthy-config
              mountPath: /app/config.toml
              readOnly: true
              subPath: config.toml
          readinessProbe:
            httpGet:
              scheme: HTTP
              # Hiqlite API port
              port: 8200
              path: /ready
            initialDelaySeconds: 5
            # Do NOT increase this period, because otherwise K8s may not catch
            # a shutting down pod fast enough and may keep routing requests to
            # it while is will be unable to handle them properly because of
            # the shutdown.
            periodSeconds: 3
            # We may get a single failure during leader switches
            failureThreshold: 2
          livenessProbe:
            httpGet:
              scheme: HTTP
              # Rauthy API port
              port: 8080
              path: /auth/v1/health
            initialDelaySeconds: 60
            periodSeconds: 30
            # We may get a single failure during leader switches
            failureThreshold: 2
          resources:
            requests:
              # Tune the memory requests value carefully. Make sure, that the
              # pods request at least:
              # `ARGON2_M_COST` / 1024 * `MAX_HASH_THREADS` Mi + idle memory
              # The actual usage also heavily depends on the Memory Allocator
              # tuning. You can find more information in the Tuning section
              # in this book.
              #
              # A HA instance with Hiqlite enabled and without additional memory
              # tuning will usually settle ~100mb idle memory being used.
              # If you use an external Postgres, idle memory can go as low as
              # ~30mb.
              memory: 64Mi
              # The CPU needs to be adjusted during runtime. This heavily
              # depends on your use case.
              cpu: 100m
            limits:
            # Be careful with the memory limit. You must make sure, that the
            # (very costly) password hashing has enough memory available. If not,
            # the application will crash. You do not really need a memory limit,
            # since Rust is not a garbage collected language. Better take a close
            # look at what the container actually needs during
            # prime time and set the requested resources above properly.
            #memory:
            # A CPU limit may make sense in case of DDoS attacks or something
            # like this, if you do not have external rate limiting or other
            # mechanisms. Otherwise, `MAX_HASH_THREADS` is the main mechanism 
            # to limit resources.
            #cpu: 1000m
      volumes:
        - name: rauthy-config
          secret:
            secretName: rauthy-config
  volumeClaimTemplates:
    - metadata:
        name: rauthy-data
      spec:
        accessModes:
          - "ReadWriteOnce"
        resources:
          requests:
            storage: 128Mi
        #storageClassName: provideIfNeeded
```

### Ingress

This example assumes, that the deployment will run behind a Kubernetes ingress resource of your
choice.

It uses [Traefik](https://doc.traefik.io/traefik/) with the `IngressRoute` CRD.  
Nevertheless, the ingress is really simple, and it should be very easy to adopt anything else.

Create the `ingress.yaml`

```
touch ingress.yaml
```

Paste the following content into the `ingress.yaml` file and adjust to your needs

```yaml
apiVersion: traefik.containo.us/v1alpha1
kind: IngressRoute
metadata:
  name: rauthy-https
  namespace: rauthy
spec:
  entryPoints:
    - websecure
  routes:
    - match: Host(`auth.example.com`)
      kind: Rule
      services:
        - name: rauthy
          port: 8080
```

### Deploy

We are now ready to deploy:

```
kubectl apply -f .
```

And then to observe the deployment:

```
kubectl -n rauthy get pod -w
```

You can now proceed with the [First Start](first_start.md) steps.

## Production Setup

### Config

Going to production does not need too many additional steps.

#### TLS Certificates

The thing you need will be valid TLS certificates, of course. To get these, there are a lot of
existing mechanisms. If you use an internal Certificate Authority (CA), you do have you own tools to
work with this anyway. If, however, you want to use something
like [Let's Encrypt](https://letsencrypt.org/de/), I suggest to use
the [cert-manager](https://cert-manager.io/), which is easy and straight forward to use.

An example, how to add a certificate for the Traefik IngressRoute from above:

```yaml
apiVersion: traefik.containo.us/v1alpha1
kind: IngressRoute
metadata:
  name: rauthy-https
  namespace: rauthy
spec:
  entryPoints:
    - websecure
  tls:
    # Paste the name of the TLS secret here
    secretName: secret-name-of-your-tls-certificate
  routes:
    - match: Host(`auth.example.com`)
      kind: Rule
      services:
        - name: rauthy
          port: 8080
```

You may want to add an HTTPS redirect as well:

```yaml
apiVersion: traefik.containo.us/v1alpha1
kind: Middleware
metadata:
  name: https-only
  namespace: rauthy
spec:
  redirectScheme:
    scheme: https
    permanent: true
---
apiVersion: traefik.containo.us/v1alpha1
kind: IngressRoute
metadata:
  name: rauthy-http
  namespace: rauthy
spec:
  entryPoints:
    - web
  routes:
    - match: Host(`auth.example.com`)
      kind: Rule
      middlewares:
        - name: https-only
      services:
        - name: rauthy
          port: 8080
```

#### Hiqlite Internal TLS

You can of course also provide TLS certificates for the Hiqlite internal communication. Two
independent networks are created: one for the Raft-Internal network traffic like heartbeats and data
replication, and a second one for the "external" Hiqlite API. This is used by other Hiqlite cluster
members for management purposes and to execute things like consistent queries on the leader node.

You can provide TLS certificates for both of them independently via the following config variables:

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
tls_raft_danger_tls_no_verify = true

# overwritten by: HQL_TLS_API_KEY
tls_api_key = "tls/tls.key"
# overwritten by: HQL_TLS_RAFT_KEY
tls_api_cert = "tls/tls.crt"
tls_api_danger_tls_no_verify = true
```

#### Additional steps

There are a few more things to do when going into production, but these are the same for Kubernetes
and Docker and will be explained in later chapters.

You can now proceed with the [First Start](first_start.md) steps.
