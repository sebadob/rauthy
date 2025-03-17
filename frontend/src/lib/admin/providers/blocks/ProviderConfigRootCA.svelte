<script lang="ts">
    import {PATTERN_PEM} from "$utils/patterns.ts";
    import InputArea from "$lib/form/InputArea.svelte";
    import {useI18nAdmin} from "$state/i18n_admin.svelte.ts";
    import InputCheckbox from "$lib/form/InputCheckbox.svelte";

    let {
        dangerAllowInsecure = $bindable(),
        showRootPem = $bindable(),
        rootPemCert = $bindable(),
        disabled,
    }: {
        dangerAllowInsecure: boolean,
        showRootPem: boolean,
        rootPemCert: string | undefined,
        disabled?: boolean,
    } = $props();

    let ta = useI18nAdmin();
</script>

<div class="checkbox">
    <InputCheckbox ariaLabel={ta.providers.config.custRootCa} bind:checked={showRootPem}>
        {ta.providers.config.custRootCa}
    </InputCheckbox>
</div>

{#if showRootPem}
    <InputArea
            rows={17}
            name="rootPem"
            label={ta.providers.config.rootPemCert}
            placeholder="-----BEGIN CERTIFICATE-----
...
-----END CERTIFICATE-----"
            bind:value={rootPemCert}
            {disabled}
            errMsg="-----BEGIN CERTIFICATE----- ..."
            width="min(40rem, calc(100dvw - 1.75rem))"
            fontMono
            pattern={PATTERN_PEM}
    />
{:else}
    <div class="checkbox">
        <InputCheckbox
                ariaLabel={ta.providers.config.allowInsecureTls}
                bind:checked={dangerAllowInsecure}
        >
            {ta.providers.config.allowInsecureTls}
        </InputCheckbox>
    </div>
{/if}
