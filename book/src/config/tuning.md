# Tuning

Rauthys default tuning is optimized for somewhat low resource usage while still providing good performance and low
latency up to a couple of thousands of users. If you run a tiny instance with less than 100 users, or a very big one
with users going into millions, or you need a very high degree of concurrency, you can optimize the default tuning.
We will go through a few options to achieve what you need, no matter if you want even lower memory usage or higher
throughput.

## Database

### `HQL_CACHE_STORAGE_DISK`

By default, Rauthy keeps the WAL + Snapshots for the in-memory cache on disk. This makes the cache, even though it
exists in-memory only, persistent, because it can be rebuilt from disk after a restart. Blacklisted IPs for instance
exists in the cache only, as well as Auth Codes and some other values. The other advantage is, that it reduces the
overall memory usage a bit, because we don't need duplicate data in-memory just to make the Raft replication work.

These advantages are really nice and usually, because the throughput is really high anyway, you should not need to
worry about it. However, you can optionally keep everything cache related in memory only. This boosts the throughput
a lot, depending on the IOPS of your disk of course, at the cost of higher memory usage and the fact, that nodes need
to re-join the Raft cluster after a restart and do a complete re-sync for latest Snapshot + Raft Logs.

To keep everything in memory only, set:

```
HQL_CACHE_STORAGE_DISK=false
```

### `HQL_READ_POOL_SIZE` / `PG_MAX_CONN`

These values define the database connection pool sizes. `PG_MAX_CONN` is by default set to `20`, and
`HQL_READ_POOL_SIZE` to `4`.

The Postgres connections are a limit on the maximum and the application will dynamically open as many as necessary. If
you have a huge amount of concurrency, you might see improvements by increasing `PG_MAX_CONN`. Keep in mind though, that
most data that is used for normal user authentication is cached in memory, so it's possible that it will not help at
all. If you want to lower resource usage as much as possible, you might want to consider a lower value for
`PG_MAX_CONN`, even though I would only do this for very tiny Rauthy instances with less than 50 users. The connection
pool size for Postgres has a much higher impact than it does for Hiqlite, because of the networking overhead.

For Hiqlite with `HQL_READ_POOL_SIZE`, you might want to do the same of course. However, the pool is statically built to
reduce latency. On the other hand, a higher amount of connections will only very slightly increase memory usage. These
connections are only for reading data and don't do anything regarding writes. The Hiqlite writer will always be a fixed,
single writer connection running on a dedicated thread.

### `HQL_SYNC_IMMEDIATE`

TODO

## HTTP Server

### `HTTP_WORKERS`

The `HTTP_WORKERS` variable defines the amount of HTTP worker threads being spawned. This has a huge impact on the
amount of memory used, even in idle. This is especially important when you run Rauthy inside a container on a bigger
underlying host with many cores. Rauthy will spawn more workers depending on the total amount of cores, and not
depending on any container limits you might have set, which it cannot see.

The default value is:

- less than 4 CPU cores -> 1
- 4+ cores -> max(2, cores - MAX_HASH_THREADS - reserve), where `reserve` is 2 when `HIQLITE=true` and 1 otherwise

You almost **always want to tune this** to your needs, it has a big impact on memory! The best idea is probably to use
the same formula, but with respect to custom container limits, or whatever you would want Rauthy to use.

### `METRICS_ENABLE` + `SWAGGER_UI_INTERNAL` + `SWAGGER_UI_EXTERNAL`

In newer versions, the `METRICS_ENABLE` is `false` by default and opt-in. You can enable prometheus metrics with it and
also expose the `SWAGGER_UI_INTERNAL`. An independent HTTP server will be spawned and if both values are active, it will
consume an additional ~18 - 20 MB of memory.

If you then also enable `SWAGGER_UI_EXTERNAL`, it will be another ~12.8MB for the main server **PER HTTP_WORKER**!
Unfortunately, the Swagger UI cannot be boxed and made more efficient. The whole dataset is copied for each thread!

So, if you don't need metrics and especially the Swagger UIs, keep them disabled.

## Password Hashing

### `ARGON2_M_COST` + `ARGON2_T_COST` + `ARGON2_P_COST` + `MAX_HASH_THREADS`

Values for password hashing are probably the most important ones to tune overall. They do not only dictate resource
usage, but also the strength and security of password hashes. The password hashing will always be the limiting factor,
at least as long as not all of your users use Webauthn-only accounts. This book, as well as the Admin UI, have dedicated
sections and utilities for these values. Just keep in mind to tune them properly.

## Memory Allocator

Rauthy uses `jemalloc` under the hood, which you can tune via an environment variable, as long as it's set before the
application starts. A default setting is baked into the compiled binary, but when you provide the env var, you can
overwrite it. The default is tuned to be just fine for instances with a couple of hundreds of users and focuses more
on being efficient rather than providing a high degree of concurrency.

If you have a bigger instance with many thousands or even millions of users, or just a tiny one with probably less than
100, you can optimize it. There is not too much room in terms of lowering memory usage, but a lot of increasing
concurrency at the cost of higher memory usage.

I will not go into the details of `jemalloc` here. If you are interested in this topic, you will find lots of
information via the search engine of your choice. I only want to give you recipes that should just work.

```admonish caution
You need to set the `MALLOC_CONF` environment variable from the recipes below BEFORE the application starts, or 
otherwise it will be ignored. Anything like the `-e` flag for docker, or a regular environment variable inside K8s will 
work.
```

### Small Instance

If you run a small instance with less than 100 users and you want to reduce the memory footprint as much as possible,
you have two options.

**1. Lowest memory usage**

This value will probably give you no noticeable performance hit while still providing a low memory usage.

```
MALLOC_CONF=abort_conf:true,narenas:1,tcache_max:1024,dirty_decay_ms:1000,muzzy_decay_ms:1000
```

**2. Lowest possible memory at all costs.**

This will come with a **very big performance hit**! You should only use it, if you don't care about performance at all.

```
MALLOC_CONF=abort_conf:true,narenas:1,tcache:false,dirty_decay_ms:0,muzzy_decay_ms:0
```

### Medium Instance

If you run a medium instance with up to 500-1000 users, you are probably just fine with the defaults. Stick to them.

### Big Instance

A big instance is one with more than at least 1000 users. In this case, you might not get the desired performance,
especially if you have a higher value for `MAX_HASH_THREADS` or `HTTP_WORKERS`. Most requests are so small, that they
almost entirely fit in the thread-local cache for the allocator, but if dynamic brotli compression kicks in, memory
from an allocator arena will be necessary most of the time. Because Rauthy does not need the arena's that often, the
default value of `4` is too low in such a case. You should set at least `narenas` from the string below equal to the
amount of CPU cores your Rauthy instance is assigned to. Depending on the amount or concurrent logins, you can even set
`narenas` to 2-4x CPU cores at the cost of higher memory usage.

```
MALLOC_CONF=abort_conf:true,narenas:16,tcache_max:16384,dirty_decay_ms:10000,muzzy_decay_ms:10000
```

### Open End

For any instance with basically an open end of users, or if you have a very high degree of concurrent logins (or you
just
don't care about low memory usage and only about max performance), set `narenas` to 4x your CPU cores (higher values
will start to have a negative impact).

```
MALLOC_CONF=abort_conf:true,narenas:64,tcache_max:32768,dirty_decay_ms:30000,muzzy_decay_ms:30000
```

```admonish note
The Memory Allocator tuning does not work on Windows msvc targets, if you are running a custom-compiled version of 
Rauthy. It should work everywhere else though.
```
