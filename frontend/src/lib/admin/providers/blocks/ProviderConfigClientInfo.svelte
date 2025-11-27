<script lang="ts">
    import { useI18nAdmin } from '$state/i18n_admin.svelte';
    import { PATTERN_CLIENT_NAME, PATTERN_SCOPE_SPACE, PATTERN_URI } from '$utils/patterns';
    import Input from '$lib/form/Input.svelte';
    import JsonPathDesc from '$lib/admin/providers/JsonPathDesc.svelte';
    import InputPassword from '$lib/form/InputPassword.svelte';
    import InputCheckbox from '$lib/form/InputCheckbox.svelte';
    import { slide } from 'svelte/transition';

    let {
        scope = $bindable(),
        name = $bindable(),

        clientId = $bindable(),
        clientSecret = $bindable(),
        clientSecretBasic = $bindable(),
        clientSecretPost = $bindable(),

        adminClaimPath = $bindable(),
        adminClaimValue = $bindable(),
        mfaClaimPath = $bindable(),
        mfaClaimValue = $bindable(),

        usePKCE,
        inputWidth,
    }: {
        scope: string;
        name: string;

        clientId: string;
        clientSecret: undefined | string;
        clientSecretBasic: boolean;
        clientSecretPost: boolean;

        adminClaimPath: undefined | string;
        adminClaimValue: undefined | string;
        mfaClaimPath: undefined | string;
        mfaClaimValue: undefined | string;

        usePKCE: boolean;
        inputWidth: string;
    } = $props();

    let ta = useI18nAdmin();
</script>

<p class="desc">{ta.providers.config.descScope}</p>
<Input
    bind:value={scope}
    autocomplete="off"
    label="Scope"
    placeholder="openid profile email"
    required
    pattern={PATTERN_SCOPE_SPACE}
    width={inputWidth}
/>

<p class="desc">{ta.providers.config.descClientName}</p>
<Input
    bind:value={name}
    autocomplete="off"
    label={ta.providers.config.clientName}
    placeholder={ta.providers.config.clientName}
    required
    pattern={PATTERN_CLIENT_NAME}
    width={inputWidth}
/>

<p class="desc">{ta.providers.config.descClientId}</p>
<Input
    bind:value={clientId}
    autocomplete="off"
    label="Client ID"
    placeholder="Client ID"
    required
    pattern={PATTERN_URI}
    width={inputWidth}
/>

<p class="desc">{ta.providers.config.descClientSecret}</p>
<InputPassword
    bind:value={clientSecret}
    autocomplete="off"
    label="Client Secret"
    placeholder="Client Secret"
    maxLength={256}
    errMsg={ta.providers.config.errConfidential}
    required={!usePKCE}
    width={inputWidth}
/>

<p>{@html ta.providers.config.descAuthMethod}</p>
<div class="checkbox">
    <InputCheckbox
        ariaLabel="client_secret_basic"
        bind:checked={clientSecretBasic}
    >
        client_secret_basic
    </InputCheckbox>
</div>
<div class="checkbox">
    <InputCheckbox
        ariaLabel="client_secret_post"
        bind:checked={clientSecretPost}
    >
        client_secret_post
    </InputCheckbox>
</div>
{#if !usePKCE && !clientSecretBasic && !clientSecretPost}
    <div
        class="err"
        transition:slide={{ duration: 150 }}
    >
        {ta.providers.config.errNoAuthMethod}
    </div>
{/if}

<JsonPathDesc />

<p class="desc">{ta.providers.config.mapUser}</p>
<Input
    bind:value={adminClaimPath}
    autocomplete="off"
    label={ta.providers.config.pathAdminClaim}
    placeholder="$.roles.*"
    pattern={PATTERN_URI}
    width={inputWidth}
/>
<Input
    bind:value={adminClaimValue}
    autocomplete="off"
    label={ta.providers.config.valueAdminClaim}
    placeholder="rauthy_admin"
    pattern={PATTERN_URI}
    width={inputWidth}
    required={!!adminClaimPath}
/>

<p class="desc">{ta.providers.config.mapMfa}</p>
<Input
    bind:value={mfaClaimPath}
    autocomplete="off"
    label={ta.providers.config.pathMfaClaim}
    placeholder="$.amr.*"
    pattern={PATTERN_URI}
    width={inputWidth}
/>
<Input
    bind:value={mfaClaimValue}
    autocomplete="off"
    label={ta.providers.config.valueMfaClaim}
    placeholder="mfa"
    pattern={PATTERN_URI}
    width={inputWidth}
    required={!!mfaClaimPath}
/>

<style>
    .desc {
        margin-bottom: -0.5rem;
    }
</style>
