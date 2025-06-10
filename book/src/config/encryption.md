# Encryption

Rauthy does additional encryption for values in lots of places, like for instance `client_secret`s in the database
or session cookies. In the Getting Started for Kubernetes, we have set up the `encryption.keys` and
`encryption.key_active`.

The `encryption.keys` defines the static keys used for additional data encryption in a few places. This values may
contain multiple keys, if you want to rotate them at some point without breaking the decryption of all already existing
secrets.

`encryption.key_active` defines the key inside `encryption.keys` which will be used as the default. This means that all
new / current encryption's performed by the backend will use the key with the given ID.

## Setup

If you followed the Getting Started for Kubernetes, you already completed this step. If not, this is how you can
generate an encryption key.

```
echo "$(openssl rand -hex 4)/$(openssl rand -base64 32)"
```

The first part until the first `/` is the key ID. This has to be between 2 and 20 characters and should not contain any
special ones. The second part after the first `/` is the key itself. This value **must** be exactly 32 bytes long and
then be base64 encoded. If it is not, Rauthy will yell at startup and panic early.

If you have generated a key, lets say the output was

```
❯ echo "$(openssl rand -hex 4)/$(openssl rand -base64 32)"
90eb6d69/U9wZG4GS/94pVh6iTH1ijf+kj+tXJHKkQNsp5eImMQI=
```

Your config value should look like this:

```toml
[encryption]
keys = ["90eb6d69/U9wZG4GS/94pVh6iTH1ijf+kj+tXJHKkQNsp5eImMQI="]
key_active = "90eb6d69"
```

You can add more keys if you like, **separated by new lines**, which is needed for the key rotation described below.
The `encryption.key_active` will be the key being used for all new encryption's.

```admonish note
It seems that in some environments, the above `openssl` command does not output proper values, which will make Rauthy
panic on startup, when it checks the given values. If you run into that situation, you can generate them without 
`openssl` as well, with e.g:

<pre><code>echo "$(cat /dev/urandom | tr -dc 'a-zA-Z0-9' | head -c 6)/$(cat /dev/urandom | head -c 32 | base64)"</code></pre>
```

## Key Rotation

A good practice is to rotate encryption keys from time to time. Rauthy does auto-rotate the keys for signing tokens,
but the admin is responsible for the encryption keys, since they have a greater impact.

**1. Add a new key to the `encryption.keys` in you secrets**

```admonish fail
You **must not remove** a current key, before the migration has been done via the UI.  
If the old key is gone, the migration will fail.
```

**2. Generate a new key + id**

```
echo "$(openssl rand -hex 4)/$(openssl rand -base64 32)"
```

The part before the first `/` is the `key_active` and the part afterwards is the key base64 encoded.
You might set the new `encryption.key_active` to the newly generated key ID.

Keep in mind, you need to ADD this to your existing keys and not just replace them! If you just replace them, almost
all things will break and fall apart.

The final format of the `encryption.keys` should look something like this, for instance:

```toml
[encryption]
key = [
    "Wx1zrbLF/5vTaB7LdUSg1aTecmqHJOu2+RnU6zgTwNkDQU52Y3JM=",
    "6uf5QebA/9DsKMoq8A+Gn2WQrTcSpz5sg751yYs3IJlkw3dn0rII=",
]
```

In this example, if the first key should be the new active default, set your `key_active` to

```toml
[encryption]
key_active = "Wx1zrbLF"
```

**3. Set the `key_active` to the ID of your newly generated key**

This will make sure, that all new encryptions will use the new key. If you do not care about removing the old keys,
because you maybe just want to rotate because its good practice, the secrets will migrate "by themselves" over time.
If Rauthy finds any secrets during its normal operation, that have been encrypted with an older key than the current
`key_active`, it will re-encrypt these secrets and update the values.  
This means, you may just stop at this point, if this is good enough for you.

**4. Migrate Keys**

If you however want to trigger a re-encryption of all existing secrets on purpose, there is a small tool in the
Admin UI which helps you with this.

Log in to the Admin UI and navigate to `Config` -> `Encryption Keys`.  
You will see the currently recognized keys and the currently active ID.

You can then make sure, that the ID you want to migrate secrets to is selected and execute the migrations.
Please keep in mind, that if you have a lot of data, it might take a few seconds to perform this operation.
This will migrate all encrypted data for existing OIDC clients, all JWKs, and so on, with the new key.

**5. Remove old keys**

After a successful migration via the UI tool, you could remove old keys from the `keys` value, but it is not
recommended as long as you did not have a known data breach. Just keep them around for some time because of
encrypted cookies with older keys.

```admonish caution
All cookies are encrypted with the `key_active`. This means, if you remove something from the `keys` which has
been used to encrypt cookies, the user will be prompted to log in again, even if cookies have not expired yet. 
```

```admonish note
Rauthy uses ChaCha20Poly1305 for any encryption. AES GCM is not used on purpose, because it has some attack vectors if 
its used without hardware acceleration.  

Usually, devices these days all come with dedicated AES acceleration, even embedded ones. However, with VM's this is 
often a different story and its not guaranteed, that you will have AES acceleration when you spin up a VM in some cloud. 
Rauthy tries to be as secure as possible by default and therefore ChaCha20Poly1305 has a slight advantage over AES.
```