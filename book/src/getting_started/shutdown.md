# Shutdown

The shutdown may confuse some people. For single instance deployments, it will only take a few
seconds. For a HA cluster node though, it will usually end up somewhere in the range of ~15 seconds.
Depending on the current cluster state, healthiness of remote nodes, a possibly ongoing leader
election, and so on, it may even take up to 25 - 30 seconds to do a graceful shutdown.
The reason behind this is that the shutdown logic tries to do it as smooth as possible, to e.g.
never have requests in Kubernetes going to an already shutting down node.

This behavior is absolutely normal and expected! It does not hang or get stuck. Even though it can
recover from it, you should never force-kill the process, as it will trigger an additional integrity
check of WAL files and a full rebuild of the DB (when
using [Hiqlite](https://github.com/sebadob/hiqlite)) during the next startup, so that a consistent
state can always be guaranteed.

```admonish caution
Some container runtimes do force-kill automatically after 10 seconds. This is for instance the case
for `podman`, or for `docker compose`.  

Make sure to adjust your shutdown timeout to at least 30 seconds.
```