# UNIX Domain Sockets

Rauthy supports listening on a UNIX domain socket. To enable this feature, you will need to specify the following
configurations.

```toml
[server]
# The server address to listen on. Can bind to a specific IP.
# When LISTEN_SCHEME is set to use UNIX sockets, this specifies
# the path to listen on.
#
# default: '0.0.0.0'
# overwritten by: LISTEN_ADDRESS
listen_address = "0.0.0.0"

# The scheme to use locally, valid values:
# http | https | http_https | unix_http | unix_https
# For more details about the UNIX domain socket, check out its
# documentation page.
#
# default: http_https
# overwritten by: LISTEN_SCHEME
scheme = "unix_https"

# When rauthy is running behind a reverse proxy, set to true
#
# default: false
# overwritten by: PROXY_MODE
proxy_mode = true
```

```admonish caution
Note that `unix_https` does not make a socket with TLS. When `unix_https` is used, the socket will still be plain HTTP!

`unix_https` specifies the scheme to be prepended to `pub_url`, so whether to use`unix_http` and `unix_https` depends on the scheme you are using on the reverse-proxy.
```

UNIX domain sockets should be used with a reverse proxy. Without such a proxy, Rauthy could not know the IP address of
visitors, and it will see all visitors as `192.0.0.8`, the IPv4 dummy address. Rate-limits and IP blacklists will apply
on
all visitors at the same time.

After setting up a reverse proxy, please also remember to set your proxy to send peer IP in headers (for example,
`X-Forwarded-For`), and set `PROXY_MODE` to let Rauthy read the IP. Once everything is set, check the `/auth/v1/whoami`
endpoint to see if your IP address is shown correctly.

It is also recommended to use POSIX ACLs to limit access to Rauthy socket to your reverse proxy, so other UNIX users
won't be able to connect to the socket directly and bypass the reverse proxy.
