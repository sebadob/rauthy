<script lang="ts">
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import { PATTERN_CLIENT_NAME, PATTERN_SCOPE_SPACE, PATTERN_URI } from '#utils/patterns.ts';
    import Input from '#lib/form/Input.svelte';
    import JsonPathDesc from '#lib/admin/providers/JsonPathDesc.svelte';
    import InputPassword from '#lib/form/InputPassword.svelte';
    import InputCheckbox from '#lib/form/InputCheckbox.svelte';
    import { slide } from 'svelte/transition';
    import type { Snippet } from 'svelte';
    import Section from '#lib/Section.svelte';

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
        clientControls,
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
        clientControls?: Snippet;
    } = $props();

    let ta = useI18nAdmin();
</script>

<Section title={ta.providers.config.sections?.client ?? 'Client credentials'}>
    <div class="credentials">
        <div class="fields" class:integrated={!!clientControls}>
            <Input
                bind:value={name}
                autocomplete="off"
                label={ta.providers.config.clientName}
                placeholder={ta.providers.config.clientName}
                description={ta.providers.config.descClientName}
                required
                pattern={PATTERN_CLIENT_NAME}
                width={inputWidth}
            />
            <Input
                bind:value={scope}
                autocomplete="off"
                label="Scope"
                placeholder="openid profile email"
                description={ta.providers.config.descScope}
                required
                pattern={PATTERN_SCOPE_SPACE}
                width={inputWidth}
            />
            <Input
                bind:value={clientId}
                autocomplete="off"
                label="Client ID"
                placeholder="Client ID"
                description={ta.providers.config.descClientId}
                required
                pattern={PATTERN_URI}
                width={inputWidth}
            />
            <InputPassword
                bind:value={clientSecret}
                autocomplete="off"
                label="Client Secret"
                placeholder="Client Secret"
                description={ta.providers.config.descClientSecret}
                maxLength={256}
                errMsg={ta.providers.config.errConfidential}
                required={!usePKCE}
                width={inputWidth}
            />
        </div>
    </div>
    <div class="methods">
        <h4>{ta.providers.config.sections?.tokenAuth ?? 'Token authentication'}</h4>
        <div class="switches">
            {@render clientControls?.()}
            <InputCheckbox ariaLabel="client_secret_basic" bind:checked={clientSecretBasic}>
                client_secret_basic
            </InputCheckbox>
            <InputCheckbox ariaLabel="client_secret_post" bind:checked={clientSecretPost}>
                client_secret_post
            </InputCheckbox>
        </div>
        <p class="ui-muted-copy">{@html ta.providers.config.descAuthMethod}</p>
        {#if !usePKCE && !clientSecretBasic && !clientSecretPost}
            <div class="err" transition:slide={{ duration: 150 }}>
                {ta.providers.config.errNoAuthMethod}
            </div>
        {/if}
    </div>
</Section>

<Section title={ta.providers.config.sections?.claims ?? 'Claim mapping'}>
    <div class="claims">
        <p class="ui-muted-copy">{ta.providers.config.jsonPath.p1}</p>
        <fieldset class="claim-pair">
            <legend>{ta.providers.config.sections?.adminMapping ?? 'Admin access'}</legend>
            <p class="ui-muted-copy">{ta.providers.config.mapUser}</p>
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
        </fieldset>
        <fieldset class="claim-pair">
            <legend
                >{ta.providers.config.sections?.mfaMapping ?? 'Multi-factor authentication'}</legend
            >
            <p class="ui-muted-copy">{ta.providers.config.mapMfa}</p>
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
        </fieldset>
        <JsonPathDesc />
    </div>
</Section>

<style>
    .credentials,
    .claims {
        container-type: inline-size;
    }

    .fields,
    .methods,
    .claims,
    .claim-pair {
        display: grid;
        gap: var(--ui-space);
        align-items: start;
        --ui-field-margin: 0;
    }

    .methods {
        padding-top: var(--ui-space);
        border-top: 1px solid var(--ui-outline);
        gap: 0.5rem;
    }

    h4,
    legend {
        color: var(--ui-text);
        font-size: var(--ui-control-font-size);
        font-weight: 600;
    }

    .switches {
        display: flex;
        flex-wrap: wrap;
        gap: 0.5rem 1rem;
    }

    .claim-pair {
        min-inline-size: 0;
        padding: 0.75rem;
        border: 1px solid var(--ui-outline);
        border-radius: var(--border-radius);
        gap: 0.75rem 1rem;
    }

    legend {
        padding-inline: 0.375rem;
    }

    .claim-pair > p {
        grid-column: 1 / -1;
    }

    @container (width >= 34rem) {
        .fields.integrated,
        .claim-pair {
            grid-template-columns: repeat(2, minmax(0, 1fr));
        }
    }

    .fields :global(.admin-password-field) {
        margin-block: 0;
    }
</style>
