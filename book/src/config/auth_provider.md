# Upstream Authentication Provider

Rauthy works with upstream authentication providers, if you are using one already.  
The configuration is pretty straightforward and will be shown with GitHub in this example.

1. Create a new client on your upstream provider.

In case of GitHub, open the `Settings` for your account,  
then navigate to `Developer Settings`  
and `OAuth Apps` -> `Register a new application`

In this screenshot, I used a host on my local network for testing.

TODO add screenshot

After the registration, you will get a `Client ID` and you can `Generate a new client secret`

TODO add screenshot

https://192.168.14.50:5173/auth/v1/providers/callback?error=
redirect_uri_mismatch&
error_description=The+redirect_uri+MUST+match+the+registered+callback+URL+for+this+application.&error_uri=https%3A%2F%2Fdocs.github.com%2Fapps%2Fmanaging-oauth-apps%2Ftroubleshooting-authorization-request-errors%2F%23redirect-uri-mismatch&state=HqRTg4Xsztnc41FhNspCW1zqZTzr5J5R