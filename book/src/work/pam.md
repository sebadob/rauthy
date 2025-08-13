# PAM

OIDC / OAuth covers almost all web apps, and for those that don't have any support, Rauthy comes with `forward_auth`
support. To not need an additional LDAP / AD / something similar for your backend and workstations, Rauthy comes with
its own custom PAM module. It does not just use JWT Tokens for logging in, but you can actually manage all your Linux
hosts, groups and users in different ways. You have the option to secure local logins to workstations via Yubikey
(only USB Passkeys supported, no QR-code / software keys), and all SSH logins can be done with ephemeral, auto-expiring
passwords, that you can generate via your Account dashboard, if an Admin has created a PAM user for you.
This means you basically have MFA-secured SSH logins without the need for any modifications or additional software on
your local SSH client, and you can use any SSH client from any machine securely, even if it's not your own.

Pam hosts, groups and users are managed via Rauthys Admin UI in the `PAM` section.

In addition to a PAM module, you get an NSS module and an NSS proxy that runs on each machine. You can dynamically
log in to any machine an Admin has given you access to. Users and groups are not added to local files, but will be
resolved via the network.

This module is published in a separate repo to avoid licensing issues, since it relies on some GPLv3 dependencies. You
can take a look at it here: [rauthy-pam-nss](https://github.com/sebadob/rauthy-pam-nss).

```admonish note
You will notice on the following pages and when testing the PAM setup via the Admin UI, that quite a few resources and
values are immutable after their initial creation. The reason is simply to avoid things like accidentially leaking
left-over user data, when for instance a user was deleted and a new one with the exact same name was created, but maybe
some files are left on some hosts, and so on. The same is true for instance for group names and types, just to avoid
confusion for admins.
```
