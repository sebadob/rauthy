# Getting Started

## Choose A Database

Rauthy's default database is [Hiqlite](https://github.com/sebadob/hiqlite). Under the hood, it's
using SQLite, but it adds a additional layer on top making it highly-available using
the [Raft Consensus Algorithm](https://raft.github.io/). Don't let the SQLite engine under the hood
fool you, it will handle most probably anything you throw at it, as long as your disks are fast
enough. Hiqlite can easily saturate a 1GBit/s network connection with just database (write) traffic.
All reads are local, which means they are way faster than with Postgres in any scenario.

If you already have a Postgres up an running with everything set up anyway, you might want to choose
it as your main DB, but I do not recommend deploying a Postgres instance just for Rauthy. This would
be a waste of precious resources.

```admonish hint
If you want to migrate between databases at a later point, you can do this at any time very easily.
Just take a look at the [Reference Config](../config/config.html) and the variable `MIGRATE_DB_FROM`. 
```

## Container Images

Rauthys images are built for `x86_64` and `arm64` architectures. There is no native installer (
yet?), but if you want to run it outside a container, you can either just copy the binary out of if,
or you can build from source pretty easily. Take a look at
the [CONTRIBUTING.md](https://github.com/sebadob/rauthy/blob/main/CONTRIBUTING.md) for more
information.

At the time of writing, you can run Rauthy either with [Docker](./docker.md) (or any other
compatible container runtime) or inside [Kubernetes](./k8s.md). Both *Getting Started* guides do not
cover all setup you might want to do for going into production. Especially the *Docker* guide is
more for testing.

You should take a look at the [Reference Config](../config/config.html) to see, what you might want
to configure additionally, or for a more minimal setup
the [Minimal Production Config](../config/config_minimal.md).
