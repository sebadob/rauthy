# PAM + NSS Modules

This is a documentation for the separately managed project [rauthy-pam-nss](https://github.com/sebadob/rauthy-pam-nss).
It's a separate project to avoid licensing issues because of some dependencies the PAM and NSS bindings rely on.

These modules are basically the "client part" for all the PAM configuration you did on Rauthys side. You need to install
3 things:

- NSS module
- NSS Proxy
- PAM module

The NSS module + proxy are there to dynamically resolve users, groups and hosts via Rauthy without the need to add them
to any of the local files like `/etc/passwd` or `/etc/group`. The NSS module will be called from all different sorts of
applications, that need to resolve some names. The NSS module exposes the `C` bindings and then execute the logic in
Rust. This module is pretty simple, as it only connects to a local Unix Domain Socket, where the NSS Proxy will be
listening. This proxy is the one doing the actual requests to Rauthy, as well as cache results for faster subsequent
lookups.

The reason why there is a proxy in between is not only to be able to cache results, but also to be more efficient and
more importantly, to only give the NSS proxy access to the actual host ID and secret, which are needed to do requests
against Rauthys API. These should have the least possible amount of exposure, because with these credentials, you could
technically find out which usernames exist on Rauthy and which do not. This is not a security issue right away, but not
leaking anything is always preferred.

The main configuration file for NSS lookups is `/etc/nsswitch.conf`, but no worries, it's not only pretty simple to
understand, but will also be automatically managed by the install script. The only important part about it is, if you
want to be able to use the `local` PAM groups feature, that you add `[SUCCESS=merge]` in between entries for `group`
lookups.

You can test very easily if NSS lookups are working with e.g. `getent groups` or `getent users`, and you should see
the Rauthy-managed resources included in the output. If at the beginning or at the end depends on your `nsswitch.conf`,
and you of course can only see users if there are any configured on Rauthy. Only if NSS lookups are working fine, you
should install the PAM module.

The PAM module exposes the `C` bindings for, surprise, PAM. Configuring PAM is its own, very big topic and I will not
go into detail here. There is lots of documentation and guides in the internet. For simple setups, like if you did not
do any customizations to your hosts anyway, the provided default templates should work out of the box (hopefully). If
you have done customizations, you probably know about how it's done anyway and maybe don't even want to use the provided
templates. That's up to you of course. The same for NSS, as long as you add Rauthys modules to your setup correctly, it
should work.

For standard scenarios, you will get an install script. It currently should upport RHEL and Debian based distros. Other
ones (suse, arch, gentoo) may need contributions to include them in the script.

## Install TL;DR

The PAM project is in a very early phase and there are no distro-packaged versions available. Here is the gist how to
install:

```bash
curl -LO https://raw.githubusercontent.com/sebadob/rauthy-pam-nss/refs/heads/main/install/rauthy-pam-nss-install.tar.gz \
  && tar -xzf rauthy-pam-nss-install.tar.gz \
  && cd rauthy-pam-nss-install
```

Then, since you should never blindly execute a random bash script from the internet, especially with `sudo`, inspect
`install.sh` and afterward install config files, NSS Proxy and (depending on availability) SELinux policy:

```bash
sudo ./install.sh nss
```

Then check via e.g. `getent hosts` or `getent groups` that you get data from Rauthy. However, the script does it as well
and you should see an error message about exceeded retries it you e.g. have given invalid credentials. When NSS lookups
are working fine, proceed with the PAM module installation:

```bash
sudo ./install.sh pam
```

If your OS is managed by `authselect`, you need to activate the new custom profile afterward, just like mentioned in the
output. On other OSes like Debian, the script will create backups of config files and then copy the Rauthy configs in
place directly.

**CAUTION:** Make sure to test all the logins and things that should work at this point BEFORE logging out. Keep a
backup session open, just in case something broke. Incorrectly configured PAM modules can lock you out of your machine!

```admonish danger
Before you install anything, make sure that you have at least a backup session / terminal somewhere in the background
that you don't use for installation / testing. If you mess up your PAM setup, you can easily lock yourself out of your 
machine and it could be very hard, or sometimes impossible, to recover it. Keep the backup session open and don't do 
anything with it, until you verified after the installation that everything is working as expected. If this should not 
be the case, you should revert your PAM config. The install script will copy the old files and appends `.bak-non-rauthy` 
to them.

If you are not experienced with NSS / PAM yet, you should not try it on any important or production machine. The best
thing would be to spin up an empty VM for testing, or at least create a new snapshot before you start. The default PAM 
configs are safe, but if you do anything custom, it is very easy to make your whole system insecure if done incorrectly.
```

```admonish hint
The install script detects `authselect`-managed environments automatically and creates a `custom/rauthy` profile in this
case, which you need to activate manually. However, it does NOT work with other management systems yet and if it does
not find `authselect`, it will create backup copies of `/etc/nsswitch.conf` and the ones in `/etc/pam.d/` before copying
its own files in place. If you manage PAM via other systems, feel welcome to provide a PR to include it in the script.
```

## Where to find everything

The install script will take care of everything to get it up and running for standard scenarios. However, if you need to
debug, do custom configuration, or anything else, here is some more information:

- The config file can be found here: `/etc/rauthy/rauthy-pam-nss.toml`
- `/etc/skel_rauthy` contains the template for home dir creation.
- The PAM and NSS modules will go "somewhere" into `/lib/`, depending on your distro.
- The NSS socket, which should be carefully protected, so a normal user cannot spoof it, can be found under
  `/run/rauthy/rauthy_proxy.sock`. This is wrapped inside a sub-dir on purpose for 2 reasons: Additional protection via
  access rights, because it needs to be world-accessible for a lot of services, and to be able to mount the dir into
  a container, just in case someone maybe wants to run the NSS proxy as a quadlet. You cannot mount a socket file into
  a container directly (as listener at least) and need to mount the parent directory instead.
- `/var/lib/pam_rauthy/` contains the optional session start and close scripts (enabled via the config above) and all
  the PAM tokens for users that are created during login.

```admonish caution
It is VERY important that you never mess up the access rights for all of these files and dirs. Most importantly for the
UDS so it can never be spoofed. Everything should run as `root` for the highest level of protection. In case of the
session open and close scripts, their access rights will be checked by the PAM module before each execution and if they
don't match `0700` or are not owned by `root`, they will not be executed.
```

```admonish caution
If you change any of the paths like e.g. the `data_dir` and you have SELinux running, you must make sure that the
correct types are added via the file context.
```

```admonish note
All of these locations and executables have dedicated SELinux types and a policy to provide additional protection. This
will of course only work on systems where SELinux is available. An AppArmor equivalent does NOT exist yet, but the 
generic Linux access rules provide proper protection, at least as long a not another `root` application is malicious.
```

## SELinux

An SELinux policy with quite a lot of apps and services is already included and will be installed automatically, if
SELinux exists on your system. However, I am pretty sure that depending on the type of apps you have installed, you
might get warnings for apps about access the `rauthy_var_run_t` with `write` or `connectto`. This is generally fine to
allow for anything, as long as you don't allow anything else. `write` and `connectto` are necessary to do NSS lookups
via the NSS module.

If you stumble about warnings and missing types that should be added to the policy, please open an issue or directly a
PR about it. I can of course only validate for types that I come across on a daily basis. Apart from writing and
connecting to the UDS, you should not see any warnings (hopefully), but if you do, again, please open an issue / PR.

## AppArmor

I am working with RHEL based distros and SELinux exclusively and therefore don't use AppArmor and I have no experience
with it. No rules have been added to the repo or install script. If you use AppArmor, please feel free to provide a PR.
