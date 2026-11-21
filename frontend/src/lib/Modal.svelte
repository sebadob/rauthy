<script lang="ts">
    import type { Snippet } from 'svelte';
    import { useI18n } from '#state/i18n.svelte.ts';
    import Button from '#lib/button/Button.svelte';
    import IconStop from '#icons/IconStop.svelte';

    let {
        showModal = $bindable(),
        closeModal = $bindable(),
        onClose,
        strict = false,
        expanded = $bindable(),
        prerender = false,
        ariaLabel,
        children,
    }: {
        showModal?: boolean;
        closeModal?: () => void;
        onClose?: () => void;
        strict?: boolean;
        expanded?: boolean;
        prerender?: boolean;
        ariaLabel?: string;
        children: Snippet;
    } = $props();

    let t = useI18n();

    let refDialog: undefined | HTMLDialogElement;
    let refContent: undefined | HTMLDivElement = $state();
    let refFirst: undefined | HTMLElement = $state();
    let refLast: undefined | HTMLElement = $state();

    $effect(() => {
        closeModal = close;
    });

    $effect(() => {
        if (showModal) {
            refDialog?.showModal();
        }
    });

    $effect(() => {
        if (refContent && (showModal || prerender)) {
            let elems = findFocusable(refContent.children);

            if (elems.length > 0) {
                refFirst = elems[0];
                refLast = elems[elems.length - 1];

                refFirst.addEventListener('keydown', ev => {
                    if (ev.code === 'Tab' && ev.shiftKey) {
                        ev.preventDefault();
                        refLast?.focus();
                    }
                });
                refLast.addEventListener('keydown', ev => {
                    if (ev.code === 'Tab' && !ev.shiftKey) {
                        ev.preventDefault();
                        refFirst?.focus();
                    }
                });

                refFirst.focus();
            }
        } else {
            refFirst = undefined;
            refLast = undefined;
        }
    });

    function findFocusable(elems: HTMLCollection): HTMLElement[] {
        let res: HTMLElement[] = [];

        for (let elem of elems) {
            if (
                elem.getAttribute('hidden') === 'true' ||
                elem.getAttribute('disabled') === 'true'
            ) {
                // noop
            } else if (elem.getAttribute('tabindex') !== null) {
                res.push(elem as HTMLElement);
            } else if (['A', 'AREA'].includes(elem.tagName) && elem.getAttribute('href') !== null) {
                res.push(elem as HTMLElement);
            } else if (['BUTTON', 'IFRAME', 'INPUT', 'SELECT', 'TEXTAREA'].includes(elem.tagName)) {
                res.push(elem as HTMLElement);
            }

            if (elem.children) {
                let els = findFocusable(elem.children);
                if (els.length > 0) {
                    res.push(...els);
                }
            }
        }

        return res;
    }

    function close(ev?: Event) {
        ev?.preventDefault();
        refDialog?.close();
        showModal = false;
    }

    function outsideClick() {
        if (!strict) {
            close();
        }
    }

    function stopPropagation(ev: Event) {
        ev.stopPropagation();
    }
</script>

<!--
According to MDN docs, a dialog element must not have a tabindex.
We need the onclick listener here to make easy click-outside work.
-->
<!--svelte-ignore a11y_click_events_have_key_events-->
<!--svelte-ignore a11y_no_noninteractive_element_interactions-->
<dialog
    bind:this={refDialog}
    aria-modal="true"
    aria-label={ariaLabel}
    data-strict={strict}
    class:expandable={expanded !== undefined}
    class:expanded
    onclose={() => {
        showModal = false;
        onClose?.();
    }}
    onclick={outsideClick}
>
    <div bind:this={refContent} role="none" onclick={stopPropagation}>
        {#if !strict}
            <div class="relative close-anchor">
                <div class="absolute close corner-control">
                    <Button ariaLabel={t.common.close} invisible onclick={close}>
                        <span class="closeIcon">
                            <IconStop color="currentColor" width="1.2rem" />
                        </span>
                    </Button>
                </div>
            </div>
        {/if}
        <!--
        Just make sure that whatever we have in here will be loaded / fetched lazy.
        There is no need to load resources like images if the dialog is closed anyway.
        If `renderUpfront === true`, this will render and possibly fetch data upfront,
        even when the dialog is still hidden. May make sense in some cases.
        -->
        {#if prerender || showModal}
            {@render children()}
            {#if expanded !== undefined}
                <div class="expand corner-control" title={t.common.expandContent}>
                    <Button
                        ariaLabel={t.common.expandContent}
                        ariaPressed={expanded}
                        invisible
                        onclick={() => (expanded = !expanded)}
                    >
                        <svg
                            aria-hidden="true"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                        >
                            {#if expanded}
                                <path d="M6 10h4V6m8 4h-4V6M6 14h4v4m8-4h-4v4" />
                            {:else}
                                <path d="M10 6H6v4m8-4h4v4M6 14v4h4m8-4v4h-4" />
                            {/if}
                        </svg>
                    </Button>
                </div>
            {/if}
        {/if}
    </div>
</dialog>

<style>
    dialog {
        margin: auto;
        padding: 0;
        border: 1px solid hsl(var(--bg-high));
        border-radius: var(--border-radius);
        color: hsl(var(--text));
        background: hsl(var(--bg));
    }

    dialog[data-strict='true']::backdrop {
        background: hsla(var(--bg) / 0.8);
    }

    dialog[data-strict='false']::backdrop {
        background: hsla(var(--bg) / 0.2);
    }

    dialog > div {
        max-width: 100dvw;
        max-height: 100dvh;
        padding: 1rem;
        box-shadow: 0 15px 15px black;
        overflow: auto;
    }

    dialog[open] {
        animation: var(--animate-zoom);
    }

    .close {
        top: -1.1rem;
        right: -1rem;
        cursor: pointer;
    }

    .closeIcon {
        color: hsla(var(--text) / 0.5);
        transition: color 150ms;
    }

    .closeIcon:hover {
        color: hsl(var(--error));
    }

    @media (max-width: 600px) {
        dialog {
            max-height: calc(100dvh - 0.5rem);
            max-width: calc(100dvw - 0.5rem);
        }

        dialog > div {
            padding: 0.5rem;
        }

        .close {
            top: -0.5rem;
            right: -0.4rem;
            cursor: pointer;
        }
    }

    @media (max-width: 800px) {
        :global(.admin-shell) dialog > :global(div) {
            padding: 24px;
        }
    }
    :global(.admin-shell) dialog :global(.modal) {
        max-width: 100%;
        width: min(640px, calc(100dvw - 96px));
    }
    :global(.admin-shell) dialog:not(:global(.branding-preview *)) {
        border: 1px solid var(--ui-outline);
        border-radius: var(--border-radius);
        padding: 0;
        background: var(--ui-surface);
        max-width: calc(100dvw - 32px);
        box-shadow: 0 18px 60px #0003;
    }
    :global(.admin-shell) dialog:not(:global(.branding-preview *)) > :global(div) {
        padding: var(--dialog-padding, 32px);
        box-shadow: none;
    }
    :global(.admin-shell) dialog:not(:global(.branding-preview *))::backdrop {
        background: #0006;
    }
    :global(.admin-shell) dialog:not(.expandable, :global(.branding-preview *)) .close {
        top: max(-20px, calc(4px - var(--dialog-padding, 32px)));
        right: max(-20px, calc(4px - var(--dialog-padding, 32px)));
    }

    :global(.admin-shell) dialog {
        max-height: calc(100dvh - 32px);
        overflow: hidden;
    }
    :global(.admin-shell) dialog > :global(div) {
        max-height: calc(100dvh - 34px);
        overflow: auto;
    }

    :global(.admin-shell) dialog.expanded {
        max-width: 100dvw;
        max-height: 100dvh;
        border: 0;
        border-radius: 0;
    }

    :global(.admin-shell) dialog.expanded > div {
        max-height: 100dvh;
    }

    dialog.expandable {
        --modal-corner-size: 40px;
        --modal-corner-inset: clamp(6px, 1vw, 10px);
    }

    .expandable .close-anchor {
        position: static;
    }

    .expandable .corner-control {
        position: absolute;
        inset: auto;
        inset-inline-end: var(--modal-corner-inset);
        z-index: 1;
    }

    .expandable .close {
        inset-block-start: var(--modal-corner-inset);
    }

    .expandable .expand {
        inset-block-end: var(--modal-corner-inset);
    }

    .expandable .corner-control :global(button) {
        --ui-button-height: var(--modal-corner-size);
        --ui-button-min-height: var(--modal-corner-size);
        width: var(--modal-corner-size);
    }

    .expandable .corner-control :global(svg) {
        width: 22px;
        height: 22px;
    }

    .expandable .closeIcon {
        color: inherit;
    }

    .expandable .close:hover .closeIcon {
        color: hsl(var(--error));
    }

    .expand :global(button[aria-pressed='true']) {
        background: var(--ui-tonal);
        color: var(--ui-primary);
    }

    :global(.admin-shell) dialog :global(.meta) {
        max-width: 100%;
    }
    :global(.admin-shell) dialog :global(form) {
        max-width: 100%;
        min-width: 0;
    }
</style>
