# E-Mail Templates

The templates for E-Mails being sent by Rauthy are baked into the binary. This is the most efficient and fastest way
of handling them. It simplifies the container images and speeds up the internal generation of new ones from templates
compared to dynamic ones. At the same time, Rauthy checks these templates for correctness at compile time. It makes
sure, that each templated value exists in the internal application logic and vice versa.

If you need to modify the content of these E-Mails, you have 2 options:

- Use the pre-defined config variables
- Clone the repo, modify the templates and build it from source

```admonish info
You can only modify the *New Password* and *Password Reset* E-Mail currently. There was no need yet for any of the
others.
```

## Existing Config Values

Modifying the E-Mail contents via config variables is very straight forward. You don't need to compile from source,
therefore can't make any mistakes and for all variables that are not set, Rauthy will simply fall back to the default.

At the time of writing, translations do exist for english (default) and german only. Values starting with `TPL_EN`
will set the english translation while `TPL_DE` will set german.

You can take a look at the raw templates directly in the repo, so you can see which value will be mapped to which
location in the final message.

[Text Version](https://github.com/sebadob/rauthy/blob/main/templates/email/reset.txt)

[HTML Version](https://github.com/sebadob/rauthy/blob/main/templates/email/reset.html)

The values to modify the sections are the following:

```
# New Password E-Mail
#TPL_EN_PASSWORD_NEW_SUBJECT="New Password"
#TPL_EN_PASSWORD_NEW_HEADER="New password for"
#TPL_EN_PASSWORD_NEW_TEXT=""
#TPL_EN_PASSWORD_NEW_CLICK_LINK="Click the link below to get forwarded to the password form."
#TPL_EN_PASSWORD_NEW_VALIDITY="This link is only valid for a short period of time for security reasons."
#TPL_EN_PASSWORD_NEW_EXPIRES="Link expires:"
#TPL_EN_PASSWORD_NEW_BUTTON="Set Password"
#TPL_EN_PASSWORD_NEW_FOOTER=""

#TPL_DE_PASSWORD_NEW_SUBJECT="Passwort Reset angefordert"
#TPL_DE_PASSWORD_NEW_HEADER="Passwort Reset angefordert für"
#TPL_DE_PASSWORD_NEW_TEXT=""
#TPL_DE_PASSWORD_NEW_CLICK_LINK="Klicken Sie auf den unten stehenden Link für den Passwort Reset."
#TPL_DE_PASSWORD_NEW_VALIDITY="Dieser Link ist aus Sicherheitsgründen nur für kurze Zeit gültig."
#TPL_DE_PASSWORD_NEW_EXPIRES="Link gültig bis:"
#TPL_DE_PASSWORD_NEW_BUTTON="Passwort Setzen"
#TPL_DE_PASSWORD_NEW_FOOTER=""

# Password Reset E-Mail
#TPL_EN_RESET_SUBJECT="Password Reset Request"
#TPL_EN_RESET_HEADER="Password reset request for"
#TPL_EN_RESET_TEXT=""
#TPL_EN_RESET_CLICK_LINK="Click the link below to get forwarded to the password request form."
#TPL_EN_RESET_VALIDITY="This link is only valid for a short period of time for security reasons."
#TPL_EN_RESET_EXPIRES="Link expires:"
#TPL_EN_RESET_BUTTON="Reset Password"
#TPL_EN_RESET_FOOTER=""

#TPL_DE_RESET_SUBJECT="Passwort Reset angefordert"
#TPL_DE_RESET_HEADER="Passwort Reset angefordert für"
#TPL_DE_RESET_TEXT=""
#TPL_DE_RESET_CLICK_LINK="Klicken Sie auf den unten stehenden Link für den Passwort Reset."
#TPL_DE_RESET_VALIDITY="Dieser Link ist aus Sicherheitsgründen nur für kurze Zeit gültig."
#TPL_DE_RESET_EXPIRES="Link gültig bis:"
#TPL_DE_RESET_BUTTON="Passwort Zurücksetzen"
#TPL_DE_RESET_FOOTER=""
```

## Modify Templates Directly

Modifying templates directly gives you the most amount of control of course. You can change everything you like about
them. However, you need to clone the whole repo, modify the templates in
[templates/email/](https://github.com/sebadob/rauthy/tree/main/templates/email), and then rebuild the whole project
from source on your own. The existing setup should make it pretty easy to do. To get started, take a look at the
[CONTRIBUTING.md](https://github.com/sebadob/rauthy/blob/main/CONTRIBUTING.md).