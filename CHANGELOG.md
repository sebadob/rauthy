# Changelog

## v0.14.2

- Fix for the new LangSelector component on mobile view
- Add default translations (english) for the PasswordPolicy component for cases when it is used
in a non-translated context
[2f8a627](https://github.com/sebadob/rauthy/commit/2f8a6270f97df075d52507c0aa4e6850e5ef8edc)

## v0.14.1

Bugfix release for the Dockerfiles and Pagination in some places

- Split the Dockerfiles into separate files because the `ARG` introduced access rights problems
[25e3918](https://github.com/sebadob/rauthy/commit/25e39189f95d21e84e24f6d21670709a7ee1effd)
- Small bugfix for the pagination component in some places
[317dbad](https://github.com/sebadob/rauthy/commit/317dbadb0391b78aca0cdadc65dff79d9bf74717)

## v0.14.0

This release is mostly about UI / UX improvements and some smaller bugfixes.

- UI: Client side pagination added
[60a499a](https://github.com/sebadob/rauthy/commit/60a499aee0ff071937a03a78759e447f0d477c90)
- Browsers' native language detection
[884f599](https://github.com/sebadob/rauthy/commit/884f5995b71b8faf6ebcacf4331fdda6ccd78d57)
- `sqlx` v0.7 migration
[7c7a380](https://github.com/sebadob/rauthy/commit/7c7a380bdac520df7c29d0d0f4b3b7d3d48be943)
- Docker container image split
[adb3971](https://github.com/sebadob/rauthy/commit/adb397139627a4e3b3f72682457bdfd24b6ce9f4)
- Target database validation before startup
[a68c652](https://github.com/sebadob/rauthy/commit/a68c6527a68b970221f3c48982cabc418aa81d39)
- UI: I18n (english and german currently) for: Index, Login, Logout, Account, User Registration
[99e454e](https://github.com/sebadob/rauthy/commit/99e454ee459ac041c6975df99d481f0145cf7fa4)
[dd2e9ae](https://github.com/sebadob/rauthy/commit/dd2e9ae579444359dc04db76bc1e13d3d0753fe6)
[7b401f6](https://github.com/sebadob/rauthy/commit/7b401f6f0639c053b0e9475121f9ec814f80ef65)
- UI: Custom component to overwrite the browsers' native language
[4208fdb](https://github.com/sebadob/rauthy/commit/4208fdb9904044f9c5b9ddfb5eb2c42c1264481a)
- Some Readme and Docs updates
[e2ebef9](https://github.com/sebadob/rauthy/commit/e2ebef9c72d0f212ac5a4b39241b6d5d486bd8b0)
[d0a71d6](https://github.com/sebadob/rauthy/commit/d0a71d641c3c829b33191cc2c0ff04e5f7d27017)
- The `sub` claim was added to the ID token and will contain the Users UID
[6b0a8b0](https://github.com/sebadob/rauthy/commit/6b0a8b0679484c5515a068911810159a5a39e07d)

## v0.13.3

- UI: small visual bugfixes and improvements in different places
[459bdbd](https://github.com/sebadob/rauthy/commit/459bdbd55ca60bdb0076908131c569a4dc653086)
[57a5600](https://github.com/sebadob/rauthy/commit/57a56000f6ffecf46bd1d202a3bea5a2ded4985f)
- UI: All navigation routes can be reached via their own link now. This means a refresh of
the page does not return to the default anymore
[4999995](https://github.com/sebadob/rauthy/commit/49999950ac1ade24e433e911df84c99256a7f4d0)
[7f0ac0b](https://github.com/sebadob/rauthy/commit/7f0ac0b0d1cf1e2c53881c4a4e010ce43cc2ec11)
[cadaa40](https://github.com/sebadob/rauthy/commit/cadaa407efa9b70b5159e6ec42b5151f8ef79997)
- UI: added an index to the users table to prevent a rendering bug after changes
[e35ffbe](https://github.com/sebadob/rauthy/commit/e35ffbe9cb4e14785c61249d141895c1a7fb4921)

## v0.13.2

- General code and project cleanup
[4531ae9](https://github.com/sebadob/rauthy/commit/4531ae93d453429a54198211b7d122dada452ae4)
[782bb9a](https://github.com/sebadob/rauthy/commit/782bb9adbbb12f77232b1820e7dd05265c0fdf00)
[0c5ad02](https://github.com/sebadob/rauthy/commit/0c5ad02e369935b01aac46988a2242c859737e24)
[e453142](https://github.com/sebadob/rauthy/commit/e45314269234612a3eec046073e988e260a7ca31)
[85fbafe](https://github.com/sebadob/rauthy/commit/85fbafe5ef6b8f124af6af1508b6e2bab067a8ff)
- Created a `justfile` for easier development handling
[4aa5b99](https://github.com/sebadob/rauthy/commit/4aa5b9993897e43dfc765eb2849172bc087ea34c)
[1489efe](https://github.com/sebadob/rauthy/commit/1489efe139c0a0c79169f47ba4fc964cdc6b6e3e)
- UI: fixed some visual bugs and improved the rendering with larger default browser fonts
[45334fd](https://github.com/sebadob/rauthy/commit/45334fd65049f2950dae3a2bc28c5667c275aa1d)

## v0.13.1

This is just a small bugfix release.

- UI Bugfix: Client flow updates were not applied via UI
[6fe8fbc](https://github.com/sebadob/rauthy/commit/6fe8fbc1440498ea126a5aee5bed9dfe34e367d4)

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
