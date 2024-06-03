# Getting Started

## Choose A Database

You only need to answer a single question to decide, which database you should use:

**Do you want / need a HA deployment?**

If the answer is **Yes**, choose **Postgres**, **otherwise** choose **SQLite**.

SQLite is no performance bottleneck at all. After some first very rough tests, it does not have problems with even
millions of users. The bottleneck will always be the password hashing algorithm settings, your needs for how secure
it should be and how many concurrent logins you want to be able to handle (more on that later).

```admonish hint
If you want to migrate from Postgres to SQLite at a later point, you can do this at any time very easily.
Just take a look at the [Reference Config](../config/config.html) and the variable `MIGRATE_DB_FROM`. 
```

## Container Images

Rauthy comes with different container images. The difference between them is not only x86 vs arm64, but the database
driver under the hood as well. The reason is, that almost all SQL queries are checked at compile time. To make this
possible, different images need to be created. Apart from the database driver, there is no difference between them.
You also can't use the "wrong" image by accident. If you try to use a Postgres image with a SQLite database URL and
vice versa, Rauthy will yell at you at startup and panic on purpose.

- The "normal" container images can be used for Postgres
- The `*-lite` images use an embedded SQLite
- The `MIGRATE_DB_FROM` (explained later) can be used with any combination of image / database

At the time of writing, you can run Rauthy either with [Docker](./docker.md) or inside [Kubernetes](./k8s.md).  
Both *Getting Started* guides do not cover all set up you might want to do for going into production. Especially the
*Docker* guide is more for testing.  
You should take a look at the [Reference Config](../config/config.html) to see, what you might want to configure
additionally. A dedicated *Going into production* guide will be written in the future.
