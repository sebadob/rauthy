# Changelog

## v0.16.0

### Breaking

This version does modify the database and is therefore not backwards compatible with any previous version.
If you need to downgrade vom v0.15 and above, you will only be able to do this via by applying a DB Backup.

### New Features

#### User Expiry

It is now possible to limit the lifetime of a user.  
You can set an optional expiry date and time for each user. This enables temporary access, for instance in
a support case where an external person needs access for a limited time.

Once a user has expired, a few things will happen:
- The user will not be able to log in anymore.
- As soon as the scheduler notices the expiry, it will automatically invalidate all possibly existing
sessions and refresh tokens for this user. How often the scheduler will run can be configured with the
`SCHED_USER_EXP_MINS` variable. The default is 'every 60 minutes' to have a good balance between security
and resource usage. However, if you want this to be very strict, you can adjust this down to something like
'5 minutes' for instance.
- If configured, expired users can be cleaned up automatically after the configured time.
By default, expired users will not be cleaned up automatically. You can enable this feature with the
´SCHED_USER_EXP_DELETE_MINS` variable.

#### `WEBAUTHN_NO_PASSWORD_EXPIRY`

With this new config variable, you can define, if users with at least one valid registered passkey will
have expiring passwords (depending on the current password policy), or not.  
By default, these users do not need to renew their passwords like it is defined in the password policy.

#### Peer IP's for sessions -> `SESSION_VALIDATE_IP`

When a new session is being created, the peer / remote IP will be extracted and saved with the session
information. This peer IP can be checked with each access and the session can be rejected, if this IP
has changed, which will force the user to do a new login.

This will of course happen if a user is "on the road" and uses different wireless networks on the way,
but it prevents a session hijack and usage from another machine, if an attacker has full access to the
victims machine and even can steal the encrypted session cookie and(!) the csrf token saved inside the
local storage. This is very unlikely, since the attacker would need to have full access to the machine
anyway already, but it is just another security mechanism.

If this IP should be validated each time can be configured with the new `SESSION_VALIDATE_IP` variable.
By default, peer IP's will be validated and a different IP for an existing session will be rejected.

#### Prometheus metrics exporter

Rauthy starts up a second HTTP Server for prometheus metrics endpoint and (optional) SwaggerUI.
By default, the SwaggerUI from the `Docs` link in the Admin UI will not work anymore, unless you
specify the SwaggerUI via config to be publicly available. This just reduces the possible attack
surface by default.

New config variables are:

```
# To enable or disable the additional HTTP server to expose the /metrics endpoint
# default: true
#METRICS_ENABLE=true

# The IP address to listen on for the /metrics endpoint.
# You do not want to expose your metrics on a publicly reachable endpoint!
# default: 0.0.0.0
#METRICS_ADDR=0.0.0.0

# The post to listen on for the /metrics endpoint.
# You do not want to expose your metrics on a publicly reachable endpoint!
# default: 9090
#METRICS_PORT=9090

# If the Swagger UI should be served together with the /metrics route on the internal server.
# It it then reachable via:
# http://METRICS_ADDR:METRICS_PORT/docs/v1/swagger-ui/
# (default: true)
#SWAGGER_UI_INTERNAL=true

# If the Swagger UI should be served externally as well. This makes the link in the Admin UI work.
#
# CAUTION: The Swagger UI is open and does not require any login to be seen!
# Rauthy is open source, which means anyone could just download it and see on their own,
# but it may be a security concern to just expose less information.
# (default: false)
#SWAGGER_UI_EXTERNAL=false
```

#### User Verification status for Passkeys

For all registered passkeys, the User Verification (UV) state is now being saved and optionally checked.
You can see the status for each device with the new fingerprint icon behind its name in the UI.

New config variable:

```
# This feature can be set to 'true' to force User verification during the Webauthn ceremony.
# UV will be true, if the user does not only need to verify its presence by touching the key,
# but by also providing proof that he knows (or is) some secret via a PIN or biometric key for
# instance. With UV, we have a true MFA scenario where UV == false (user presence only) would
# be a 2FA scenario (with password).
#
# Be careful with this option, since Android and some special combinations of OS + browser to
# not support UV yet.
# (default: false)
#WEBAUTHN_FORCE_UV=true
```

#### Passkey Only Accounts

This is the biggest new feature for this release. It allows user accounts to be "passkey only".

A passkey only account does not have a password. It works only with registered passkeys with
forced additional User Verification (UV).

Take a look at the updated documentation for further information:  
[Passkey Only Accounts]https://sebadob.github.io/rauthy/config/fido.html#passkey-only-accounts


#### New E-Mail verification flow

If an already existing user decides to change the E-Mail linked to the account, a new verification
flow will be started:

- A user changes the E-Mail in the Account view.
- The E-Mail will not be updated immediately, but:
    - A verification mail will be sent to the new address with an expiring magic link.
    - After the user clicked the link in the mail, the new address will be verified.
- Once a user verifies the new E-Mail:
    - The address will finally be updated in the users profile.
    - Information E-Mails about the change will be sent to the old and the new address

#### `EMAIL_SUB_PREFIX` config variable

This new variable allows you to add an E-Mail subject prefix to each mail that Rauthy sends out.
This makes it easier for external users to identify the email, what it is about and what it is
doing, in case the name 'Rauthy' does not mean anything to them.

```
# Will be used as the prefix for the E-Mail subject for each E-Mail that will be sent out to a client.
# This can be used to further customize your deployment.
# default: "Rauthy IAM"
EMAIL_SUB_PREFIX="Rauthy IAM"
```

#### New nicely looking error page template

In a few scenarios, for instance wrong client information for the authorization code flow or
a non-existing or expired magic link, Rauthy now does not return the generic JSON error response,
but actually a translated HTML page which informs the user in a nicer looking way about the problem.
This provides a way better user experience especially for all Magic Link related requests.

#### Rauhty DB Version check

This is an additional internal check which compares the version of the DB during startup and 
the App version of Rauthy itself. This makes it possible to have way more stable and secure
migrations between versions in the future and helps prevent user error during upgrades.

### Changes

- legacy MFA app pieces and DB columns have been cleaned up
[dce148a](https://github.com/sebadob/rauthy/commit/dce148a2d665836ec733b8d0904e9d3892c40b2f)
[423db7a](https://github.com/sebadob/rauthy/commit/423db7a66c412ce9d04ec317cd0a237e278653bb)
- user expiry feature
[bab6bfc](https://github.com/sebadob/rauthy/commit/bab6bfcecd62e44d23b9527009904c12a04e567a)
[e63d1ce](https://github.com/sebadob/rauthy/commit/e63d1ce746dd27ed78acebb688d44fbf7a0b50dc)
[566fff1](https://github.com/sebadob/rauthy/commit/566fff12e0a3e8ee2b867ecfd590cbe0ef24378d)
- `WEBAUTHN_NO_PASSWORD_EXPIRY` config variable
[7e16b6e](https://github.com/sebadob/rauthy/commit/7e16b6e7c5e5038c653f55807e6aeed35e7536cb)
- `SESSION_VALIDATE_IP` config variable
[828bcd2](https://github.com/sebadob/rauthy/commit/828bcd285b844d846f23e22e488a372dba0c59ff)
[26924ff](https://github.com/sebadob/rauthy/commit/26924ffbc2054abc537bba4ef6cd98b50ad594c3)
- `/metrics` endpoint
[085d412](https://github.com/sebadob/rauthy/commit/085d412e51c3f77449c39449206d296b710c1d9b)
- `WEBAUTHN_FORCE_UV` config variable
- Passkey Only Accounts
[6c2406c](https://github.com/sebadob/rauthy/commit/6c2406c4f261ec0e784df6ee50efd837a86160f7)
- New E-Mail verification flow
[260169d](https://github.com/sebadob/rauthy/commit/260169d289731c44289cd6fce1c642c6d26786ae)
- New nicely looking error page template
[0e476ab](https://github.com/sebadob/rauthy/commit/0e476ab0ec8728450d83a315240dd870ac34cddf)
- Rust v1.73 update
[43f5b61](https://github.com/sebadob/rauthy/commit/43f5b61c86d563cefaf2ea2ee832e3317bcfcfb7)
- `EMAIL_SUB_PREFIX` config variable
[af85839](https://github.com/sebadob/rauthy/commit/af858398d61b2bea347dabcd4b7adf6a8c60f86d)
- Rauthy DB Version check
[d2d9271](https://github.com/sebadob/rauthy/commit/d2d92716b93a9ed1e6dc02ce7906f0741044d464)
- Cleanup schedulers in HA_MODE run on leader only
[4881dd8](https://github.com/sebadob/rauthy/commit/4881dd83a9b7d298fc998c635face7feb55549ff)
- Updated documentation and book
[5bbaae9](https://github.com/sebadob/rauthy/commit/5bbaae9c24ce49dec0c10908d351257519279c9a)
- Updated dependencies
[0f11923](https://github.com/sebadob/rauthy/commit/0f1192354c805832d8f252d3204f624deb9e6561)

## v0.15.0

### Breaking

This version does modify the database and is therefore not backwards compatible with any previous version.
If you need to downgrade vom v0.15 and above, you will only be able to do this via by applying a DB Backup.

### Changes

This release is all about new Passkey Features.

- A user is not limited to just 2 keys anymore
- During registration, you can (and must) provide a name for the passkey, which helps you identify and distinguish
your keys, when you register multiple ones.
- The `exclude_credentials` feature is now properly used and working. This makes sure, that you cannot register the
same Passkey multiple times.
- The Passkeys / MFA section in the Admin UI has been split from the User Password section to be more convenient to use

Commits:  
- New Passkey Features  
[d317d90](https://github.com/sebadob/rauthy/commit/d317d902f66613b743955117dbede3d52efa74b6)
[cd5d086](https://github.com/sebadob/rauthy/commit/cd5d086e0b38d8d906d3ce698ed9860bec3d52ab)
[61464bf](https://github.com/sebadob/rauthy/commit/61464bfcc69921fae0c826fe8c3bac73e290a242)
[e70c5a7](https://github.com/sebadob/rauthy/commit/e70c5a7bbf8f40f90a9d8d3d6570736e5322a0c5)
[bc75610](https://github.com/sebadob/rauthy/commit/bc75610d7aa4cfc35abf1e0ee67af3b4dc2eea82)
[49e9630](https://github.com/sebadob/rauthy/commit/49e9630bb3ffc25be1523492ef47d2fc66dc041c)
- New config option `WEBAUTHN_FORCE_UV` to optionally reject Passkeys without user verification
[c35ecc0](https://github.com/sebadob/rauthy/commit/c35ecc090b222196cf1ac095aa2b340edbeec0f5)
- Better `/token` endpoint debugging capabilities
[87f7969](https://github.com/sebadob/rauthy/commit/87f79695b86f9b7199dbad244c61e2eb1a0c6bcb)

## v0.14.5

This is the last v0.14 release.  
The next v0.15 will be an "in-between-release" which will do some migration preparations for Webauthn
/ FIDO 2 updates and features coming in the near future.

- Removed duplicate `sub` claims from JWT ID Tokens
[a35db33](https://github.com/sebadob/rauthy/commit/a35db330ff7c6ee680a7d834f08a3db077e08073)
- Small UI improvements:
    - Show loading indicator when doing a password change
    - The Loading animation was changes from JS to a CSS animation
[abd0a06](https://github.com/sebadob/rauthy/commit/abd0a06280de4fedef9028f142b6e844bf132d80)
- Upgrades to actix-web 4.4 + rustls 0.21 (and all other minor upgrades)
[070a453](https://github.com/sebadob/rauthy/commit/070a453aaa584ff8d024284de91477626fe5ea6c)

## v0.14.4

This release mostly finishes the translation / i18n part of rauthy for now and adds some other
smaller improvements.  
Container Images will be published with ghcr.io as well from now on. Since I am on the free plan
here, storage is limited and too old versions will be deleted at some point in the future.
However, I will keep pushing all of them to docker hub as well, where you then should be able
to find older versions too. ghcr.io is just preferred, because it is not so hardly rate limited
than the docker hub free tier is.

- Added translations for E-Mails
[11544ac](https://github.com/sebadob/rauthy/commit/11544ac46fcddeb53a511b8ad702b1ad2868148e)
- Made all UI parts work on mobile (except for the Admin UI itself)
[a4f31f2](https://github.com/sebadob/rauthy/commit/a4f31f22396b5767c5a2c20e0253171910296447)
[4ee3540](https://github.com/sebadob/rauthy/commit/4ee3540d9b32e6f0e2fbd5cf5eabcd7736179da8)
- Images will be published on Github Container Registry as well from now on
[cc15ea9](https://github.com/sebadob/rauthy/commit/cc15ea9cb5d7c0bd6d3cad1fea908e656f488e50)
- All dependencies have been updates in various places. This just keeps everything up to date
and fixed some potential security issues in third party libraries

## v0.14.3

- UI: UX Improvements to Webauthn Login when the user lets the request time out
[7683133](https://github.com/sebadob/rauthy/commit/76831338e88a36cc3166039493318c38cc7a1e49)
- UI: i18n for password reset page
[27e620e](https://github.com/sebadob/rauthy/commit/27e620edb34d803259a000d837513920905d2332)
- Keep track of users chosen language in the database
[7517693](https://github.com/sebadob/rauthy/commit/7517693ddec4f5e0215fcced9dde6322e665ff10)
- Make user language editable in the admin ui
[77886a9](https://github.com/sebadob/rauthy/commit/77886a958655b721e96c092df739627d0d5d9172)
[1061fc2](https://github.com/sebadob/rauthy/commit/1061fc212bb684b153ae99f1439ea312091e32cd)
- Update the users language in different places:
    - Language switch in the Account page
    - Fetch users chosen language from User Registration
    - Selector from Registration in Admin UI
[5ade849](https://github.com/sebadob/rauthy/commit/5ade849dd5a0b139c73a61b8fefde03eff3036bf)

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
