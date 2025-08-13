# PAM Users

A PAM User is always linked to a "real" user and after it was created, it is immutable and automatically bound to the
lifetime of the "real" user. You cannot change the `username` afterward or even delete it and create a new one easily.
The reason is the same as for the immutable group names, to never accidentally leak some left-over files. There is also
typically no need to ever change these names, as it would just be confusing anyway.

A `uid` is globally unique over all hosts, just as `gid`s for groups. This means you can rely on it even for managing
NFS shares and things like that. The only value you can change for a user after the creation is the `shell`, but this
will just stay the default `/bin/bash` in probably almost all cases anyway.

The more important section for a user is the groups membership. Groups are separated by type, and these types are
explained in the [PAM Groups](pam_groups.md#types) doc. At this point, there should be not open questions about them.

PAM Users are pretty simple. The most important thing about them is their additional `username`, which must follow
Linux username regexes. Technically, in most situations, providing your email address from Rauthy should work, but there
are drawbacks and in some cases, such usernames lead to errors and not working commands. If Rauthy had chosen emails as
Linux usernames, you would also do very confusing stuff like `ssh my.user@example.com@host`, which is not only hard to
read, but does not even work on some systems.

Users are not allowed to choose their Linux username themselves. This must be done by a Rauthy admin to avoid users
trying things like `root`, `postgres` or `systemd`, which would be very bad of course. An admin needs to configure
access to hosts anyway, so there is no real drawback, that users cannot choose on their own. Maintaining a blacklist
with all possibly problematic usernames across all distros and systems is actually impossible, as Rauthy cannot even
know which names might already be special in some companies and situations.

As soon as an Admin has configured a Linux user for a "real" user, this user can log in to the account dashboard and a
PAM section will pop up in the tab bar. Here you can generate the auto-expiring Remote PAM Passwords that are used for
basically all actions via SSH like logging in and `sudo`, and you will see a list of all hosts you have access to. For
these PAM passwords, you have 2 static config varibales:

```toml
[pam]
# The length of newly generated PAM remote passwords via the
# account dashboard. The default is fine as long as you can copy
# & paste them. You may want to reduce the length here if you e.g.
# occasionally generate them on mobile and need to type them
# manually into some terminal.
#
# default: 24
# overwritten by: PAM_REMOTE_PASSWORD_LEN
remote_password_len = 24

# The TTL for newly generated PAM remote passwords in seconds.
# The default gives you plenty of time to open a few sessions in
# some terminals and maybe switch to `root` on some remote machines,
# while still expiring quick enough to be secure.
#
# default: 120
# overwritten by: PAM_REMOTE_PASSWORD_TTL
remote_password_ttl = 120
```
