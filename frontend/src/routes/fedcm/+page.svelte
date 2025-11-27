<script lang="ts">
    import { onMount } from 'svelte';
    import Button from '$lib5/button/Button.svelte';
    import Input from '$lib5/form/Input.svelte';
    import CheckIcon from '$lib5/CheckIcon.svelte';
    import Main from '$lib5/Main.svelte';
    import ContentCenter from '$lib5/ContentCenter.svelte';
    import { PATTERN_URI } from '$utils/patterns';
    import Form from '$lib5/form/Form.svelte';
    import { genKey } from '$utils/helpers';

    let isSupported = $state(false);
    let isLoggedIn = $state(false);
    let credentials = $state('');
    let credentialType = $state('');

    let clientId = $state('fedcm');
    let configUrl = $state('any');

    onMount(async () => {
        configUrl = `${window.location.origin}/auth/v1/fed_cm/config`;
        clientId = `${window.location.origin}/auth/v1/fed_cm/client_config`;

        let res = await fetch('/auth/v1/fed_cm/status');
        if (res.status === 200) {
            console.log('FedCM status is: logged-in');
            isLoggedIn = true;
        } else {
            console.log('FedCM status is: logged-out');
        }

        // @ts-ignore
        if (
            window['IdentityProvider'] && // Is FedCM available?
            // @ts-ignore
            IdentityProvider.register != undefined // Is the IdP Registration API available?
        ) {
            console.log('FedCM is supported');
            isSupported = true;
        } else {
            console.error('FedCM is not supported');
        }
    });

    async function login() {
        // console.log('using credentials get values: configUrl: ' + configUrl + ' / clientId: ' + clientId);

        try {
            let creds = await navigator.credentials.get({
                // @ts-ignore
                identity: {
                    // context: "sign-in",
                    mode: 'button', // button mode will actually open the `login_url` if `logged-out`
                    providers: [
                        {
                            configURL: configUrl,
                            clientId: clientId,
                            // CAUTION: the genKey is not crypto safe - just for testing!
                            nonce: genKey(48),
                        },
                    ],
                },
            });

            console.log(creds);
            // @ts-ignore
            credentialType = creds.type;
            // @ts-ignore
            credentials = creds.token;
            isLoggedIn = true;
        } catch (err) {
            console.error('FedCM credentials error: ' + err);
        }
    }

    function register() {
        // @ts-ignore
        IdentityProvider.register(configUrl);
    }
</script>

<Main>
    <ContentCenter>
        <div class="flex-col">
            <h1>FedCM Testing</h1>
            <div class="row">
                <div><b>FedCM supported:</b></div>
                <div class="check">
                    <CheckIcon check={isSupported} />
                </div>
            </div>
            {#if !isSupported}
                <br />-> enable idp registration:<br />
                <code>chrome://flags/#fedcm-idp-registration</code>
            {/if}

            {#if isSupported}
                <p>
                    <Button
                        onclick={register}
                        level={2}>Register IdP</Button
                    >
                </p>

                <Form
                    action=""
                    onSubmit={login}
                >
                    <p>
                        <Input
                            bind:value={clientId}
                            autocomplete="off"
                            label="Client ID"
                            placeholder="Client ID"
                            required
                            pattern={PATTERN_URI}
                        />
                        <Input
                            bind:value={configUrl}
                            autocomplete="off"
                            label="Config URL"
                            placeholder="Config URL"
                            pattern={PATTERN_URI}
                        />
                        <Button type="submit">Login</Button>
                    </p>
                </Form>

                <div class="row">
                    <b>Logged In:</b>
                    <div class="check">
                        <CheckIcon check={isLoggedIn} />
                    </div>
                </div>
            {/if}
        </div>
    </ContentCenter>
</Main>

{#if credentials}
    <div class="row">
        <b>Credential Type:</b>
        <span>{credentialType}</span>
    </div>
    <div class="token">
        {credentials}
    </div>
{/if}

<style>
    .check {
        margin-bottom: -0.3rem;
    }

    .row {
        display: inline-flex;
        align-items: center;
        gap: 0.25rem;
    }

    .token {
        word-wrap: break-word;
        width: 20rem;
    }
</style>
