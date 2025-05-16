# Kubernetes

At the time of writing, there is no Helm Chart or Kustomize files available yet. The whole setup is pretty simple
on purpose though, so it should not be a big deal to get it running inside Kubernetes.

## Single Instance

Since rauthy uses pretty aggressive caching for different reasons, you cannot just have a single deployment and
scale up the replicas without enabling `HA_MODE`. How to deploy a HA version is described below.

The steps to deploy on Kubernetes are pretty simple.

- Create namespace
- Create and apply the config
- Create and apply secrets
- Create and apply the stateful set

### Create Namespace

For the purpose of this documentation, we assume that rauthy will be deployed in the `rauthy` namespace.  
If this is not the case for you, change the following commands accordingly.

```
kubectl create ns rauthy
```

### Create and apply the config

This documentation will manage the Kubernetes files in a folder called `rauthy`.

```
mkdir rauthy && cd rauthy
```

Create the config file, paste the [reference config](../config/config.html) and adjust it to your needs.  
There is no "nice 1-liner" available yet.

```
echo 'apiVersion: v1
kind: ConfigMap
metadata:
  name: rauthy-config
  namespace: rauthy
data:
  rauthy.cfg: |+
    PASTE CONFIG HERE - WATCH THE INDENTATION' > config.yaml
```

Open the config with your favorite editor and paste the [reference config](../config/config.html) in place.  
Make sure to watch the indentation.

```admonish caution
Do not include sensitive information like for instance the ENC_KEYS inside the normal Config.
Use the secrets from the next step for this.  
```

```admonish note
I recommend to just always set `HQL_NODE_ID_FROM=k8s` when deploying a StatefulSet. This will parse the Raft NodeID 
automatically from the K8s Pod / Hostname and you don't have to worry about the `HQL_NODE_ID`. For instance, a Pod
named `rauthy-0` will be translated to `HQL_NODE_ID=1` automatically.
```

### Create and apply secrets

```
touch secrets.yaml
```

Paste the following content into the `secrets.yaml` file:

```yaml
apiVersion: v1
kind: Secret
metadata:
  name: rauthy-secrets
  namespace: rauthy
type: Opaque
stringData:
  HQL_S3_KEY:
  HQL_S3_SECRET:

  # Secrets for Raft internal authentication as well as for the Hiqlite API.
  # These must be at least 16 characters long and you should provide
  # different ones for both variables.
  HQL_SECRET_RAFT:
  HQL_SECRET_API:

  # The password for the Hiqlite dashboard as Argon2ID hash.
  # '123SuperMegaSafe' in this example
  #
  # You should only provide it, if you really need to access the DB
  # directly for some reasons.
  #HQL_PASSWORD_DASHBOARD=JGFyZ29uMmlkJHY9MTkkbT0zMix0PTIscD0xJE9FbFZURnAwU0V0bFJ6ZFBlSEZDT0EkTklCN0txTy8vanB4WFE5bUdCaVM2SlhraEpwaWVYOFRUNW5qdG9wcXkzQQ==

  # You need to define at least one valid encryption key.
  # These keys are used in various places, like for instance
  # encrypting confidential client secrets in the database, or
  # encryption cookies, and so on.
  #
  # The format must match:
  # ENC_KEYS: |-
  #   q6u26vXV/M0NFQzhSSldCY01rckJNa1JYZ3g2NUFtSnNOVGdoU0E=
  #   bVCyaggQ/UzluN29DZW41M3hTSkx6Y3NtZmRuQkR2TnJxUTYzcjQ=
  ENC_KEYS: |-

  # This identifies the key ID from the `ENC_KEYS` list, that
  # should actively be used for new encryption's.
  ENC_KEY_ACTIVE:

  # Needed for sending E-Mails for password resets and so on
  SMTP_PASSWORD:

  # The Webhook for Slack Notifications.
  # If left empty, no messages will be sent to Slack.
  #EVENT_SLACK_WEBHOOK=

  # Matrix variables for event notifications.
  # `EVENT_MATRIX_USER_ID` and `EVENT_MATRIX_ROOM_ID` are mandatory.
  # Depending on your Matrix setup, additionally one of
  # `EVENT_MATRIX_ACCESS_TOKEN` or `EVENT_MATRIX_USER_PASSWORD` 
  # is needed. If you log in to Matrix with User + Password, you 
  # may use `EVENT_MATRIX_USER_PASSWORD`.
  # If you log in via OIDC SSO (or just want to use a session token 
  # you can revoke), you should provide `EVENT_MATRIX_ACCESS_TOKEN`.
  # If both are given, the `EVENT_MATRIX_ACCESS_TOKEN` will be preferred.
  #
  # If left empty, no messages will be sent to Slack.
  # Format: `@<user_id>:<server address>`
  #EVENT_MATRIX_USER_ID=
  # Format: `!<random string>:<server address>`
  #EVENT_MATRIX_ROOM_ID=
  #EVENT_MATRIX_ACCESS_TOKEN=
  #EVENT_MATRIX_USER_PASSWORD=

  #PG_USER=
  #PG_PASSWORD=
```

All variables specified here should be out-commented in the `rauthy-config` from above.  
Make sure that things like `CACHE_AUTH_TOKEN` and `ENC_KEYS` are generated in a secure random way.

Generate a new encryption key with ID in the correct format.

```
echo "$(openssl rand -hex 4)/$(openssl rand -base64 32)"
```

Paste the String quoted in the secrets for `ENC_KEYS`.
The `ENC_KEY_ID` are the characters in the beginning until the first `/`, for instance when

```
❯ echo "$(openssl rand -hex 4)/$(openssl rand -base64 32)"
d4d1a581/mNIqEpxz4UudPggRpF1QJtjVdZ6JEeVAHepDLZZYI2M=
```

The `ENC_KEY_ID` would be

```
d4d1a581
```

You can generate safe values for both `HQL_SECRET_RAFT` and `HQL_SECRET_API` in many ways. You can just provide a random
alphanumeric value, which for instance:

```
cat /dev/urandom | tr -dc 'a-zA-Z0-9' | head -c48
```

or you can use the above `openssl` command again, even though Hiqlite does not need or utilize base64:

```
openssl rand -base64 48
```

If you plan on using S3 for backups, paste the proper values into `HQL_S3_KEY` and `HQL_S3_SECRET`, otherwise
out-comment them.

```admonish note
It seems that in some environments, the above `openssl` command does not output proper values, which will make Rauthy
panic on startup, when it checks the given values. If you run into that situation, you can generate them without 
`openssl` as well, with e.g:

<pre><code>echo "$(cat /dev/urandom | tr -dc 'a-zA-Z0-9' | head -c 6)/$(cat /dev/urandom | head -c 32 | base64)"</code></pre>
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
    # Ports 8100 and 8200 (by default) are used for the Hiqlite internal communication.
    - name: hiqlite-raft
      protocol: TCP
      port: 8100
      targetPort: 8100
    - name: hiqlite-api
      protocol: TCP
      port: 8200
      targetPort: 8200
    # Assuming that this example file will run behind a Kubernetes ingress and does
    # use HTTP internally.
    - name: http
      port: 8080
      targetPort: 8080
    # Uncomment, if you change to direct HTTPS without a reverse proxy
    #- name: https
    #  port: 8443
    #  targetPort: 8443
---
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: rauthy
  namespace: rauthy
  labels:
    app: rauthy
spec:
  serviceName: rauthy
  # Do not just scale up replicas without a proper HA Setup
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
          image: ghcr.io/sebadob/rauthy:0.29.4
          imagePullPolicy: IfNotPresent
          securityContext:
            # User ID 10001 is actually built into the container at the creation for
            # better security
            runAsUser: 10001
            runAsGroup: 10001
            allowPrivilegeEscalation: false
          ports:
            # Hiqlite internal ports
            - containerPort: 8100
            - containerPort: 8200
            # You may need to adjust this, if you decide to start in https only mode
            # or use another port
            - containerPort: 8080
            - containerPort: 8443
          env:
            # You must set both Hiqlite secrets even for a single node deployment
            - name: HQL_SECRET_RAFT
              valueFrom:
                secretKeyRef:
                  name: rauthy-secrets
                  key: HQL_SECRET_RAFT
            - name: HQL_SECRET_API
              valueFrom:
                secretKeyRef:
                  name: rauthy-secrets
                  key: HQL_SECRET_API

            # If you don't want to use S3 for backups, out-comment these 2.
            - name: HQL_S3_KEY
              valueFrom:
                secretKeyRef:
                  name: rauthy-secrets
                  key: HQL_S3_KEY
            - name: HQL_S3_SECRET
              valueFrom:
                secretKeyRef:
                  name: rauthy-secrets
                  key: HQL_S3_SECRET

            # Only out-comment if you use Postgres
            #- name: DATABASE_URL
            #  valueFrom:
            #    secretKeyRef:
            #      name: rauthy-secrets
            #      key: DATABASE_URL

            # Encryption keys used for encryption in many places
            - name: ENC_KEYS
              valueFrom:
                secretKeyRef:
                  name: rauthy-secrets
                  key: ENC_KEYS
            - name: ENC_KEY_ACTIVE
              valueFrom:
                secretKeyRef:
                  name: rauthy-secrets
                  key: ENC_KEY_ACTIVE

            - name: SMTP_PASSWORD
              valueFrom:
                secretKeyRef:
                  name: rauthy-secrets
                  key: SMTP_PASSWORD

            #- name: EVENT_SLACK_WEBHOOK
            #  valueFrom:
            #    secretKeyRef:
            #      name: rauthy-secrets
            #      key: EVENT_SLACK_WEBHOOK

            #- name: EVENT_MATRIX_USER_ID
            #  valueFrom:
            #    secretKeyRef:
            #      name: rauthy-secrets
            #      key: EVENT_MATRIX_USER_ID
            #- name: EVENT_MATRIX_ROOM_ID
            #  valueFrom:
            #    secretKeyRef:
            #      name: rauthy-secrets
            #      key: EVENT_MATRIX_ROOM_ID
            #- name: EVENT_MATRIX_ACCESS_TOKEN
            #  valueFrom:
            #    secretKeyRef:
            #      name: rauthy-secrets
            #      key: EVENT_MATRIX_ACCESS_TOKEN
            #- name: EVENT_MATRIX_USER_PASSWORD
            #  valueFrom:
            #    secretKeyRef:
            #      name: rauthy-secrets
            #      key: EVENT_MATRIX_USER_PASSWORD
          volumeMounts:
            - mountPath: /app/data
              name: rauthy-data
              readOnly: false
            - mountPath: /app/rauthy.cfg
              subPath: rauthy.cfg
              name: rauthy-config
              readOnly: true
          livenessProbe:
            httpGet:
              # You may need to adjust this, if you decide to start in https only
              # mode or use another port
              scheme: HTTP
              port: 8080
              #scheme: HTTPS
              #port: 8443
              path: /auth/v1/health
            initialDelaySeconds: 1
            periodSeconds: 30
          resources:
            requests:
              # Tune the memory requests value carefully. Make sure, that the
              # pods request at least:
              # `ARGON2_M_COST` / 1024 * `MAX_HASH_THREADS` Mi + idle memory
              memory: 164Mi
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
          configMap:
            name: rauthy-config
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

This example assumes, that the deployment will run behind a Kubernetes ingress resource of your choice.

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

The thing you need will be valid TLS certificates, of course. To get these, there are a lot of existing mechanisms.  
If you use an internal Certificate Authority (CA), you do have you own tools to work with this anyway. If, however, you
want to use something like [Let's Encrypt](https://letsencrypt.org/de/), I suggest to use the
[cert-manager](https://cert-manager.io/), which is easy and straight forward to use.

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

You can of course also provide TLS certificates for the Hiqlite internal communication. Two Independent networks are
created: one for the Raft-Internal network traffic like heartbeats and data replication, and a second one for the
"external" Hiqlite API. This is used by other Hiqlite cluster members for management purposes and to execute things
like consistent queries on the leader node.

You can provide TLS certificates for both of them independently via the following config variables:

```
## Hiqlite TLS

# If given, these keys / certificates will be used to establish
# TLS connections between nodes.
HQL_TLS_RAFT_KEY=tls/key.pem
HQL_TLS_RAFT_CERT=tls/cert-chain.pem
HQL_TLS_RAFT_DANGER_TLS_NO_VERIFY=true

HQL_TLS_API_KEY=tls/key.pem
HQL_TLS_API_CERT=tls/cert-chain.pem
HQL_TLS_API_DANGER_TLS_NO_VERIFY=true
```

#### Additional steps

There are a few more things to do when going into production, but these are the same for Kubernetes and Docker and will
be explained in later chapters.

You can now proceed with the [First Start](first_start.md) steps.
