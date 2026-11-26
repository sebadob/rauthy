<script lang="ts">
    import type { ConsentContentTemplate, ConsentScope } from '#api/templates/ConsentContent.ts';
    import { useI18n } from '#state/i18n.svelte.ts';
    import InputCheckbox from '#lib/form/InputCheckbox.svelte';
    import Button from '#lib/button/Button.svelte';
    import Modal from '#lib/Modal.svelte';

    let {
        consent,
        clientName,
        accepted = $bindable(),
        confirmed = $bindable(),
        choicesFor = $bindable(),
        disabled,
        notice,
    }: {
        consent: ConsentContentTemplate;
        clientName: string;
        accepted: string[];
        confirmed: boolean;
        // the content the choices in `accepted` and `confirmed` were made for
        choicesFor: undefined | ConsentContentTemplate;
        disabled?: boolean;
        // shown with this row instead of among the errors of the page, which sit far below it
        notice?: string;
    } = $props();

    let t = useI18n();

    // built-in descriptions of the standard scopes for the `local` source
    let builtin: Record<string, string> = $derived(t.authorize.consent.scopes);
    let detailsScope: undefined | string = $state();
    let showModal = $state(false);
    let closeModal: undefined | (() => void) = $state();
    let refLink: undefined | HTMLButtonElement = $state();

    // "I consent to the [transmission of my data]": the part in brackets opens the full content
    let label = $derived.by(() => {
        let m = /^(.*)\[(.+)\](.*)$/s.exec(t.authorize.consent.agree);
        return m
            ? { before: m[1], link: m[2], after: m[3], full: m[1] + m[2] + m[3] }
            : {
                  before: '',
                  link: t.authorize.consent.agree,
                  after: '',
                  full: t.authorize.consent.agree,
              };
    });

    // A new content, e.g. after a `consent_required` answer, needs a new confirmation. The choices
    // live with the page, so that they stay when this row is shown at another place of it.
    $effect(() => {
        if (choicesFor !== consent) {
            accepted = consent.scopes
                .map(s => s.name)
                .filter(
                    s => consent.default_scopes.includes(s) || consent.granted_scopes.includes(s),
                );
            confirmed = false;
            detailsScope = undefined;
            choicesFor = consent;
        }
    });

    function setScope(name: string, checked: boolean) {
        accepted = consent.scopes
            .map(s => s.name)
            .filter(s => (s === name ? checked : accepted.includes(s)));
    }

    // A remote description may contain HTML. Only its text is shown: an inert document from the
    // DOMParser neither runs scripts nor loads anything. The text of scripts and styles is dropped,
    // and a line break and every block end become a newline, which `pre-line` shows.
    function details(scope: ConsentScope): null | string {
        if (consent.source !== 'remote') {
            return builtin[scope.name] ?? null;
        }
        if (!scope.description) {
            return null;
        }
        let body = new DOMParser().parseFromString(scope.description, 'text/html').body;
        body.querySelectorAll('script, style, template').forEach(n => n.remove());
        body.querySelectorAll('br').forEach(n => n.replaceWith('\n'));
        body.querySelectorAll(
            'address, article, aside, blockquote, dd, div, dl, dt, figcaption, figure, footer, h1, h2, h3, h4, h5, h6, header, hr, li, main, nav, ol, p, pre, section, table, tr, ul',
        ).forEach(n => n.append('\n'));
        return body.textContent.replace(/\s*\n\s*/g, '\n').trim() || null;
    }

    function toggleDetails(scope: ConsentScope) {
        detailsScope = detailsScope === scope.name ? undefined : scope.name;
    }

    // a click on the link inside the label opens the content, it does not toggle the checkbox
    function openModal(ev: Event) {
        ev.preventDefault();
        ev.stopPropagation();
        showModal = true;
    }
</script>

{#if notice}
    <div class="notice">{notice}</div>
{/if}

<!-- the login waits for this box, so an unchecked one is marked as what's missing, not as an error
     message somewhere else on the page -->
<div class="agree" class:pending={!confirmed && !disabled}>
    <InputCheckbox
        ariaLabel={label.full}
        bind:checked={confirmed}
        {disabled}
        borderColor={confirmed || disabled ? undefined : 'hsl(var(--error))'}
    >
        {label.before}<button
            type="button"
            class="link"
            aria-haspopup="dialog"
            bind:this={refLink}
            onclick={openModal}>{label.link}</button
        >{label.after}
    </InputCheckbox>
</div>

<Modal
    bind:showModal
    bind:closeModal
    ariaLabel={t.authorize.consent.title}
    onClose={() => refLink?.focus()}
>
    <div class="consent">
        <h3>{t.authorize.consent.title}</h3>
        <!-- the client asks, `name` is the one of the consent source, i.e. of the resource -->
        <p>
            {(consent.name ? t.authorize.consent.introResource : t.authorize.consent.intro).replace(
                /\{\{ (CLIENT|RESOURCE) \}\}/g,
                (_, key) => (key === 'CLIENT' ? clientName : (consent.name ?? '')),
            )}
        </p>
        {#if consent.intro}
            <p class="intro">{consent.intro}</p>
        {/if}

        <ul>
            {#each consent.scopes as scope (scope.name)}
                {@const isDefault = consent.default_scopes.includes(scope.name)}
                {@const desc = details(scope)}
                <li>
                    <!-- a default is always included: only its checkbox is locked, its text stays -->
                    <div class="scope" class:locked={isDefault && !disabled}>
                        <InputCheckbox
                            ariaLabel={scope.title || scope.name}
                            bind:checked={
                                () => accepted.includes(scope.name), v => setScope(scope.name, v)
                            }
                            disabled={isDefault || disabled}
                        >
                            <span class:warning={scope.warning}>
                                {scope.title || scope.name}
                            </span>
                            {#if isDefault}
                                <span class="required">({t.authorize.consent.required})</span>
                            {/if}
                        </InputCheckbox>
                        {#if desc}
                            <span class="toggle">
                                <Button
                                    invisible
                                    ariaExpanded={detailsScope === scope.name}
                                    onclick={() => toggleDetails(scope)}
                                >
                                    {t.authorize.consent.details}
                                </Button>
                            </span>
                        {/if}
                    </div>
                    {#if scope.warning}
                        <div class="warning">{t.authorize.consent.warning}</div>
                    {/if}
                    {#if detailsScope === scope.name && desc}
                        <div class="details">{desc}</div>
                    {/if}
                </li>
            {/each}
        </ul>

        {#if consent.terms_url}
            <a href={consent.terms_url} target="_blank" rel="noopener noreferrer">
                {t.authorize.consent.terms}
            </a>
        {/if}

        <div class="done">
            <Button level={2} onclick={() => closeModal?.()}>{t.common.close}</Button>
        </div>
    </div>
</Modal>

<style>
    /* one fixed row, so that nothing around it moves when it shows */
    .agree {
        min-height: 1.75rem;
        margin: 0.75rem 0 0;
        font-size: 0.9rem;
    }

    .agree.pending {
        color: hsl(var(--error));
    }

    .notice {
        margin: 0.75rem 0 -0.25rem;
        color: hsl(var(--error));
        font-size: 0.85rem;
    }

    .link {
        display: inline;
        margin: 0;
        padding: 0;
        border: 0;
        background: none;
        color: inherit;
        font: inherit;
        text-decoration: underline;
        text-underline-offset: 0.15em;
        cursor: pointer;
    }

    .link:hover {
        color: hsl(var(--action));
    }

    .consent {
        width: min(28rem, calc(100dvw - 3rem));
    }

    h3 {
        margin: 0.25rem 2rem 0.5rem 0;
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
        white-space: pre-line;
    }

    .intro {
        overflow-wrap: anywhere;
    }

    .required {
        font-size: 0.85rem;
    }

    /* the shell fades a disabled checkbox with its label, a locked default fades its box only */
    :global(.ui-shell) .scope.locked :global(.admin-checkbox[data-disabled='true']) {
        opacity: 1;
        cursor: default;
    }

    :global(.ui-shell) .scope.locked :global(.admin-checkbox input:disabled) {
        opacity: 0.6;
    }

    /* no background that stays after a click, the focus ring only on keyboard focus */
    :global(.ui-shell) .scope .toggle :global(.admin-control[data-variant='invisible']) {
        background: transparent;
        color: var(--ui-muted);
    }

    @media (hover: hover) {
        :global(.ui-shell) .scope .toggle :global(.admin-control[data-variant='invisible']:hover) {
            color: var(--ui-primary);
        }
    }

    :global(.ui-shell) .scope .toggle :global(.admin-control:focus:not(:focus-visible)) {
        outline: none;
    }

    .warning {
        color: hsl(var(--error));
    }

    .done {
        display: flex;
        justify-content: flex-end;
        margin-top: 1rem;
    }
</style>
