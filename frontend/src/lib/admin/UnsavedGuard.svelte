<script lang="ts">
    import { afterNavigate, beforeNavigate, goto } from '$app/navigation';
    import Button from '#lib/button/Button.svelte';
    import Modal from '#lib/Modal.svelte';
    import { useI18n } from '#state/i18n.svelte.ts';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import { hasUnsavedChanges, type UnsavedField } from '#utils/unsaved.ts';

    let {
        fields,
        guard = $bindable(run => run()),
        isSameTarget = () => false,
        onDiscard,
    }: {
        // every form value the guard watches, each next to its persisted value
        fields: UnsavedField[];
        // wraps a state change that is not a navigation, i.e. a tab switch
        guard?: (run: () => void) => void;
        // true when the target keeps the current form mounted, e.g. the very same record
        isSameTarget?: (to: URL) => boolean;
        // reset the form to its persisted state
        onDiscard: () => void | Promise<void>;
    } = $props();

    const t = useI18n();
    const ta = useI18nAdmin();

    let dirty = $derived(hasUnsavedChanges(fields));
    let show = $state(false);
    let closeModal: undefined | (() => void) = $state();
    // the navigation we re-issue ourselves must not be blocked by our own hook again
    let bypass = false;
    let continueTo: undefined | (() => void);

    beforeNavigate(nav => {
        if (bypass || !dirty) {
            return;
        }
        const to = nav.to;
        if (to && isSameTarget(to.url)) {
            return;
        }
        nav.cancel();
        // 'leave' unloads the document: cancel() already pops the browser's own prompt
        if (nav.type === 'leave' || !to) {
            return;
        }
        continueTo = () => {
            bypass = true;
            void goto(to.url);
        };
        show = true;
    });

    afterNavigate(() => (bypass = false));

    $effect(() => {
        guard = run => {
            if (!dirty) {
                run();
                return;
            }
            continueTo = run;
            show = true;
        };
    });

    async function discard() {
        closeModal?.();
        await onDiscard();
        continueTo?.();
        continueTo = undefined;
    }
</script>

<Modal bind:showModal={show} bind:closeModal ariaLabel={ta.common.unsavedChanges} strict>
    <div class="unsaved">
        <h3>{ta.common.unsavedChanges}</h3>
        <p>{ta.common.unsavedChangesDesc}</p>
        <div class="actions">
            <Button level={2} onclick={() => closeModal?.()}>
                {t.common.cancel}
            </Button>
            <Button level={-1} onclick={discard}>
                {ta.common.discard}
            </Button>
        </div>
    </div>
</Modal>

<style>
    .unsaved {
        width: min(24rem, calc(100dvw - 4rem));
        overflow-wrap: anywhere;
    }
    .unsaved h3 {
        margin-top: 0;
    }
    .actions {
        display: flex;
        justify-content: end;
        gap: 0.5rem;
        margin-top: 1.25rem;
    }
</style>
