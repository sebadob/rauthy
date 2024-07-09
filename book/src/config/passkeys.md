# Passkeys / WebAuthn

## Passkey Only Accounts

Rauthy provides the ability to create Passkey only accounts.  
These accounts do not have any password at all. The user can login via E-Mail + providing the Passkey. Only keys
with additional user verification (UV) will be accepted for these accounts. This makes sure, that they are 2FA / MFA
secured (depending on the device) all the time.

You can choose the account type either during the initial password reset link you get via E-Mail, or you can
convert a traditional password account to a passkey only account in your account view, if you have at least
one Passkey with additional UV registered.

Passkey only accounts provide a few benefits:

- no need to remember or store any passwords
- way easier and faster logging in
- always 2FA / MFA
- strongest type of authentication
- no need to satisfy password policies
- no need to reset your password after it has been expired

```admonish caution
Passkey only accounts cannot use the traditional password reset E-Mails.

This is a drawback and a benefit at the same time:  
No way to take over an account if the E-Mail account has been compromised, but at the same time the user
relies on an Admin to reset the MFA devices, if no backup exists or all are lost.
```

```admonish note
Unfortunately, passkeys have one big drawback when you use the discoverable credentials / redident keys feature.
This is really cool and the best possible UX at first glance, because the user does not need to provide even a 
username / email manually, but has one very big drawback.

**The amount of resident keys a passkey can store is always limited.**

There are devices, that can get locked up completely if you try to register resident keys beyond the capacity of 
the device itself. For this reason, Rauthy strictly discourages the use of resident keys and will always request the
user to provide the email, so you can never end up in such a situation with Rauthy. Non-discoverable credentials are
not limited per device.

Rauthy remembers the last email that has been used for a login on each machine. This provides the same best possible
UX in the end that you would get with discoverable credentials but without the drawbacks of the limited amount.
```

```admonish info
Android has finally added support for biometric UV in September 2023.  
This has made is possible to implement this feature into Rauthy without sacrificing security.

However, at the time of writing (2024/06/05), PIN / biometric UV via NFC keys like Yubikeys does still not work. 

Sadly, Google just decided to not implement this feature for other keys than their own implementation of it.
```

```admonish hint
If you want to register an Android device for a Passkey only account, but you are using for instance Yubikeys
with PIN UV, you can do the following trick to get it done (works only with the latest Play store version):

- Create a password for your account, if it does not exist yet
- Remove all registered passkeys
- Log in to your account view on your Android device and another device that works with your Yubikey
- With both devices logged in at the same time:
    - Register a new passkey with Android and choose "this device", which will create a Passkey flow with your fingerprint
    - Register the Yubikey on the other device
- You should now have 2 Passkeys: Android + Yubikey
- Navigate to the Password page inside your account on any device and convert it to Passkey only
- You should now be able to log in on your Android device with Passkey only and with your Yubikey
```

## Config

You should use Passkeys / Webauthn in production for 2FA / MFA.
To make sure it works, you need to check your the config.

Adjust the following variables in your config, to make sure it works correctly.

### `RP_ID`

This is the *Relaying Party (RP) ID*, which should be your effective domain name.  
Let's say our application is available at `auth.example.com`, then this should be:

```
RP_ID=auth.example.com
```

```admonish caution
When the `RP_ID` changes, already registered devices will stop working and users cannot log in anymore!
Be very careful, if you want / need to do this in production.
```

### `RP_ORIGIN`

The second important variable is the `RP_ORIGIN`. This needs to be set to the scheme + URL containing the effective
domain name + port.

```admonish caution
The `RP_ORIGIN` must always include the port number, even if it is just the default 443 for HTTPS.
```

In this example, assuming Rauthy will be available at port 443, correct would be:

```
RP_ORIGIN=https://auth.example.com:443
```

### `RP_NAME`

This variable can be set to anything "pretty".  
This may be shown to the user in a way like "`RP_BNAE` requests your security key ...". If this is shown depends on the
OS and the browser the client uses. Firefox, for instance, does not show this at the time of writing.

You can change the `RP_NAME` later on without affecting the validation of already registered keys.

### `WEBAUTHN_RENEW_EXP`

For all non Passkey only accounts, Rauthy will always prompt a user at least once for the password on a new machine,
even with active
passkeys. The keys are used either as a strong second factor, when they do not work with a PIN, or bump up the whole
login to real MFA, if the OS / Browser / Key does support this.

When a user has logged in successfully on a new device and active 2FA / MFA, Rauthy will set an encrypted cookie as a
"remember me". The lifetime of this cookie can be configured with `WEBAUTHN_RENEW_EXP`. The **default** of this value
is **2160 hours**.

As long as this cookie is present and can be decrypted by the backend, the user can log in from this very
device with the registered Passkey key only, which makes a very good user experience for the whole login flow.
The E-Mail will already be filled automatically and only a single click on the login button is necessary.
