# Rauthy Helm chart

A Helm chart for [Rauthy](https://github.com/sebadob/rauthy), a memory-safe
OpenID Connect provider and SSO solution. It deploys Rauthy as a `StatefulSet`
forming a [Hiqlite](https://github.com/sebadob/hiqlite) Raft cluster.

## Design

This chart is a thin, faithful Helmification of the Kubernetes deployment
documented in the [Rauthy book](https://sebadob.github.io/rauthy/getting_started/k8s.html).
It deliberately keeps configuration out of the chart:

- **You own the config.** You provide the complete `config.toml` (the
  documented Reference Config, or the Minimal Production Config) as a Secret.
  The chart mounts it verbatim at `/app/config.toml`. The chart does **not**
  re-template individual config values - every Rauthy setting is already
  documented per-value in the book, and duplicating it into chart values would
  only add drift and maintenance overhead.
- **The chart owns only the Raft topology.** The single thing it injects over
  your config is the cluster membership: `HQL_NODE_ID_FROM=k8s` and `HQL_NODES`,
  both derived from `replicas`. These cannot live in a static file because they
  depend on the StatefulSet and the install/upgrade-time replica count.
- **No autoscaling.** A Raft cluster cannot be safely scaled down, so there is
  no `HorizontalPodAutoscaler` and no scale-down path (see [Scaling](#scaling)).
- **No generated secrets.** The chart never invents `secret_raft`, `secret_api`
  or encryption keys for you; you generate and manage them, as in the book.

## Prerequisites

- Kubernetes 1.23+
- Helm 3.8+
- A `config.toml` with valid `cluster.secret_raft`, `cluster.secret_api` and
  `encryption.keys` / `key_active`. See `files/config.toml.example` and the
  book's [Minimal Production Config](https://sebadob.github.io/rauthy/config/config_minimal.html).

## Install

Provide your config either inline or, recommended, via an existing Secret.

### Option A - existing Secret (recommended)

```bash
# Your config.toml must contain the cluster secrets and encryption keys.
kubectl create namespace rauthy
kubectl -n rauthy create secret generic rauthy-config --from-file=config.toml

helm install rauthy ./charts/rauthy \
  -n rauthy \
  --set config.existingSecret=rauthy-config
```

### Option B - inline config

```bash
helm install rauthy ./charts/rauthy \
  -n rauthy --create-namespace \
  --set-file config.content=./config.toml
```

A single `helm install` (or `helm upgrade`) is all that is needed; there are no
manual steps to run beforehand beyond having your config.

## Upgrade

```bash
helm upgrade rauthy ./charts/rauthy -n rauthy --reuse-values
```

Rolling updates are safe: each node drains Raft leadership on graceful shutdown
and re-joins from its persistent WAL on restart. Keep
`terminationGracePeriodSeconds` generous (default 30s) so leadership hand-off
can complete.

## High availability and scaling

Rauthy uses an in-memory cache backed by a disk-persisted Hiqlite Raft log, so
you cannot just bump replicas on a plain Deployment - it must be a Raft cluster.

- Recommended HA size is **3** (or **5** for higher resilience). More works,
  but Raft write throughput degrades.
- A persistent volume is required (provided per-pod via `volumeClaimTemplates`)
  so a restarted node re-joins from its WAL instead of doing a full re-sync.
- `spreadAcrossNodes` (default true) adds a preferred pod anti-affinity so
  replicas land on different nodes; set a required rule via `affinity` for
  strict production spreading.
- A `PodDisruptionBudget` (`maxUnavailable: 1`) is created automatically when
  `replicas > 1`.

### Scaling

**Scale up only.** To grow the cluster, raise `replicas` to a higher odd number
and `helm upgrade`:

```bash
helm upgrade rauthy ./charts/rauthy -n rauthy --reuse-values --set replicas=3
```

You must **never scale back down**. Raft membership is derived from `replicas`;
a removed member remains in every other node's configuration and the cluster
will keep trying to reach it. This is also why the chart ships no autoscaler.

## In-cluster TLS

If your mesh does not already encrypt pod-to-pod traffic, configure Hiqlite's
TLS (`cluster.tls_*`, e.g. `tls_auto_certificates`) in your `config.toml`. The
Raft (`8100`) and Hiqlite-API (`8200`) ports are exposed only on the headless
service and must never be routed through your ingress.

## Values

| Key | Default | Description |
|---|---|---|
| `replicas` | `1` | Raft cluster members. Scale up only. |
| `image.repository` | `ghcr.io/sebadob/rauthy` | Image repository. |
| `image.tag` | `""` | Defaults to the chart `appVersion`. |
| `image.pullPolicy` | `IfNotPresent` | Image pull policy. |
| `imagePullSecrets` | `[]` | Pull secrets for private registries. |
| `config.existingSecret` | `""` | Secret holding `config.toml` (recommended). |
| `config.existingSecretKey` | `config.toml` | Key inside the Secret. |
| `config.content` | `""` | Inline `config.toml`; chart creates the Secret. |
| `service.type` | `ClusterIP` | Service type for the HTTP port. |
| `service.port` | `8080` | HTTP service/target port. |
| `ingress.enabled` | `false` | Create an Ingress. |
| `ingress.className` | `""` | IngressClass name. |
| `ingress.annotations` | `{}` | Ingress annotations. |
| `ingress.host` | `""` | Host; should match `server.pub_url`. |
| `ingress.tlsSecret` | `""` | TLS Secret for the host. |
| `persistence.size` | `128Mi` | Per-pod PVC size for `/app/data`. |
| `persistence.storageClass` | `""` | PVC StorageClass (empty = default). |
| `resources.requests` | `cpu: 100m, memory: 64Mi` | Resource requests. |
| `resources.limits` | `{}` | Resource limits (mind Argon2 memory). |
| `podSecurityContext` | `fsGroup: 10001` | Pod security context. |
| `securityContext` | non-root, drop ALL | Container security context. |
| `spreadAcrossNodes` | `true` | Preferred anti-affinity when `affinity` empty. |
| `affinity` | `{}` | Full affinity override. |
| `nodeSelector` | `{}` | Node selector. |
| `tolerations` | `[]` | Tolerations. |
| `terminationGracePeriodSeconds` | `30` | Graceful shutdown budget. |
| `extraEnv` | `[]` | Extra container env vars. |
| `podAnnotations` | `{}` | Pod annotations. |
| `podLabels` | `{}` | Pod labels. |
| `nameOverride` / `fullnameOverride` | `""` | Name overrides. |
