# Getting Started

```admonish hint
Rauthy `v0.14` and beyond uses differnt container images for the different target databases.  
Until and including `v0.13`, rauthy used the `Any` driver from [sqlx](https://crates.io/crates/sqlx). However, this
driver has quite a few problems since sqlx 0.7, and I decided to migrate away from it, because the future of it seems
too uncertain to me.
```

- The "normal" container images can be used for Postgres
- The `*-lite` images use an embedded SQLite
- The `DB_MIGRATE_FROM` (explained later) can be used with any combination of image / database

At the time of writing, you can run Rauthy either with [Docker](./docker.md) or inside [Kubernetes](./k8s.md).  

