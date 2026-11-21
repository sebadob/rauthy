<script lang="ts">
    import type { Snippet } from 'svelte';
    import Button from '#lib/button/Button.svelte';
    import IconTrash from '#icons/IconTrash.svelte';
    import { useI18n } from '#state/i18n.svelte.ts';

    let {
        name,
        badges,
        metadata,
        onDelete,
    }: {
        name: string;
        badges?: Snippet;
        metadata: { label: string; value: string }[];
        onDelete?: () => void;
    } = $props();

    const t = useI18n();
</script>

<article class="credential">
    <header>
        <h4>{name}</h4>
        {@render badges?.()}
    </header>
    <dl>
        {#each metadata as { label, value } (label)}
            <div>
                <dt>{label}</dt>
                <dd>{value}</dd>
            </div>
        {/each}
    </dl>
    {#if onDelete}
        <div class="actions">
            <Button level={-3} onclick={onDelete}>
                <IconTrash width="1rem" />
                {t.common.delete}
            </Button>
        </div>
    {/if}
</article>

<style>
    .credential {
        display: grid;
        grid-template-columns: minmax(0, 1fr) auto;
        align-items: start;
        gap: 0.5rem 1rem;
        padding: 0.75rem;
        border: 1px solid var(--ui-outline);
        border-radius: var(--border-radius);
        background: var(--ui-hover);
    }

    header {
        display: flex;
        align-items: center;
        flex-wrap: wrap;
        gap: 0.375rem;
    }

    h4 {
        font-size: var(--ui-control-font-size);
        font-weight: 600;
        overflow-wrap: anywhere;
    }

    dl {
        grid-row: 2;
        display: flex;
        flex-wrap: wrap;
        gap: 0.25rem 1.25rem;
        color: var(--ui-muted);
        font-size: 0.8125rem;
    }

    dd {
        color: var(--ui-text);
        font-variant-numeric: tabular-nums;
        overflow-wrap: anywhere;
    }

    .actions {
        grid-column: 2;
        grid-row: 1 / span 2;
        align-self: center;
    }

    @media (max-width: 480px) {
        dl {
            grid-column: 1 / -1;
        }

        .actions {
            grid-row: 1;
        }
    }
</style>
