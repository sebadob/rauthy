# PAM Groups

PAM Groups are independent of the general OIDC groups. This is the case for all PAM resources, because Linux regexes for
validation are much stricter. PAM groups can also do different things, depending on their type.

PAM Groups are the first resource you need for a new setup, because they are the glue between hosts and users and
manage general access.

## `wheel-rauthy`

This group is special. It will always exist, it will always have `gid` 100_000 and it cannot be deleted. Membership
for this group can also not be configured directly. Instead, it will be calculated dynamically for each user / host
combination depending on if the user is allowed to `wheel` in that particular host group. More on that later.

The idea of `wheel-rauthy` is that it should serve the same purpose as `wheel` on hosts, like e.g. allowing all members
in `wheel-rauthy` to use `sudo`. Theoretically, it could be merged with `wheel` on all hosts locally, but having a
distinct name provides more flexibility for configuration when you e.g. want to handle local users a bit differently
from Rauthy-managed ones.

## Types

You can create groups of 4 different types. Each type serves a specific purpose.

### `host` groups

`host` groups manage access to hosts. Each PAM Host must be assigned to exactly a single host group. When you assign
a PAM User as a member in a host group, this user will be allowed to log in to all hosts in this specific group, and it
will be rejected when the user is not a member. In addition, when you also allow `wheel` for a user in a host group,
this user will be dynamically assigned to the `wheel-rauthy` group mentioned above, and can therefore be used for things
like access to `sudo` and such.

You might be used to manage hosts by domain. Rauthy does not care about in which domain a host is and instead has the
host groups. The reason is that host groups provide a more fine-grained way of managing access and are therefore more
flexible. You can have multiple host groups in the same domain, and e.g. name your groups after teams or projects, while
the hosts are maybe technically in the same domain.

```admonish hint
If you want to delete a host group, you can only do this, if there is not a single host left that is assigned to this
group, to prevent invalid setups. You will only see the delete button, if the host group has no host assigned. If users
are still assigned, the relation will simply be deleted.
```

```admonish note
You will not see `host` groups in NSS lookups, like when you do `getent group`. They only exist for managing access
to hosts. All other group types will be resolved on hosts like you would expect it.
```

### `generic` groups

`generic` groups can be seen as a counterpart to your groups from `/etc/group`. The exception is, that they don't exist
locally and will be resolved via the NSS module. The important thing about them is, that their `gid` is globally unique,
not just on a host or inside a host group. This means if you e.g. have an NFS share and manage permissions via `gid` of
a generic group, it will be the exact same value with the exact same access on each host where you mount it.

### `local` groups

`generic` groups are almost the same, but when your `/etc/nsswitch.conf` is configured correctly, the `rauthy-nss` proxy
will merge them with groups that might exist locally in `/etc/group` on the hosts. This is for instance important for
situations like giving a user access to `docker`, which typically requires them to be a member of the `docker` group.
To achieve this, create a `local` group with the name `docker`, and the results will be merged and access granted.
Their `gid` is globally unique on Rauthy, but the NSS proxy will overwrite it on each host individually with the local
`gid` from the `/etc/group` entry.

### `user` groups

You cannot create these groups manually. When a new user is created, a `user` group with the same name as the username
will be created, just like you know if from any Linux system. You can assign other users to existing user groups, e.g.
when users want to be able to share certain files on hosts, but apart from that, they are managed automatically. Their
lifetime is bound to the users' lifetime. They provide the same guarantees as `generic` groups. Their `gid` will be
globally unique everywhere.
