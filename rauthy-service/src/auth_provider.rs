// This module is basically a more generic version of the `rauthy-client`.
// It only implements the very basics that are needed to use upstream auth providers for logging
// in to Rauthy. It less strict than `rauthy-client` and works with not only OIDC providers, but
// also oauth2 to have the most possible compatibility.
//
// This module is a big TODO right now. The below notes are just for the development process.
//
// The main idea of the login flow is the following:
// - Show additional buttons for each configured provider
// - When a user chooses an external one, cache all request params for the `/authorization` with
// a new uid for later use
// - Save a new, encrypted cookie on the client with the uid for the initial params
// - Redirect the client to the external provider
// - After the external stuff has been done, the user will land on the callback URL, where Rauthy
// extracts the encrypted request params uid and does a cache lookup for the initial data
// - Get the tokens from the provider and extract all necessary claims
// - If everything was fine, continue with the login flow as is the user would have used Rauthy
// directly via the `/authorization` page and delete the cookie data.
//
// Notes:
// - [x] Providers should be configurable over the Admin UI:
//     - [x] Either, enter the ISS URL and Rauthy will try to do an auto-lookup and config
//     - [x] Or, provide an options list in the UI with templated values for the most known out there
//     - [x] Update and Delete for providers
// - [x] The amount of configurable providers should not be limited
// - [x] !!! Do not link a user to multiple providers to reduce attack surface. This "feature" would
// not really be a quality of life improvement, but actually just support the lazyness of users.
// If they want to use an external provider, they should simply choose the correct one for their
// account.
// - [x] Providers will be stored in a new table with all the necessary information, as well as some
// UX things like a logo and nice to read name.
// - [x] The providers must be connected to a user account on creation with something like a
// `federation_id`, which will be a DB FK to a new table with additional data for this user from
// the provider (provider id, external uid, ...).
// - [x] When the migrations on the `users` table will be done, add another (not related to this feature)
// column named `last_login` (Needed feature - makes sense to do just one migration and not multiple).
// - We don't really need to care about JWKs and validation of external providers' tokens, because
// we will only use them once after Rauthy has changed them for the `code` on the callback. At this
// point, there is no need to validate the tokens, since they just came from the external provider
// over an encrypted connection. This simplifies this module and means we don't need caching.
// - [x] Implement all necessary additional API endpoints in `rauthy-handlers`
// - [x] Create a nice looking callback page for the user with maybe additional information, because
// there are a lot of things that can go wrong during the flow with external providers. If everything
// was good, we could just show a loading spinner for now.
// - [x] It should be possible to add custom Root CA's for each providers independently.
// - [x] It should be possible for internal networks and testing that insecure TLS connections
// are opt-in for each provider independently.
// - [x] Think about a solution for the case, that a user is already registered with Rauthy, does
// an external login flow and we find out, that the exact same E-Mail is used:
//    - Update this user with the external providers data or reject it?
//    - We cannot know, which values should have the priority in case of conflicts.
//    - Simply always federate it and only use values that Rauthy does not have?
//    - Should it be possible to "un-federate" an account on Rauthy and basically remove the link?
//    - At the same time, should it be possible to federate it on purpose via account page?
// - [x] Save upstream provider secrets only encrypted
// - [x] include provider secrets in secrets migration task
// - [x] custom validation check for providers to either be confidential or support pkce
//
// - [x] Show additional information in each users account page, if it is a federated account
// - [x] Fix the new UI bug in the account page if the account does not have any password + passkey
// -> page is not rendering currently
// - [x] Make it possible to create a Password / Passkey for federated accounts, so this flow may
// be used for the initial setup only and use Rauthy's advanced security mechanisms afterwards.

// TODO:
// - [ ] i18n for the auth provider callback page, especially the different error cases
// - [ ] Unlink an account from federation
// - [ ] Define mandatory mechanisms and features, that external providers must support to make
// everything working securely and write these down in the Rauthy book (new section)
// - [ ] Make it possible to upload a small logo or icon for providers?
// What about the image size? Should Rauthy actually process and optimize it or "just take it" and
// then show it on the login page? What about max size recommendations or simply grabbing a
// favicon from the remote URL?
// If Rauthy should process the image (preferred solution), maybe implement it for the normal
// client logos as well instead of sending back the `data:` image url for the login page
// -> more safe CSP, since we could remove the allowance of `data:` for `img` urls, but requires
// more work.
// - change GET `/providers` to POST to introduce an additional CSRF check in the backend because
// of the very sensitive `client_secret`
//
//
// Rough timeline for the implementation (multiple PRs makes sense):
// 1. Implement API Endpoint for provider metadata lookup and try to build a valid config from that.
// Return that validated config to the Admin UI with all the necessary data filled out and insert
// into (non-editable) input fields.
// 2. API Endpoint for creating an auth provider with values from the Admin UI (eventually pre-filled
// out from step 1). Do another lookup to that URL and re-validate the config, throw errors if needed.
// Create DB migrations for the new table to save all necessary data for this provider. At the same
// time, create migrations for the users and the providers tables to possibly link a user to the
// newly registered provider. Complete the DB in that step as well so everything makes sense.
// 3. API Endpoint for the `/providers/callback` with all strings attached. This needs setting either
// cookies or local storage before doing the redirect and validating these on the callback. Create
// a new HTML for that which shows at least a loading spinner to give feedback to the user that
// something is happening, because a lot of these providers are not the fastest. This means
// issuing the HTML first and on the client side, extract all values and make the final request to
// the backend manually, which gives more control over the UI part while only loosing a couple of
// ms.
// The UI / API should handle all kinds of errors with i18n even to create a good UX. On success
// though, proceed as if the user had been logging in to Rauthy directly. This needs additionally
// cached values in the backend for the whole verification -> new data structure (cache only, no DB).
//
// Modify the Login page to actually get a button for logging in with an external provider. This
// will need modifications to the HTML pre-rendering on the server after a DB / Cache lookup for
// possible providers and pre-build the redirect URI for each rendering. Another solution could be
// to just show the mandatory values and fetch additional metadata on the client side, since a user
// needs at least a second to click a button. Find the best solution for that.
//
// 4. We should have the basic stuff working now.
// We need to solve bugs that came up by now being able to logged into the account without any
// prior setup like setting a password or a passkey (possibly null / unbedinged recences in the UI).
// We also need to modify the account page and show additional information in case of a federated
// user. We should also include the ability to unlink (and maybe even link later?) a federation, if
// the user has been set up on our side properly beforehand.
//
// 5. Cleanup and quality of life improvements - if an external provider does not provide the
// metadata lookup for auto-discovery, make it able for the user to insert all values manually.
// Templates for "the big ones" like Github, Google, ..., should not be necessary, since they all
// provide the metadata endpoints. This means lees maintenance in the future.
// If the auto-lookup fails, maybe provide a small text for helping out with finding the correct
// issuer, which should have the .well-known correctly linked.
// Resolve all left over TODO's from above as well.
