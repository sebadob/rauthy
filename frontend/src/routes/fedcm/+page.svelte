<script>
    import {onMount} from "svelte";
    import {getKey} from "$lib/utils/helpers.js";

    let configUrl = '';
    let isSupported = false;
    let isLoggedIn = false;
    let credentials = '';

    onMount(async () => {
        configUrl = `${window.location.origin}/auth/v1/fed_cm/config`;

        if (window["IdentityProvider"] && // Is FedCM available?
            IdentityProvider.register != undefined  // Is the IdP Registration API available?
        ) {
            console.log('FedCM is supported');
            isSupported = true;
        } else {
            console.error('FedCM is not supported');
            return;
        }

        // console.log(IdentityProvider);
        // let state = IdentityProvider.getState();
        // console.log(state);
    });

    async function login() {
        try {
            credentials = await navigator.credentials.get({
                identity: {
                    context: "signin",
                    providers: [{
                        configURL: "any",
                        clientId: "fedcm",
                        nonce: getKey(48),
                    }]
                }
            })
        } catch (err) {
            console.log('FedCM credentials error: ' + err);
        }
    }

    function register() {
        console.log('Rauthy FedCM configUrl: ' + configUrl);
        IdentityProvider.register(configUrl);
    }
</script>

<h1>FedCM Testing</h1>
<p>
    <b>FedCM supported:</b>
    {#if isSupported}
        yes
    {:else}
        no -> enable idp registration:<br>
        <code>chrome://flags/#fedcm-idp-registration</code>
    {/if}
</p>

{#if isSupported}
    <p>
        <button on:click={register}>Register IdP</button>
    </p>

    <p>
        <button on:click={login}>Login</button>
    </p>
{/if}

{#if credentials}
    <p>{credentials}</p>
{/if}
