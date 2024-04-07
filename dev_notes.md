# TODO List

## CURRENT WORK

## Stage 1 - essentials

[x] finished

## Stage 2 - features - do before v1.0.0

- BUG with Bitwarden Passkey implementation - app bug and Rauthy cannot do anythign about it? -> investigate:

```
2024-03-21T09:56:04.903993Z ERROR webauthn_rs_core::core: Credential indicates it is backed up, but has not declared valid backup elligibility
2024-03-21T09:56:04.904071Z ERROR rauthy_models::entity::webauthn: Webauthn Auth Finish: CredentialMayNotBeHardwareBound
```

    -> may cause an extraction error in the UI, because no error body is being returned

- BUG: wrong path in the default Dockerfile which points to the DEV TLS certificates
  -> Should work fine when we just get rid of the `/app` path in the Dockerfile -> test!
  -> Fix to `Dockerfile` has been done - check with next nightly release
- make it possible to define a custom header to extract peer IP's (e.g. CDN headers)
- BUG: when webauthn key in `../finish` is not accepted -> HTTP 401 -> no error message -> improve UX in UI
- improve the book with all the new features
- upstream auth providers
- add `at_hash` claim to the ID token
- impl oidc metadata `check_session_iframe` ?
- remove `offline_access` everywhere, because its overhead to manage and not really beneficial with webauthn?
- admin ui: template button for client branding: default-light + default-dark ?
- benchmarks and performance tuning
- maybe get a nicer logo

## Stage 3 - Possible nice to haves

- respect `display=popup` and / or `display=touch` on `/authorize`
- impl experimental `dilithium` alg for token signing to become quantum safe
- 'rauthy-migrate' project to help migrating to rauthy?
- custom event listener template to build own implementation? -> only if NATS will be implemented maybe?
