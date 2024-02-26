/// This module is basically a more generic version of the `rauthy-client`.
/// It only implements the very basics that are needed to use upstream auth providers for logging
/// in to Rauthy. It less strict than `rauthy-client` and works with not only OIDC providers, but
/// also oauth2 to have the most possible compatibility.
///
/// This module is a big TODO right now. The below notes are just for the development process.
///
/// The main idea of the login flow is the following:
/// - Show additional buttons for each configured provider
/// - When a user chooses an external one, cache all request params for the `/authorization` with
/// a new uid for later use
/// - Save a new, encrypted cookie on the client with the uid for the initial params
/// - Redirect the client to the external provider
/// - After the external stuff has been done, the user will land on the callback URL, where Rauthy
/// extracts the encrypted request params uid and does a cache lookup for the initial data
/// - Get the tokens from the provider and extract all necessary claims
/// - If everything was fine, continue with the login flow as is the user would have used Rauthy
/// directly via the `/authorization` page and delete the cookie data.
///
/// Notes and TODO:
/// - [ ] Providers should be configurable over the Admin UI:
///     - [ ] Either, enter the ISS URL and Rauthy will try to do an auto-lookup and config
///     - [ ] Or, provide an options list in the UI with templated values for the most known out there
/// - [ ] The amount of configurable providers should not be limited
/// - [ ] Providers will be stored in a new table with all the necessary information, as well as some
/// UX things like a logo and nice to read name.
/// - [ ] The providers must be connected to a user account on creation with something like a
/// `federation_id`, which will be a DB FK to a new table with additional data for this user from
/// the provider (provider id, external uid, ...).
/// - [ ] When the migrations on the `users` table will be done, add another (not related to this feature)
/// column named `last_login` (Needed feature - makes sense to do just one migration and not multiple).
/// - We don't really need to care about JWKs and validation of external providers' tokens, because
/// we will only use them once after Rauthy has changed them for the `code` on the callback. At this
/// point, there is no need to validate the tokens, since they just came from the external provider
/// over an encrypted connection. This simplifies this module and means we don't need caching.
/// - [ ] Implement all necessary additional API endpoints in `rauthy-handlers`
/// - [ ] Create a nice looking callback page for the user with maybe additional information, because
/// there are a lot of things that can go wrong during the flow with external providers. If everything
/// was good, we could just show a loading spinner for now.
/// - [ ] Define mandatory mechanisms and features, that external providers must support to make
/// everything working and secure and write these down in the Rauthy book (new section).
/// - [ ] It should be possible to add custom Root CA's for each providers independently.
/// - [ ] It should be possible for internal networks and testing that insecure TLS connections
/// are opt-in for each provider independently.
/// - [ ] Show additional information in each users account page, if it is a federated account.
/// - [ ] Make it possible to create a Password / Passkey for federated accounts, so this flow may
/// be used for the initial setup only and use Rauthy's advanced security mechanisms afterwards.
/// - [ ] Think about a solution for the case, that a user is already registered with Rauthy, does
/// an external login flow and we find out, that the exact same E-Mail is used:
///    - Update this user with the external federation data or reject it?
///    - We cannot know, which values should have the priority in case of conflicts.
///    - Simply always federate it and only use values that Rauthy does not have?
///    - Should it be possible to "un-federate" an account on Rauthy and basically remove the link?
///    - At the same time, should it be possible to federate it on purpose via account page?
