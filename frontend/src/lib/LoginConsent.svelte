<script lang="ts">
    import type { ConsentContentTemplate, ConsentScope } from '#api/templates/ConsentContent.ts';
    import { useI18n } from '#state/i18n.svelte.ts';
    import { useMarkdownWorker } from '#lib/text_edit/useWorker.svelte.ts';
    import InputCheckbox from '#lib/form/InputCheckbox.svelte';
    import Button from '#lib/button/Button.svelte';

    let {
        consent,
        clientName,
        accepted = $bindable(),
        confirmed = $bindable(),
        disabled,
    }: {
        consent: ConsentContentTemplate;
        clientName: string;
        accepted: string[];
        confirmed: boolean;
        disabled?: boolean;
    } = $props();

    let t = useI18n();
    let md = useMarkdownWorker();

    // built-in descriptions of the standard scopes for the `local` source
    let builtin: Record<string, string> = $derived(t.authorize.consent.scopes);
    let detailsScope: undefined | string = $state();
    let checks: Record<string, boolean> = $state({});

    // a new content, e.g. after it changed on the server, needs a new confirmation
    $effect(() => {
        checks = Object.fromEntries(
            consent.scopes.map(s => [
                s.name,
                consent.default_scopes.includes(s.name) || consent.granted_scopes.includes(s.name),
            ]),
        );
        confirmed = false;
        detailsScope = undefined;
    });

    $effect(() => {
        accepted = consent.scopes.map(s => s.name).filter(s => checks[s]);
    });

    function details(scope: ConsentScope): string | undefined {
        if (consent.source === 'remote') {
            return scope.description;
        }
        return builtin[scope.name];
    }

    function toggleDetails(scope: ConsentScope) {
        if (detailsScope === scope.name) {
            detailsScope = undefined;
            return;
        }
        detailsScope = scope.name;
        if (consent.source === 'remote' && scope.description) {
            md.sanitizeHTML(scope.description);
        }
    }
</script>

<div class="consent">
    <h3>{t.authorize.consent.title}</h3>
    <p>
        {t.authorize.consent.intro.replace('{{ CLIENT }}', consent.name || clientName)}
    </p>
    {#if consent.intro}
        <p class="intro">{consent.intro}</p>
    {/if}

    <ul>
        {#each consent.scopes as scope (scope.name)}
            {@const isDefault = consent.default_scopes.includes(scope.name)}
            {@const desc = details(scope)}
            <li>
                <div class="scope">
                    <InputCheckbox
                        ariaLabel={scope.title || scope.name}
                        bind:checked={
                            () => checks[scope.name] ?? false, v => (checks[scope.name] = v)
                        }
                        disabled={isDefault || disabled}
                    >
                        <span class:warning={scope.warning}>
                            {scope.title || scope.name}
                        </span>
                        {#if isDefault}
                            <span class="muted">({t.authorize.consent.required})</span>
                        {/if}
                    </InputCheckbox>
                    {#if desc}
                        <Button invisible onclick={() => toggleDetails(scope)}>
                            {t.authorize.consent.details}
                        </Button>
                    {/if}
                </div>
                {#if scope.warning}
                    <div class="warning">{t.authorize.consent.warning}</div>
                {/if}
                {#if detailsScope === scope.name && desc}
                    <div class="details">
                        {#if consent.source === 'remote'}
                            <!-- sanitized by the markdown worker -->
                            {@html md.sanitizedHTML()}
                        {:else}
                            {desc}
                        {/if}
                    </div>
                {/if}
            </li>
        {/each}
    </ul>

    {#if consent.terms_url}
        <a href={consent.terms_url} target="_blank" rel="noopener noreferrer">
            {t.authorize.consent.terms}
        </a>
    {/if}

    <div class="confirm">
        <InputCheckbox ariaLabel={t.authorize.consent.confirm} bind:checked={confirmed} {disabled}>
            {t.authorize.consent.confirm}
        </InputCheckbox>
    </div>
</div>

<style>
    .consent {
        margin: 1rem 0 0.5rem;
        padding: 0.5rem 0.75rem;
        border: 1px solid hsl(var(--bg-high));
        border-radius: var(--border-radius, 0.25rem);
    }

    h3 {
        margin: 0.25rem 0 0.5rem;
    }

    ul {
        margin: 0.5rem 0;
        padding: 0;
        list-style: none;
    }

    li {
        margin: 0.25rem 0;
    }

    .scope {
        display: flex;
        justify-content: space-between;
        align-items: center;
        gap: 0.5rem;
        --ui-button-padding: 0 0.25rem;
    }

    .details {
        margin: 0.25rem 0 0.5rem 1.9rem;
        font-size: 0.9rem;
        overflow-wrap: anywhere;
    }

    .intro {
        overflow-wrap: anywhere;
    }

    .muted {
        opacity: 0.7;
        font-size: 0.85rem;
    }

    .warning {
        color: hsl(var(--error));
    }

    .confirm {
        margin-top: 0.75rem;
    }
</style>
