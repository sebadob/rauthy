# High Availability

Rauthy is capable of running in a High Availability Mode (HA).

Some values, like authentication codes for instance, do live in the cache only. Additionally, there might come an
option with a future version which offers a special in-memory only mode in some situations.

Because of this, all instances create and share a single HA cache layer, which means at the same time, that you cannot
just scale up the replicas infinitely without adjusting the config. The optimal amount of replicas for a HA mode would
be 3, or if you need even higher resilience 5. More replicas should work just fine, but this has never been really
tested and the performance will degrade at some point.

The Cache layer uses another project of mine called [Hiqlite](https://github.com/sebadob/hiqlite). It uses the Raft
algorithm under the hood to achieve consistency.

```admonish caution
Even though everything is authenticated, you should not expose the
Hiqlite ports to the public, if not really necessary for some reason. You configure these ports with the `HQL_NODES`
config value in the `CACHE` section.
```

## Configuration

Earlier versions of Rauthy have been using [redhac](https://github.com/sebadob/redhac) for the HA cache layer. While
`redhac` was working fine, it had a few design issues I wanted to get rid of. Since `v0.26.0`, Rauthy uses the above
mentioned [Hiqlite](https://github.com/sebadob/hiqlite) instead. You only need to configure a few variables:

### `HQL_NODE_ID`

The `HQL_NODE_ID` is mandatory, even for a single replica deployment with only a single node in `HQL_NODES`.
If you deploy Rauthy as a StatefulSet inside Kubernetes, you can ignore this value and just set `HQL_NODE_ID_FROM`
below. If you deploy anywere else or you are not using a StatefulSet, you need to set the `HQL_NODE_ID` to tell Rauthy
which node of the Raft cluster it should be.

```
# The node id must exist in the nodes and there must always be
# at least a node with ID 1
# Will be ignored if `HQL_NODE_ID_FROM=k8s`
HQL_NODE_ID=1
```

### `HQL_NODE_ID_FROM`

If you deploy to Kubernetes as a StatefulSet, you should ignore the `HQL_NODE_ID` and just set `HQL_NODE_ID_FROM=k8s`.
This will parse the correct NodeID from the Pod hostname, so you don't have to worry about it.

```
# Can be set to 'k8s' to try to split off the node id from the hostname
# when Hiqlite is running as a StatefulSet inside Kubernetes.
#HQL_NODE_ID_FROM=k8s
```

### `HQL_NODES`

Using this value, you defined the Cache / Raft members. This must be given even if you just deploy a single instance.
The description from the reference config should be clear enough:

```
# All cluster member nodes.
# To make setting the env var easy, the values are separated by `\s`
# while nodes are separated by `\n`
# in the following format:
#
# id addr_raft addr_api
# id addr_raft addr_api
# id addr_raft addr_api
#
# 2 nodes must be separated by 2 `\n`
HQL_NODES="
1 localhost:8100 localhost:8200
"
```

### `HQL_SECRET_RAFT` + `HQL_SECRET_API`

Since you need both `HQL_SECRET_RAFT` and `HQL_SECRET_API` in any case, there is nothing to change here. These define
the secrets being used internally to authenticate against the Raft or the API server for `Hiqlite`.
You can generate safe values with for instance

```
cat /dev/urandom | tr -dc 'a-zA-Z0-9' | head -c48
```

### TLS

If you are using a service mesh like for instance [linkerd](https://linkerd.io/) which creates mTLS connections between
all pods by default, you can use the HA cache with just plain HTTP, since `linkerd` will encapsulate the traffic anyway.
In this case, there is nothing to do.

However, if you do not have encryption between pods by default, I would highly recommend, that you use [TLS](tls.md). 
