# User Registration

By default, new users can only be added by an admin. This is safe and secure, and your database
cannot ever be spammed.
However, you might want users to register themselves for whatever reason. In that case, follow the
next step.

## Open Registration

To open the registration to anyone, just set

```toml
[user_registration]
# If the User Registration endpoint should be accessible by anyone.
# If not, an admin must create each new user.
#
# default: false
# overwritten by: OPEN_USER_REG
enable = true
```

This will open the registration endpoint and make it accessible without upfront authentication.  
You will now see a new button at the page root which directs you to the very simple registration
form.

After successful registration, the user will receive an E-Mail with an activation link.
Clicking on this link will direct the user on a page, where a new password or passkey can be set.
At the same time, the account will be activated and the E-Mail will be validated as well of course.

```admonish caution
You must have configured your E-Mail sending beforehand. Otherwise the registration process will not work.
You need to configure the following values to be able to send out E-Mails:

- SMTP_URL
- SMTP_USERNAME
- SMTP_PASSWORD
```

## Captcha / Proof of Work

The registration form uses a *Proof of Work (PoW)* behind the scenes. This is basically an invisible
captcha
without the user solving weird image puzzles that sometimes even humans cannot solve. It is done
with the help of a
tiny crate I wrote myself as well.

This will of course not prevent real humans from registering fake accounts, but until now, I never
had issues with any
bots, so it does what it should while providing a way better UX than any traditional captcha
challenge.

```admonish hint
If you are interested in how it works, take a look at [spow](https://github.com/sebadob/spow).  
```

## Restricted Registration

You may want your users to register themselves, but at the same time restrict the E-Mail domains
they are using.
For instance, when you deploy Rauthy at your company for all internal applications, you may only
want users to
register with their work E-Mail:

```toml
[user_registration]
# Can be used when 'open_user_reg = true' to restrict the domains for
# a registration. For instance, set it to
# 'user_reg_domain_restriction = gmail.com' to allow only
# registrations with 'user@gmail.com'.
#
# overwritten by: USER_REG_DOMAIN_RESTRICTION
domain_restriction = 'my-domain.com'
```

## Domain Blacklisting

If you have opened your registration to anyone, you will get into the situation at some point, where
evil people will
create accounts only for checking out your security and trying to break in, execute XSS, and so on.
These are often
real people, which means any Captcha or PoW will not prevent them from registering of course.

The best thing you can do in that case, if your deployment allows this, is Geoblocking certain
regions at your
firewall / reverse proxy level. Attackers would then switch to origins in a country you allow, but
this is usually
more costly for them and more hassle, so it is a good first line of defense. At the same time, it
reduces the visibility
to those regions, and you might not get targeted in the first place because bots cannot scan you
from there.

When such attacks happen, there is no magic rule or setting to defend against them. This is really
individual each time
and depends on a lot of things out of the scope of this documentation.

Nevertheless, Rauthy provides a small feature that might help you here: **E-Mail Domain Blacklisting
**

Let's say you can't Geoblock or you found out, that specific E-Mail providers are being used during
these attacks.
If you have such a list of evil providers, you can blacklist and exclude them from the open
registration. Existing users
will keep working and an Admin can still create users with these domains. They just cannot be used
for self-registration
anymore.

You have the following config option:

```toml
[user_registration]
# Can be used when 'open_user_reg = true' to restrict the domains for
# a registration. For instance, set it to
# 'user_reg_domain_restriction = gmail.com' to allow only
# registrations with 'user@gmail.com'.
#
# overwritten by: USER_REG_DOMAIN_RESTRICTION
#domain_restriction = 'my-domain.com'

# If `open_user_reg = true`, you can blacklist certain domains on
# the open registration endpoint.
#
# default: []
# overwritten by: USER_REG_DOMAIN_BLACKLIST - single String, \n separated values
domain_blacklist = [
  'example.com',
  'evil.net',
]
```

```admonish note
If you get targeted by something like fake accounts for testing your security and so on, don't panic.
These attacks usually stop after 1-2 weeks most often. When attackers did not find a way in, they loose interest. 
```

## Downstream Application Integration

You can integrate the registration into a downstream application on 2 different ways.  
Either use the existing one with redirect hints for improved UX, or fully build your own
registration frontend.

### Redirect Hints

Let's say you have an application that uses Rauthy for user management. You want to improve the
registration flow
and your users being redirected back to your downstream app after registration / password set
instead of them
"being stuck" on Rauthy's UI, which would be the default flow.

Your app can show a link to the existing registration form with an appended query param to control
redirects.
When you append

```
?redirect_uri=https%3A%2F%2Fgithub.com
```

to the link, so you end up with for instance

```
https://iam.example.com/auth/v1/users/register?redirect_uri=https%3A%2F%2Fgithub.com
```

The following things will happen:

1. After a successful registration, the user will be redirected back to the given `redirect_uri`.
2. After the password / passkey has been set using the E-Mail link, instead of being redirected to
   the Rauthy
   account dashboard, the user will be redirected to the same URI again.

This makes it possible to use Rauthy as your upstream provider without the user really needing to
interact with or
know about it in detail, which again leads to less confusion.

By default, the allowed `redirect_uri`s are restricted to all existing `client_uri`s in the
database. They will be
compared via `client_uri.startsWith(redirect_uri)`. If you want to opt-out of the additional
redirect_uri checks and
configure and open redirect to allow just anything, you can do so:

```toml
[user_registration]
# If set to `true`, any validation of the `redirect_uri` provided
# during a user registration will be disabled. Clients can use
# this feature to redirect the user back to their application
# after a successful registration, so instead of ending up in the
# user dashboard, they come back to the client app that initiated
# the registration.
#
# The given `redirect_uri` will be compared against all registered
# `client_uri`s and will throw an error, if there is no match.
# However, this check will prevent ephemeral clients from using
# this feature. Only if you need it in combination with ephemeral
# clients, you should set this option to `true`. Otherwise, it is
# advised to set the correct Client URI in the admin UI. The
# `redirect_uri` will be allowed if it starts with any registered
# `client_uri`.
#
# default: false
# overwritten by: USER_REG_OPEN_REDIRECT
allow_open_redirect = false
```

### Custom Frontend

Depending on your application, you may want to create your own frontend for the registration. For
speed and efficiency
reasons, Rauthy does not allow you to overwrite the existing templates, but you can host your own UI
of course.

The registration page is super simple and you can take a look at what it does here:
[page.svelte](https://github.com/sebadob/rauthy/blob/main/frontend/src/routes/users/register/%2Bpage.svelte)

The registration endpoint allows CORS requests. The only thing you need to care about is the PoW
calculation.

1. Accept the input from the user via any form inputs
2. When the user clicks submit, fetch a new PoW from Rauthy via `POST /auth/v1/pow`
3. Solve the PoW using [spow](https://github.com/sebadob/spow)
4. As soon as the PoW is solved, do a `POST /auth/v1/users/register` with the payload

```rust
struct NewUserRegistrationRequest {
  #[validate(email)]
  email: String,
  /// Validation: `[a-zA-Z0-9À-ÿ-'\\s]{1,32}`
  #[validate(regex(path = "*RE_USER_NAME", code = "[a-zA-Z0-9À-ɏ-'\\s]{1,32}"))]
  family_name: String,
  /// Validation: `[a-zA-Z0-9À-ÿ-'\\s]{1,32}`
  #[validate(regex(path = "*RE_USER_NAME", code = "[a-zA-Z0-9À-ɏ-'\\s]{1,32}"))]
  given_name: String,
  /// Validation: `[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%]+`
  #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]+"))]
  pow: String,
  /// Validation: `[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%]+`
  #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]+"))]
  redirect_uri: Option<String>,
}
```

```admonish caution
Do not fetch and solve a PoW when the user has not submitted the form yet!  

The PoWs have a very short lifetime by default to prevent them from being used multiple times. Rauthy has additional 
re-use prevention, but a POST to get a PoW will modify the backend state. This is unnecessary if the user decides to not
submit the form at all.

You can configure PoWs with `pow.difficulty` and `pow.exp`.  
Keep in mind, that the `pow.exp` should be a high as necessary but always as low as possible.
```
