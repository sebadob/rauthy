# High Availability

Rauthy is capable of running in a High Availability Mode (HA).

Some values, like authentication codes for instance, do live in the cache only. Because of this, all instances create
and share a single HA cache layer, which means at the same time, that you cannot just scale up the replicas infinitely
without adjusting the config. The optimal amount of replicas for a HA mode would be 3, or if you need even higher
resilience 5. More replicas should work just fine, but at some point, the write throughput will degrade.

The Cache layer uses [Hiqlite](https://github.com/sebadob/hiqlite). It uses the Raft algorithm under the hood to achieve
consistency.

```admonish caution
Even though everything is authenticated, you should not expose the
Hiqlite ports to the public, if not really necessary for some reason. You configure these ports with the `cluster.nodes`
config value.
```

## Configuration

Earlier versions of Rauthy have been using [redhac](https://github.com/sebadob/redhac) for the HA cache layer. While
`redhac` was working fine, it had a few design issues I wanted to get rid of. Since `v0.26.0`, Rauthy uses the
above-mentioned [Hiqlite](https://github.com/sebadob/hiqlite) instead. You only need to configure a few variables:

### `node_id`

The `cluster.node_id` is mandatory, even for a single replica deployment with only a single node in `cluster.nodes`.
If you deploy Rauthy as a StatefulSet inside Kubernetes, you can ignore this value and just set `HQL_NODE_ID_FROM`
below. If you deploy anywhere else, or you are not using a StatefulSet, you need to set the `node_id_from` to tell
Rauthy
which node of the Raft cluster it should be.

```toml
[cluster]
# The node id must exist in the nodes and there must always be
# at least a node with ID 1
# Will be ignored if `node_id_from = k8s`
#
# At least `node_id_from` or `node_id` are required.
#
# default: 0 (invalid)
# overwritten by: HQL_NODE_ID
node_id = 1
```

### `node_id_from`

If you deploy to Kubernetes as a StatefulSet, you should ignore the `cluster.node_id` and just set
`cluster.node_id_from = "k8s"`. This will parse the correct NodeID from the Pod hostname, so you don't have to worry
about it.

```toml

[cluster]
# Can be set to 'k8s' to try to split off the node id from the hostname
# when Hiqlite is running as a StatefulSet inside Kubernetes.
#
# default: unset
# overwritten by: HQL_NODE_ID_FROM
node_id_from = "k8s"
```

### `nodes`

This value defines the Raft members. It must be given even if you just deploy a single instance. The description from
the reference config should be clear enough:

```toml
[cluster]
# All cluster member nodes. For a single instance deployment,
# `"1 localhost:8100 localhost:8200"` will work just fine.
# Each array value must have the following format:
# `id addr_raft addr_api`
#
# default: ["1 localhost:8100 localhost:8200"]
# overwritten by: HQL_NODES
nodes = [
    "1 localhost:8100 localhost:8200",
    #    "2 localhost:8101 localhost:8201",
    #    "3 localhost:8102 localhost:8202",
]
```

### `secret_raft` + `secret_api`

Since you need both `cluster.secret_raft` and `cluster.secret_api` in any case, there is nothing to change here. These
define the secrets being used internally to authenticate against the Raft or the API server for `Hiqlite`.
You can generate safe values with for instance

```
cat /dev/urandom | tr -dc 'a-zA-Z0-9' | head -c48
```

### TLS

If you are using a service mesh like for instance [linkerd](https://linkerd.io/) which creates mTLS connections between
all pods by default, you can use the HA cache with just plain HTTP, since `linkerd` will encapsulate the traffic anyway.
In this case, there is nothing to do.

However, if you do not have encryption between pods by default, I would highly recommend, that you use [TLS](tls.md). 
