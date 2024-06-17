# High Availability

Rauthy is capable of running in a High Availability Mode (HA).

Some values, like authentication codes for instance, do live in the cache only. Additionally, there might come an
option with a future version which offers a special in-memory only mode in some situations.

Because of this, all instances create and share a single HA cache layer, which means at the same time, that you cannot
just scale up the replicas infinitely. The optimal amount of replicas for a HA mode would be 3, or if you need even higher 
resilience 5. More replicas should work just fine, but this has never been really tested and the performance will
degrade at some point.

To achieve the HA caching layer embedded directly into the application, I created a library (or crate in Rust terms)
called `redhac`.  
This crate will create each a gRPC server and a client part and each node will connect to all other ones. Once quorum
has been reached, a leader will be elected, which then will execute all insert requests by default to avoid overlaps
or inconsistencies and to guarantee a configured level of safety. Different so called `AckLevel` are available, like
`Quorum`, `Once` and `Leader` in addition to a direct cache put without any safeties.  
Rauthy uses different levels in different situations to provide real HA and sync all caches between the pods. This
means that you can loose a pod and still have the in-cache-only values available on the other ones.

This syncing of the cache is the reason why write performance will degrade, if you scale up too many replicas, which should
not really be necessary anyway. The best HA performance will be achieved with 3 replicas and then scaling up the 
resources for each pod before adding more replicas.

## Configuration

The way to configure the `HA_MODE` is optimized for a Kubernetes deployment but may seem a bit odd at the same time,
if you deploy somewhere else. You need to the following values in the config file:

### `HA_MODE`

The first one is easy, just set `HA_MODE=true`

### `HA_HOSTS`

The `HA_HOSTS` is working in a way, that it is really easy inside Kubernetes to configure it, as long as a 
StatefulSet is used for the deployment.

The way a cache node finds its members is by the `HA_HOSTS` and its own `HOSTNAME`.  
In the `HA_HOSTS`, add every cache member. For instance, if you want to use 3 replicas in HA mode which are running
and are deployed as a StatefulSet with the name `rauthy` again:

```
HA_HOSTS="http://rauthy-0.rauthy:8000, http://rauthy-1.rauthy:8000, http://rauthy-2.rauthy:8000"
```

The way it works:

1. A node gets its own hostname from the OS  
This is the reason, why you use a StatefulSet for the deployment, even without any volumes attached. For StatefulSet
called `rauthy`, the replicas will always have the names `rauthy-0`, `rauthy-1`, ..., which are at the same time the
hostnames inside the pod.
2. Find "me" inside the `HA_HOSTS` variable  
If the hostname cannot be found in the `HA_HOSTS`, the application will panic and exit because of a misconfiguration.
3. Use the port from the "me"-Entry that was found for the server part  
This means you do not need to specify the port in another variable which eliminates the risk of having inconsistencies
or a bad config in that case.
4. Extract "me" from the `HA_HOSTS`, take the leftover nodes as all cache members and connect to them
5. Once a quorum has been reached, a leader will be elected  
From that point on, the cache will start accepting requests
6. If the leader is lost - elect a new one - No values will be lost
7. If quorum is lost, the cache will be invalidated  
This happens for security reasons to provide cache inconsistencies. Better invalidate the cache and fetch the values
fresh from the DB or other cache members than working with possibly invalid values, which is especially true in an
authn / authz situation.

```admonish hint
If you are in an environment where the described mechanism with extracting the hostname would not work, for instance
you want a HA deployment with just different docker hosts, you can set the `HOSTNAME_OVERWRITE` for each instance to
match one of the `HA_HOSTS` entries.
```

### `CACHE_AUTH_TOKEN`

You need to set a secret for the `CACHE_AUTH_TOKEN` which was left out in the
[Getting Started](../getting_started/k8s.md#create-and-apply-secrets)

Just create a secret and add it in the same way:

```
echo "$(cat /dev/urandom | tr -dc 'a-zA-Z0-9' | head -c48)" | base64
```

### TLS

If you are using a service mesh like for instance [linkerd](https://linkerd.io/) which creates mTLS connections between
all pods by default, you can use the HA cache with just plain HTTP, since linkerd will encapsulate the traffic anyway.

You may then set

```
CACHE_TLS=false
```

to disable the use of TLS certificates between cache member.

However, if you do not have encryption between pods by default, I would highly recommend, that you use [TLS](tls.md). 
