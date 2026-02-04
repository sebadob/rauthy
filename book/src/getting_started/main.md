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

Rauthy versions before `0.27.0` had different container images depending on the database you choose.
However, this is not the case anymore. There is only a single image which works with any
configuration. It works on `x86_64` and `arm64` architectures.

At the time of writing, you can run Rauthy either with [Docker](./docker.md) or
inside [Kubernetes](./k8s.md). \
Both *Getting Started* guides do not cover all set up you might want to do for going into
production. Especially the *Docker* guide is more for testing. \   
You should take a look at the [Reference Config](../config/config.html) to see, what you might want
to configure additionally. A dedicated *Going into production* guide will be written in the future.
