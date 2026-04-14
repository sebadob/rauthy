# E-Mail Templates

The templates for E-Mails being sent by Rauthy are baked into the binary. This is the most efficient
and fastest way of handling them. It simplifies the container images and speeds up the internal
generation of new ones from templates compared to dynamic ones. At the same time, Rauthy checks
these templates for correctness at compile time. It makes sure, that each templated value exists in
the internal application logic and vice versa.

If you need to modify the content of these E-Mails, you have 2 options:

- Use the pre-defined config variables
- Clone the repo, modify the templates and build it from source

```admonish info
You can only modify the *New Password* and *Password Reset* E-Mail currently. There was no need yet for any of the
others.
```

## Existing Config Values

Modifying the E-Mail contents via config variables is very straight forward. You don't need to
compile from source, therefore can't make any mistakes and for all variables that are not set,
Rauthy will simply fall back to the default.

You can take a look at the raw templates directly in the repo, so you can see which value will be
mapped to which location in the final message.

[Text Version](https://github.com/sebadob/rauthy/blob/main/templates/email/reset.txt)

[HTML Version](https://github.com/sebadob/rauthy/blob/main/templates/email/reset.html)

The values are given as multiplem `[[templates]]` blocks. Each given block must match a separate
`lang` / `typ` combination to be valid:

```toml
[[templates]]
# You can overwrite some default email templating values.
# If you want to modify the basic templates themselves, this is
# only possible with a custom build from source. The content
# however can mostly be set here.
# If the below values are not set, the default will be taken.
#
# NOTE: This is an array value, and you can specify it multiple
# times for different `lang` / `typ` combinations.

# one of: en de ko uk zh_hans
lang = 'en'
# one of: password_new, password_reset, email_registered_already
typ = 'password_new'

subject = 'New Password'
header = 'New password for'

# Note: This value will be empty for `password_new` and `password_reset`.
# You can set custom content that you wish to add to your emails though.
# For `email_registered_already`, it will have a default text.
#text = """Someone tried to register a new account with your E-Mail address
#while an account exists already. If that was you, and it was a mistake, you
#can ignore this message. However, if you forgot your password, you can use
#the link belopw to reset it.
#If it was not you trying to register the new account - no need to worry.
#Your account has not been compromised and no data was leaked."""

# Exists only for: password_new, password_reset
click_link = 'Click the link below to get forwarded to the password form.'
# Exists only for: password_new, password_reset
validity = 'This link is only valid for a short period of time for security reasons.'
# Exists only for: password_new, password_reset
expires = 'Link expires:'
footer = ''

# Exists only for password reset E-Mails for initialized accounts or
# `email_registered_already`
button_text_request_new = ''
```

## Modify Templates Directly

Modifying templates directly gives you the most amount of control of course. You can change
everything you like about them. However, you need to clone the whole repo, modify the templates in
[templates/email/](https://github.com/sebadob/rauthy/tree/main/templates/email), and then rebuild
the whole project from source on your own. The existing setup should make it pretty easy to do. To
get started, take a look at
the [CONTRIBUTING.md](https://github.com/sebadob/rauthy/blob/main/CONTRIBUTING.md).
