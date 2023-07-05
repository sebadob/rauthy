# Changelog

## v0.13.0

- Improved container security: Rauthy is based off a Scratch container image by default now. This improved the security
quite a lot, since you cannot even get a shell into the container anymore, and it reduced the image size by another
~4MB.  
This makes it difficult however if you need to debug something, for instance when you use a SQLite deployment. For this reason, you can append `-debug` to a tag
and you will get an Alpine based version just like before.
[1a7e79d](https://github.com/sebadob/rauthy/commit/1a7e79dc96d27d8d180d1e4394644c8851cbdf70)
- More stable HA deployment: In some specific K8s HA deployments, the default HTTP2 keep-alive's from
[redhac](https://github.com/sebadob/redhac) were not good enough and we got broken pipes in some environments which
caused the leader to change often. This has been fixed in [redhac-0.6.0](https://github.com/sebadob/redhac/releases/tag/v0.6.0)
too, which at the same time makes Rauthy HA really stable now.
- The client branding section in the UI has better responsiveness for smaller screens
[dfaa23a](https://github.com/sebadob/rauthy/commit/dfaa23a30ccf77da2b29654c7dd3b41a4ca78168)
- For a HA deployment, cache modifications are now using proper HA cache functions. These default back to the single
instance functions in non-HA mode since [redhac-0.6.0](https://github.com/sebadob/redhac/releases/tag/v0.6.0)
[7dae043](https://github.com/sebadob/rauthy/commit/7dae043d7b42724adad85b5ed54f1dcd9d143d27)
- All static UI files are now precompressed with gzip and brotli to use even fewer resources
[10ad51a](https://github.com/sebadob/rauthy/commit/10ad51a296c5a7596b34f9c726fe87480b6ec42c)
- CSP script-src unsafe-inline was removed in favor of custom nonce's
[7de918d](https://github.com/sebadob/rauthy/commit/7de918d601007d2807701a096d6403bf2b3274c9)
- UI migrated to Svelte 4
[21f73ab](https://github.com/sebadob/rauthy/commit/21f73abfb0332be3fc391b9d108655a0cd5a3cec)

## v0.12.0

Rauthy goes open source
