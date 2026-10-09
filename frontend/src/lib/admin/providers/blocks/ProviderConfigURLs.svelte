<script lang="ts">
    import { useI18nAdmin } from '$state/i18n_admin.svelte';
    import { PATTERN_URI } from '$utils/patterns';
    import Input from '$lib/form/Input.svelte';

    let ta = useI18nAdmin();

    let {
        issuer = $bindable(),
        authorizationEndpoint = $bindable(),
        tokenEndpoint = $bindable(),
        userinfoEndpoint = $bindable(),
        inputWidth,
        disabled,
    }: {
        issuer: string;
        authorizationEndpoint: string;
        tokenEndpoint: string;
        userinfoEndpoint: string;
        inputWidth: string;
        disabled?: boolean;
    } = $props();
</script>

<Input
    bind:value={issuer}
    autocomplete="off"
    label={ta.providers.config.issuerUrl}
    placeholder={ta.providers.config.issuerUrl}
    required
    pattern={PATTERN_URI}
    width={inputWidth}
    {disabled}
/>
<Input
    typ="url"
    bind:value={authorizationEndpoint}
    autocomplete="off"
    label={ta.providers.config.authorizationEndpoint}
    placeholder={ta.providers.config.authorizationEndpoint}
    required
    pattern={PATTERN_URI}
    width={inputWidth}
    {disabled}
/>
<Input
    typ="url"
    bind:value={tokenEndpoint}
    autocomplete="off"
    label={ta.providers.config.tokenEndpoint}
    placeholder={ta.providers.config.tokenEndpoint}
    required
    pattern={PATTERN_URI}
    width={inputWidth}
    {disabled}
/>
<Input
    typ="url"
    bind:value={userinfoEndpoint}
    autocomplete="off"
    label={ta.providers.config.userinfoEndpoint}
    placeholder={ta.providers.config.userinfoEndpoint}
    required
    pattern={PATTERN_URI}
    width={inputWidth}
    {disabled}
/>
