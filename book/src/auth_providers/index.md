# Upstream Authentication Providers

This section is about upstream authentication providers, for instance things like **Sign in with GitHub**.

This feature can be considered "in beta state" and the only setup guide is for GitHub for now.  
Additional ones may come, when people are using other providers and give feedback or actually contribute to these docs.

## Direct Login with `idp_hint`

Rauthy supports the `idp_hint` query parameter in the OIDC authorization request.
When this parameter is provided with the ID of an upstream authentication provider, Rauthy will automatically redirect the user to that provider's login page, bypassing the Rauthy login selection.

For example, if you have a GitHub provider with the ID `github` configured:
`GET /auth/v1/oidc/authorize?client_id=...&idp_hint=github`

This is useful if you want to provide a "Login with GitHub" button directly in your application that skips the intermediate Rauthy login selection page.
The `idp_hint` value must match the `ID` of the provider as configured in the Admin UI.

## Hide the Login Button

A provider can be configured with **Hide Login Button**. When it is active, the button of that
provider is not rendered on the login page and the registration page, which is how an internal
provider can be kept away from normal users.

Everything else about the provider behaves exactly like before. It can still be used to log in and
to issue tokens, and a targeted login via `idp_hint` keeps working:

`GET /auth/v1/oidc/authorize?client_id=...&idp_hint=internalsso`

This is the intended way to use such a provider: link that URL, with the `idp_hint` parameter
pointing at the provider's `ID`, from a dedicated login button, for instance on an internal
landing page. The provider config in the Admin UI shows a note about this while the checkbox is
active.

## Claim Mapping to Roles and Groups

An upstream provider can map values from the ID token of a successful federated login to **existing
local roles and groups**. Each rule has a claim `path`, a claim `value`, a `typ` (`role` or `group`)
and the `target` name:

```json
{ "claim_path": "$.groups[*]", "claim_value": "^engineering", "typ": "group", "target": "engineering" }
```

With an ID token like

```json
{ "groups": ["engineering", "on-call"], "exp": 1700000000 }
```

that rule matches the `engineering` node and grants the local `engineering` group.

Rules are evaluated on every federated login: a target is added when any of its rules matches, and
removed when none of them matches. **Only the targets a provider maps are ever removed.** A target a
provider maps belongs to that provider, so a login whose claims do not match revokes it even when an
admin assigned that same role or group to the user by hand. Names outside the provider's mapping set
are never touched.

### The `path` grammar

The `path` is a reduced JSONPath (RFC 9535) subset. What is supported:

| Syntax              | Meaning                                                          |
| ------------------- | ---------------------------------------------------------------- |
| `$`                 | the root of the ID token                                           |
| `.name`             | a child member                                                     |
| `.*` / `[*]`        | all members of an object or elements of an array                   |
| `['name']`          | a child member in quotes — needed for keys with special characters |
| `[0]` / `[-1]`      | an array index, negative counts from the end                       |
| `..name` / `..*`    | descendant: that key, or every node, at any depth                  |
| `..['x-groups']`    | the same in quotes                                                 |

Not supported: filter expressions (`[?@.a=='x']`), slices (`[1:3]`), unions (`[0,1]`), and function
expressions. A path outside this grammar is rejected when the provider config is saved. Two details
of the reduced implementation: a trailing `*` on a leaf selects the leaf itself
(`$.foo.bar.*` on a string), and a shorthand name may start with an uppercase letter
(`$.UserGroups`).

The `admin` and `mfa` claim fields use the same grammar. Keys with characters outside
`a-z A-Z 0-9 _ -` need the quoted form: `$..['x-groups']`.

### The `value` sigils

| Value        | Match                                                       |
| ------------ | ----------------------------------------------------------- |
| `admin`      | exactly `admin`                                               |
| `^admin`     | every value that starts with `admin`                          |
| `~^admin$`   | the regex `^admin$`                                           |
| `^^admin`    | the literal text `^admin`                                     |

A path can select multiple nodes and a rule matches if **any** selected node matches the value. A
non-string node is compared by its JSON form, so `"true"` matches a boolean `true` claim and
`"23"` matches the number `23`.

A `~` value is compiled as a regex once per provider, when the config is saved, and then evaluated
against every selected claim node of every federated login. It is linear time, but it is the most
expensive matcher type: keep patterns anchored and as simple as possible. An invalid regex fails the
config save and names the rule.

### Config limits

* A target must already exist as a role or a group. Rauthy never creates a role or a group on a
  login path, and a missing target is rejected at config save, naming the name.
* A target name must not contain a `,`, the separator of a user's role and group lists. A name
  carrying it would come back as several members, so it is rejected at config save.
* `rauthy_admin` cannot be a rule target. The full admin role is granted by the provider's own
  `admin_claim_path` / `admin_claim_value` mapping only, never by a generic rule.
* `rauthy_admin:<group>` is a valid target and it must be mapped as a `role`: it is the delegated
  group admin, the same role name an admin assigns by hand. It needs the group behind it, and
  `rauthy_admin:*` covers every group. A rule revokes it like any other target.
* A provider supports at most **32** rules.
* A `path` and its `value` must always be set together, for the `admin` and `mfa` claims as well.
* The ATProto provider completes its login from the ATOPIAN session and carries no upstream ID token
  payload, so claim mappings, and the `admin` and `mfa` claim fields, do not apply to it.
