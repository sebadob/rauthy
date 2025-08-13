# PAM Hosts

PAM Hosts are usually the 2nd resource you need, when creating a fresh setup. A default installation of Rauthy will
automatically come with a `host` group named `default` to get you started, but you can delete it and create your own,
and of course additional ones to group your hosts properly. However, at least a single host group must exist, before
you can create your first PAM Host. The hostname you provide must follow the Linux rules for hostnames.

PAM Hosts should be pretty much self-explanatory. Each one must have a unique hostname and only a single host group
assignment to manage access for users. You can set multiple IPs and aliases for a host, and they will show up in NSS
lookups, like e.g. `getent hosts`. They are basically an extension to your `/etc/hosts` file and (if configured
correctly), they will overwrite DNS resolutions, or provide a Hostname -> IP resolve even if no DNS exists, just like
your `/etc/hosts` file.

## Force MFA

This setting can be used to further lock down access to a host, in addition to membership in a host group. If set, only
accounts that are MFA secured are allowed to log in to this host. A local login would request a Passkey. For logins via
ssh, you need to generate a Remote PAM Password in your account dashboard and only when generated from a user session
that is MFA-secured, you can log in.

## Local Password Login

This option is a bit special to solve a niche issue. If you have users that should be allowed to log in locally to
workstations, and they don't have USB Passkeys (which are the only ones that actually make sense), but instead something
that generates a QR code via smartphone, or Passkeys stored in a password manager (seriously, don't do that...), you can
set this option.

If this is set, a local login will NEVER request a passkey, even if the account is MFA secured. Instead, it will only
request the users' password. The reason is, that most software passkeys will simply not work for local logins,
especially when you log in to a workstation with a window manager, where you cannot even show a QR code easily. USB
Passkeys will still be fine and you will get a prompt like `Passkey PIN` instead of the usual `Password` and you should
leave this option disabled, if all your users that need local access have USB Passkeys.

```admonish note
If you log in to a window manager locally and have a MFA secured account, and Local Password Login is NOT set, you 
usually see `Passkey PIN`. This is however not the case for all window managers. For instance, `sddm` ignores this 
prompt from the PAM module and overwrites it with `Password`. If this is the case and you have a Passkey linked to your
account, you need to enter your PIN at that point and touch your key. Sometimes, they don't forward the prompt properly.
```

## IPs and Aliases

Whatever you provide as values here can be reverse-resolved on hosts via NSS lookups. E.g. when you have a host with the
name `tux`, add an IP `192.168.1.100` and an alias of `host1.example.local`, you can directly do things like
`ping tux` or `curl host1.example.local`, and as long as everything is configured correctly, these requests can resolve
to the IP `192.168.1.100` without any additional DNS servers.

## Notes

The notes field is just for cases when you want to write some notes for all users that can log in to this hosts, like
e.g. mention some special directory or things like that.
