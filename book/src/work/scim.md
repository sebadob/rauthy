# SCIM

## tl;dr

Rauthy requires:

- the following SCIM v2 endpoints under `{scim_base_uri}`:
    - `GET /Users?filter=...`
    - `POST /Users`
    - `GET /Users/{id}`
    - `PUT /Users/{id}`
    - `DELETE /Users/{id}` with `scim.sync_delete_users = true`
    - `GET /Groups?filter=...`
    - `POST /Groups`
    - `GET /Groups/{id}`
    - `PATCH /Groups/{id}` with `PatchOp`
    - `DELETE /Groups/{id}` with `scim.sync_delete_groups = true`
- the client to handle `externalId` properly
- `fitler=externalId` on both `{base_url}/Users` and `{base_url}/Groups`
- `filter=userName` on `{base_url}/Users`
- `filter=displayName` on `{base_url}/Groups`
- `userName` to be set properly to the users `email` to match against the DB
- `display` to be set properly for `ScimGroup`s to match against the DB
- `PatchOp`s to be working properly on `PATCH {base_url}/Groups/{id}`
    - `replace` will be used for updating the group itself
    - `add` / `remove` ops will be used to modify membership

## SCIM Requests

This is just a short section about the SCIM flow Rauthy uses. If you only care about an implementation with Rauthy, you
don't need to implement the full SCIM RFC.

As an example, we will go through what happens, when you add SCIM v2 to a client. This will trigger a complete sync
which involves all steps. SCIM syncs are also triggered for instance when updating a single user or a group name, but
these are sub sets of what will happen during a full client sync. So, let's assume you add a SCIM v2 base URL with
enabled Groups sync to a client. It will then do the following requests to the client:

1. `GET {base_url}/Users` with first `filter=externalId` and if nothing was found, with `filter=userName`
    1. If the User exists and should not be synced because of a group prefix filter, `DELETE {base_url}/Users/{id}`
       (`scim.sync_delete_users`)
    2. If the user does not exist on the client, `POST {base_url}/Users` and create the new user
    3. If it was found by the `GET`, update all values and make sure `externalId` is set properly, if it has been found
       by `userName`
2. Check if Rauthy should sync groups for this client, and skip groups if not
    1. Get all remote groups via  `GET {base_url}/Groups` with optional `filter=externalId` or `filter=displayName`
    2. Compare the `user.groups` from remote against the assignments on Rauthys side
    3. Only delete groups on remote if they have a matching `externalId` and should be deleted according to Rauthys
       config (`scim.sync_delete_groups`)
    4. If groups don't exist on remote while they should, `POST {base_url}/Groups` to create them
    5. If a match was found by `externalId` with an outdated name, `PATCH {base_url}/Groups{id}` to update it properly
3. When Users and Groups exist like they should, make sure Groups assignments are correct. These are done via `PatchOp`s
   to `PATCH {base_url}/Groups/{id}`

These steps cover everything that could happen, apart from a group name update. If this happens, Rauthy will do a
`PATCH {base_url}/Groups{id}` with the `replace` setting to replace the groups name / `externalId`.

```admonish info
If `scim.sync_delete_groups` / `scim.sync_delete_users` is disabled, Rauthy will not delete the resources on remote, it
will only remove the `externalId` value to indicate, that this resource is not managed by Rauthy anymore. This is 
important for instance in some scenarios, where you may have important data linked to a user, that you want to keep, 
even if the user does not exist anymore.
```
