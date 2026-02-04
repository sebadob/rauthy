# Tuning

Rauthys default tuning is optimized for somewhat low resource usage while still providing good
performance and low latency up to a couple of thousands of users. If you run a tiny instance with
less than 100 users, or a very big one with users going into millions, or you need a very high
degree of concurrency, you can optimize the default tuning.
We will go through a few options to achieve what you need, no matter if you want even lower memory
usage or higher throughput.

## Database

### `cache_storage_disk`

By default, Rauthy keeps the WAL + Snapshots for the in-memory cache on disk. This makes the cache,
even though it exists in-memory only, persistent, because it can be rebuilt from disk after a
restart. Blacklisted IPs for instance exists in the cache only, as well as Auth Codes and some other
values. The other advantage is, that it reduces the overall memory usage a bit, because we don't
need
duplicate data in-memory just to make the Raft replication work.

These advantages are really nice and usually, because the throughput is really high anyway, you
should not need to worry about it. However, you can optionally keep everything cache related in
memory only. This boosts the throughput a lot, depending on the IOPS of your disk of course, at the
cost of higher memory usage and the fact, that nodes need to re-join the Raft cluster after a
restart and do a complete re-sync for latest Snapshot + Raft Logs.

To keep everything in memory only, set:

```toml
[cluster]
# Set to `false` to store Cache WAL files + Snapshots in-memory only.
# If you run a Cluster, a Node can re-sync cache data after a restart.
# However, if you restart too quickly or shut down the whole cluster,
# all your cached data will be gone.
# In-memory only greatly increases the throughput, though, so it
# depends on your needs, what you should prefer.
#
# default: true
# overwritten by: HQL_CACHE_STORAGE_DISK
cache_storage_disk = false
```

### `read_pool_size` / `pg_max_conn`

These values define the database connection pool sizes. `pg_max_conn` is by default set to `20`, and
`read_pool_size` to `4`.

The Postgres connections are a limit on the maximum and the application will dynamically open as
many as necessary. If you have a huge amount of concurrency, you might see improvements by
increasing `pg_max_conn`. Keep in mind though, that most data that is used for normal user
authentication is cached in memory, so it's possible that it will not help at all. If you want to
lower resource usage as much as possible, you might want to consider a lower value for
`pg_max_conn`, even though I would only do this for very tiny Rauthy instances with less than 50
users. The connection pool size for Postgres has a much higher impact than it does for Hiqlite,
because of the networking overhead.

For Hiqlite with `read_pool_size`, you might want to do the same of course. However, the pool is
statically built to reduce latency. On the other hand, a higher number of connections will only very
slightly increase memory usage. These connections are only for reading data and don't do anything
regarding writes. The Hiqlite writer will always be a fixed, single writer connection running on a
dedicated thread.

The Hiqlite read pool can be configured in the `cluster` section:

```toml
[cluster]
# The size of the pooled connections for local database reads.
#
# Do not confuse this with a pool size for network databases, as it
# is much more efficient. You can't really translate between them,
# because it depends on many things, but assuming a factor of 10 is
# a good start. This means, if you needed a (read) pool size of 40
# connections for something like a postgres before, you should start
# at a `read_pool_size` of 4.
#
# Keep in mind that this pool is only used for reads and writes will
# travel through the Raft and have their own dedicated connection.
#
# default: 4
# overwritten by: HQL_READ_POOL_SIZE
#read_pool_size = 4
```

The `pg_max_conn` is located in the `database` section:

```toml
[database]
# Max DB connections for the Postgres pool.
#
# default: 20
# overwritten by: PG_MAX_CONN
pg_max_conn = 20
```

### `log_sync`

When using Hiqlite, you can control the Raft Log sync / flush strategy. This has an immediate effect
on disk longevity and crash resistance, but of course, also on total throughput.

```toml
[cluster]
# Setting for Raft Log syncing / flushing.
#
# This value is probably the most important, depending on your needs.
# It has a huge impact on both performance and consistency.
#
# You can set 3 different levels:
# - immediate
# - immediate_async
# - interval_<time_ms> (e.g. interval_200)
#
# If you run a single instance "Cluster", you most probably want
# `immediate` to have the highest degree of consistency. If set
# to `immediate`, the Raft will block until data has been flushed
# to disk.
# `immediate` has a very huge negative impact on throughput, and
# it puts a lot of stress on your disk, which is important to
# consider for SSDs.
#
# `immediate_async` puts the same amount of stress on your SSD
# and flushes all buffers to disk immediately after a single
# Raft Log was saved. Unlike `immediate`, however, it does not
# wait for completion and directly returns `success` to the
# Raft Engine. You have a tiny bit worse consistency guarantees
# in exchange for basically the same throughput as with `interval`
# syncing.
#
# The `interval_<ms>` option will not flush immediately. Flushes
# will be triggered by an external ticker task every <ms> ms, and
# then only if buffers are dirty, meaning if any data was updated
# since the last sync. This setting has the highest throughput
# and puts the lowest amount of stress on your SSD.
#
# CAUTION: You probably never want to use this setting for a
# single instance, since it may be the case that if you are
# unlucky and your app crashes before buffers are synced, you might
# lose the last few WAL entries. Even though very unlikely in the
# real world (can be force-produced though).
#
# default: immediate_async
# overwritten by: HQL_LOG_SYNC
log_sync = "interval_200"
```

## `wal_size` + `logs_until_snapshot`

This applies to Hiqlite only as well. If you only run a small instance, you could save a little bit
of memory by adjusting the WAL file size and the threshold when to create a new Snapshot and cleanup
Raft Logs. The default values will handle as much throughput as your disk IOPS allow. However, you
could tune it down for small instances to e.g.

- `wal_size = 262144`
- `logs_until_snapshot = 1000`

```toml
[cluster]
# Hiqlite WAL files will always have a fixed size, even when they
# are still "empty", to reduce disk operations while writing. You
# can set the WAL size in bytes. The default value is 2 MiB, while
# the minimum size is 8 kiB.
# The value is given in bytes and should always be a multiple of 
# 4096.
#
# default: 2097152
# overwritten by: HQL_WAL_SIZE
wal_size = 2097152

# Sets the limit when the Raft will trigger the creation of a new
# state machine snapshot and purge all logs that are included in
# the snapshot.
# Higher values can achieve more throughput in really write heavy
# situations but will end up in more disk usage and longer
# snapshot creations / log purges.
#
# default: 10000
# overwritten by: HQL_LOGS_UNTIL_SNAPSHOT
logs_until_snapshot = 10000
```

This will trigger more often WAL log roll-overs, which of course does more syscalls. But since WAL
files are memory-mapped, you can save a little bit of memory at runtime. These values should always
be adjusted together. They both have an impact on how many WAL files will exist during normal
operation. ideally, you have 2-4 WAL files around during normal operation. At the same time, you
should make make these values too small so that Snapshots are not being created more than probably
every 15 minutes.

```admonish notice
You should not push the `wal_size` below `131072`. The benefits in memory usage will become very 
small in exchange for a lot more disk I/O. Also, the `wal_size` should always be a multiple of `4096` 
for optimal alignment. Values bigger than the default 2MiB will typically not have too much positive 
impact as well, as long as you are not having too many Snapshots because of a huge amount of 
concurrent users.
```

## HTTP Server

### `http_workers`

The `server.http_workers` variable defines the number of HTTP worker threads being spawned. This has
a big impact on the amount of memory used. This is especially important when you run Rauthy inside a
container on a bigger underlying host with many cores. Rauthy will spawn more workers depending on
the total number of cores, and not depending on any container limits you might have set, which it
cannot see.

The default value is:

- less than 4 CPU cores -> 1
- 4+ cores -> max(2, cores - MAX_HASH_THREADS - reserve), where `reserve` is 2 when `HIQLITE=true`
  and 1 otherwise

You almost **always want to tune this** to your needs, it has a big impact on memory! The best idea
is probably to use the same formula, but with respect to custom container limits, or whatever you
would want Rauthy to use.

```toml
[server]
# Limits the amount of HTTP worker threads. This value
# heavily impacts memory usage, even in idle. The default
# values are:
# - less than 4 CPU cores -> 1
# - 4+ cores -> max(2, cores - MAX_HASH_THREADS - reserve)
#   where `reserve` is 2 when `HIQLITE=true` and 1 otherwise.
#
# CAUTION: If you run your instance on a big underlying host,
# you almost always want to manually set an appropriate
# value. Rauthy can only see all available cores and not any
# possibly set container limits. This means if it runs inside
# a container on something like a 96 core host, Rauthy will
# by default spawn very many threads.
#
# overwritten by: HTTP_WORKERS
http_workers = 1
```

### `metrics_enable`

In newer versions, the `metrics_enable` is `false` by default and opt-in. You can enable prometheus
metrics with it. An independent HTTP server will be spawned, it will consume additional memory and
CPU. Leave it to `false` if not needed.

```toml
[server]
# To enable or disable the additional HTTP server to expose
# the /metrics endpoint.
#
# default: false
# overwritten by: METRICS_ENABLE
metrics_enable = false
```

### `swagger_ui_enable`

If enable `swagger_ui_enable` and set it to `true`, it will consume ~13 MB of additional memory. To
reduce overall memory fragmentation further down the road, if enabled, it will be initialized at the
very start of the application instead of being lazily initialized. If you don't need the API
documentation, leave it to the default, which is `false`.

```toml
[server]
# Can be set to `true` to enable the Swagger UI.
# This will consume ~13mb of additional memory.
#
# default: false
# overwritten by: SWAGGER_UI_ENABLE
swagger_ui_enable = false
```

## Password Hashing

### `argon2_m_cost` + `argon2_t_cost` + `argon2_p_cost` + `max_hash_threads`

Values for password hashing are probably the most important ones to tune overall. They do not only
dictate resource usage, but also the strength and security of password hashes. The password hashing
will always be the limiting factor, at least as long as not all of your users use Webauthn-only
accounts. This book, as well as the Admin UI, have dedicated sections and utilities for these
values. Keep in mind to tune them properly.

```toml
[hashing]
# Argon2ID hashing parameters. Take a look at the documentation
# for more information:
# https://sebadob.github.io/rauthy/config/argon2.html
# M_COST must never be below 32768 in production
# overwritten by: ARGON2_M_COST
argon2_m_cost = 131072
# T_COST must never be below 1 in production
# overwritten by: ARGON2_T_COST
argon2_t_cost = 4
# P_COST must never be below 2 in production
# overwritten by: ARGON2_P_COST
argon2_p_cost = 8

# Limits the maximum amount of parallel password hashes at the exact same time
# to never exceed system memory while still allowing a good amount of memory
# for the Argon2ID algorithm
#
# CAUTION: You must make sure, that you have at least
# (MAX_HASH_THREADS * ARGON2_M_COST / 1024) + idle memory of your deployment available.
#
# default: 2
# overwritten by: MAX_HASH_THREADS
max_hash_threads = 2
```

## Memory Allocator

Rauthy uses `jemalloc` under the hood, which you can tune via an environment variable, as long as
it's set before the application starts. A default setting is baked into the compiled binary, but
when you provide the env var, you can overwrite it. The default is tuned to be just fine for
instances with a couple of hundreds of users and focuses more on being efficient rather than
providing a high degree of concurrency.

If you have a bigger instance with many thousands or even millions of users, or just a tiny one with
probably less than 100, you can optimize it. There is only a small amount of room in terms of
lowering memory usage, but a lot of increasing concurrency at the cost of higher memory usage.

I will not go into the details of `jemalloc` here. If you are interested in this topic, you will
find lots of information via the search engine of your choice. I only want to give you recipes that
should just work.

```admonish caution
You need to set the `MALLOC_CONF` environment variable from the recipes below BEFORE the application 
starts, or otherwise it will be ignored. Anything like the `-e` flag for docker, or a regular 
environment variable inside K8s will work.
```

### Small Instance

If you run a small instance with less than 100 users, and you want to reduce the memory footprint as
much as possible, you have two options.

**1. Lowest memory usage**

This value will probably give you no noticeable performance hit while still providing a low memory
usage.

```
MALLOC_CONF=abort_conf:true,narenas:1,tcache_max:1024,dirty_decay_ms:1000,muzzy_decay_ms:1000
```

**2. Lowest possible memory at all costs.**

This will come with a **very big performance hit**! You should only use it, if you don't care about
performance at all. Imo, the performance hit is not worth the small savings compared to the config
above.

```
MALLOC_CONF=abort_conf:true,narenas:1,tcache:false,dirty_decay_ms:0,muzzy_decay_ms:0
```

### Medium Instance

If you run a medium instance with up to 500-1000 users, you are probably just fine with the
defaults. Stick to them. Just for reference, the currently used default config is this:

```
MALLOC_CONF=abort_conf:true,narenas:8,tcache_max:4096,dirty_decay_ms:5000,muzzy_decay_ms:5000
```

### Big Instance

A big instance is one with more than at least 1000 users. In this case, you might not get the
desired performance, especially if you have a higher value for `MAX_HASH_THREADS` or `HTTP_WORKERS`.
Most requests are so small, that they almost entirely fit in the thread-local cache for the
allocator, but if dynamic brotli compression kicks in, memory from an allocator arena will be
necessary most of the time. Because Rauthy does not need the arena's that often, the default value
of `4` is too low in such a case. You should set `narenas` from the string below at least equal to
the amount of CPU cores your Rauthy instance is assigned to. Depending on the amount or concurrent
logins, you may set `narenas` to 2-4x CPU cores at the cost of higher memory usage.

```
MALLOC_CONF=abort_conf:true,narenas:16,tcache_max:16384,dirty_decay_ms:10000,muzzy_decay_ms:10000
```

### Open End

For any instance with basically an open end of users, or if you have a very high degree of
concurrent logins (or you just don't care about low memory usage and only about max performance),
set `narenas` to 4x your CPU cores (higher values will start to have a negative impact). This config
will let you scale into millions of users easily.

```
MALLOC_CONF=abort_conf:true,narenas:64,tcache_max:32768,dirty_decay_ms:30000,muzzy_decay_ms:30000
```

```admonish note
The Memory Allocator tuning does not work on Windows msvc or freebsd targets, if you are running a 
custom-compiled version of Rauthy. It should work everywhere else though.
```
