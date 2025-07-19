<script lang="ts">
    import type {Snippet} from "svelte";
    import {useI18n} from "$state/i18n.svelte";
    import Button from "$lib5/button/Button.svelte";
    import IconStop from "$icons/IconStop.svelte";

    let {
        showModal = $bindable(),
        closeModal = $bindable(),
        onClose,
        strict = false,
        prerender = false,
        children,
    }: {
        showModal?: boolean,
        closeModal?: () => void,
        onClose?: () => void,
        strict?: boolean,
        prerender?: boolean,
        children: Snippet,
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
    })

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
                })
                refLast.addEventListener('keydown', ev => {
                    if (ev.code === 'Tab' && !ev.shiftKey) {
                        ev.preventDefault();
                        refFirst?.focus();
                    }
                })

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
            if (elem.getAttribute("hidden") === 'true' || elem.getAttribute("disabled") === 'true') {
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
        data-strict={strict}
        onclose={() => {
            showModal = false;
            onClose?.();
        }}
        onclick={outsideClick}
>
    <div bind:this={refContent} role="none" onclick={stopPropagation}>
        {#if !strict}
            <div class="relative">
                <div class="absolute close">
                    <Button ariaLabel={t.common.close} invisible onclick={close}>
                        <span class="closeIcon">
                            <IconStop color="currentColor" width="1.2rem"/>
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

    dialog[data-strict="true"]::backdrop {
        background: hsla(var(--bg) / 0.8);
    }

    dialog[data-strict="false"]::backdrop {
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
        color: hsla(var(--text) / .5);
        transition: color 150ms;
    }

    .closeIcon:hover {
        color: hsl(var(--error));
    }

    @media (max-width: 600px) {
        dialog {
            max-height: calc(100dvh - .5rem);
            max-width: calc(100dvw - .5rem);
        }

        dialog > div {
            padding: .5rem;
        }

        .close {
            top: -.5rem;
            right: -.4rem;
            cursor: pointer;
        }
    }
</style>
