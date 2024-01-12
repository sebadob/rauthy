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
If you use SQLite, you can include the DATABASE_URL in the config, since it does not contain a password, but
never do this for Postgres!
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
data:
  # The CACHE_AUTH_TOKEN is only needed for a deployment with HA_MODE == true
  # Secret token, which is used to authenticate the cache members
  #CACHE_AUTH_TOKEN:
  
  # The database driver will be chosen at runtime depending on the given DATABASE_URL format. Examples:
  # Sqlite: 'sqlite:data/rauthy.db' or 'sqlite::memory:'
  # Postgres: 'postgresql://User:PasswordWithoutSpecialCharacters@localhost:5432/DatabaseName'
  DATABASE_URL: 
    
  # Format: "key_id/enc_key another_key_id/another_enc_key" - the enc_key itself must be exactly 32 characters long and
  # and should not contain special characters.
  # The ID must match '[a-zA-Z0-9]{2,20}'
  ENC_KEYS:
  
  # Needed for sending E-Mails for password resets and so on
  SMTP_PASSWORD:
```

The secrets need to be base64 encoded. If you are on Linux, you can do this in the shell easily. If not, use
any tool you like.  
Make sure that things like `CACHE_AUTH_TOKEN` (only needed with `HA_MODE == true`) and `ENC_KEYS` are generated
in a secure random way.

The `DATABASE_URL` with SQLite, like used in this example, does not contain sensitive information, but we will
create it as a secret anyway to have an easier optional migration to postgres later on.

```
echo 'sqlite:data/rauthy.db' | base64
```

Generate a new encryption key with ID in the correct format.

```
echo "$(cat /dev/urandom | tr -dc 'a-zA-Z0-9' | head -c8)/$(cat /dev/urandom | head -c32 | base64)"  | base64
```

Paste the base64 String in the secrets for `ENC_KEYS`.  
To extract the `ENC_KEY_ID`, which needs to be added to the config from Step 2:

```
echo -n PasteTheGeneratedBase64Here | base64 -d | cut -d/ -f1
```

And finally, the `SMTP_PASSWORD`

```
echo 'PasteYourSMTPPasswordHere' | base64
```

Paste all the generated secrets into the `secrets.yaml` file and the `ENC_KEY_ID` into the `config.yaml` from the step
before.

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
    # If you use the HA feature later on, the port over which the cache layer does
    # communicate.
    - name: cache
      port: 8000
      targetPort: 8000
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
      securityContext:
        fsGroup: 10001
      containers:
        - name: rauthy
          image: ghcr.io/sebadob/rauthy:0.20.1-lite
          imagePullPolicy: IfNotPresent
          securityContext:
            # User ID 10001 is actually built into the container at the creation for
            # better security
            runAsUser: 10001
            runAsGroup: 10001
            allowPrivilegeEscalation: false
          ports:
            - containerPort: 8000
            # You may need to adjust this, if you decide to start in https only mode
            # or use another port
            - containerPort: 8080
            - containerPort: 8443
          env:
            - name: DATABASE_URL
              valueFrom:
                secretKeyRef:
                  name: rauthy-secrets
                  key: DATABASE_URL
            - name: ENC_KEYS
              valueFrom:
                secretKeyRef:
                  name: rauthy-secrets
                  key: ENC_KEYS
            - name: SMTP_PASSWORD
              valueFrom:
                secretKeyRef:
                  name: rauthy-secrets
                  key: SMTP_PASSWORD
          volumeMounts:
            - mountPath: /app/data
              name: rauthy-data
              readOnly: false
            - mountPath: /app/rauthy.cfg
              subPath: rauthy.cfg
              name: rauthy-config
              readOnly: true
          readinessProbe:
            httpGet:
              # You may need to adjust this, if you decide to start in https only
              # mode or use another port
              scheme: HTTP
              port: 8080
              #scheme: HTTPS
              #port: 8443
              path: /auth/v1/ping
            initialDelaySeconds: 1
            periodSeconds: 10
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
              # `ARGON2_M_COST` / 1024 * `MAX_HASH_THREADS` Mi
              # With SQLite: for small deployments, add additional ~20-30Mi for
              # "the rest", for larger ones ~50-70 Mi should be enough.
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
              cpu: 1000m
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
        #storageClassName: provideIfYouHaveMultipleOnes
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
  name: rauthy-https
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

#### Additional steps

There are a few more things to do when going into production, but these are the same for Kubernetes and Docker and will
be explained in later chapters.

You can now proceed with the [First Start](first_start.md) steps.
