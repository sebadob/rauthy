<script lang="ts">
    import type { Snippet } from 'svelte';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import IconEntry from '#icons/IconDocText.svelte';
    let ta = useI18nAdmin();

    let {
        children,
        wide = false,
    }: {
        children: Snippet;
        wide?: boolean;
    } = $props();
</script>

<div class="content admin-detail" class:wide>
    <div class="admin-empty">
        <div class="admin-empty-symbol" aria-hidden="true"><IconEntry width="2rem" /></div>
        <h2>{ta.common.selectEntry}</h2>
        <p>{ta.common.selectEntryHint}</p>
    </div>
    {@render children()}
</div>

<style>
    .content {
        flex: 1;
        padding: 2.2rem 1rem 0 1rem;
        overflow: auto;
    }

    @media (max-width: 800px) {
        .content {
            padding: 1.8rem 0.25rem 0 0.25rem;
        }
    }

    .admin-detail {
        --admin-detail-padding-bottom: 32px;
        min-width: 0;
        padding: 32px clamp(20px, 3vw, 48px) var(--admin-detail-padding-bottom);
        background: transparent;
    }
    .admin-detail:not(.wide) > :global(div[id]) {
        max-width: 960px;
        margin-inline: auto;
    }
    .admin-empty {
        display: none;
    }
    @media (width >= 800px) {
        :global(.admin-workspace)
            > :global(.content:has(> .navSub))
            .admin-detail:has(:global(> div[id]:empty)):not(:has(:global(> .err))) {
            display: grid;
            place-content: center;
        }
        :global(.admin-workspace)
            > :global(.content:has(> .navSub))
            .admin-detail:has(:global(> div[id]:empty)):not(:has(:global(> .err)))
            .admin-empty {
            display: flex;
            flex-direction: column;
            align-items: center;
            max-width: 280px;
            text-align: center;
            padding-bottom: 60px;
        }
    }
    .admin-empty-symbol {
        display: grid;
        place-items: center;
        width: 80px;
        height: 80px;
        border-radius: var(--border-radius);
        background: var(--ui-container);
        margin-bottom: 24px;
    }
    .admin-empty :global(h2) {
        font-size: 20px;
        margin-bottom: 8px;
    }
    .admin-empty :global(p) {
        font-size: 14px;
        line-height: 22px;
    }
    .admin-detail :global(form) {
        max-width: 100%;
    }
    .admin-detail :global(p) {
        font-size: 13px;
        line-height: 22px;
    }
    .admin-detail :global(h5) {
        margin: 24px 0 16px;
        font-size: 16px;
        font-weight: 600;
    }
    @media (max-width: 800px) {
        .admin-detail {
            --admin-detail-padding-bottom: 24px;
            padding: 48px 16px var(--admin-detail-padding-bottom);
        }
    }

    @media (width < 800px) {
        :global(.navSub[data-detail='false']) + .admin-detail {
            display: none;
        }

        :global(.navSub[data-detail='true']) + .admin-detail {
            padding-top: 12px;
        }
    }
</style>
