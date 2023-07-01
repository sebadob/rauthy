# FIDO 2 / WebAuthn

You should use FIDO 2 in production for 2FA / MFA.
To make sure it works, you need to check your the config.

Set / check some variables in your config, to make sure it works correctly.

## `RP_ID`

This usually is the 'Relaying Party (RP) ID', which should be your effective domain name.
For the above example, since our application is available under 'auth.example.com', this should also be:

```
RP_ID=auth.example.com
```

```admonish caution
When the `RP_ID` changes, already registered devices will stop working and users cannot log in anymore!
Be very careful, if you want / need to do this in production.
```

## `RP_ORIGIN`

The seconds important variable for FIDO 2 is the `RP_ORIGIN`. This needs to be set to the URL containing the effective
domain name.

```admonish caution
The `RP_ORIGIN` must always include the port number, even it is just the default 443 for HTTPS.
```

In this example, assuming rauthy will be available at port 443, correct would be:

```
RP_ORIGIN=https://auth.example.com:443
```

## `RP_NAME`

This variable can be set to anything "nice".
This may be shown to the user in a way like "`RP_BNAE` requests your security key ...". If this is shown depends on the
OS and the browser the client uses. Firefox, for instance, does not show this at the time of writing.

You can change the `RP_NAME` later on without affecting the validation of already registered keys.

## `WEBAUTHN_RENEW_EXP`

In my opinion, passwordless login with WebAuthn is the best thing for the user experience and the safest too.

However, not all operating systems and browsers have caught up fully yet until a point, where I would use only WebAuthn
on its own. Firefox for instance is a good example. On Linux and Mac OS, it does not work with a PIN or any other second
factor on your device, which basically downgrades the login to a (strong) single factor again.

For this reason, Rauthy will always prompt a user at least once for the password on a new machine, even with active
security keys. The keys are used either as a strong second factor, when they do not work with a PIN, or bump up the whole
login to real MFA, if the OS / Browser / Key does support this.

When a user as logged in successfully on a new device and active 2FA / MFA, he will get an encrypted cookie.  
The lifetime of this cookie can be configured with `WEBAUTHN_RENEW_EXP`.  
The **default** of this value is **2160 hours**. 

As long as this cookie is present, non-expired and can be encrypted by the backend, the user can log in from this very
device with his FIDO 2 key only, which makes a very good user experience for the whole login flow. The E-Mail will
already be filled automatically and only a single click on the login button is necessary.
