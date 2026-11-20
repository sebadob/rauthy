<script lang="ts">
    import Popover from '#lib/Popover.svelte';
    import Button from '#lib/button/Button.svelte';
    import IconUserCircle from '#icons/IconUserCircle.svelte';
    import IconLogout from '#icons/IconLogout.svelte';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import { useI18n } from '#state/i18n.svelte.ts';
    import { redirectToLogout } from '#utils/helpers.ts';
    let { compact }: { compact: boolean } = $props();
    let ta = useI18nAdmin();
    let t = useI18n();
</script>
<div class="admin-account-menu">
    <Popover ariaLabel={ta.common.account} btnInvisible offsetTop="-148px" lazy>
        {#snippet button()}
            <IconUserCircle width="22px" />
            {#if !compact}<span>{ta.common.account}</span><svg
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="1.7"><path d="m7 14 5-5 5 5" /></svg
                >{/if}
        {/snippet}
        <div class="admin-account-popover">
            <a href="/auth/v1/account"><IconUserCircle width="20px" />{ta.common.account}</a>
            <Button invisible onclick={() => redirectToLogout()}
                ><IconLogout width="20px" />{t.account.navLogout}</Button
            >
        </div>
    </Popover>
</div>

<style>
    .admin-account-menu > :global(div) > :global(.admin-control) {
        width: 100%;
        min-height: 44px;
        background: var(--ui-surface);
        border-radius: var(--border-radius);
        padding: 0 12px;
    }
    .admin-account-menu > :global(div) > :global(.admin-control) > :global(.children) {
        width: 100%;
        justify-content: flex-start;
    }
    .admin-account-menu > :global(div) > :global(.admin-control) :global(span) {
        flex: 1;
        text-align: left;
        font-size: 13px;
    }
    .admin-account-popover {
        display: flex;
        flex-direction: column;
        width: 192px;
        padding: 6px;
        gap: 4px;
    }
    .admin-account-popover :global(a),
    .admin-account-popover :global(.admin-control) {
        display: flex;
        align-items: center;
        justify-content: flex-start;
        gap: 12px;
        height: 40px;
        padding: 0 12px;
        color: var(--ui-text);
        text-decoration: none;
        font-size: 13px;
        font-weight: 500;
        border-radius: var(--border-radius);
    }
    .admin-account-popover :global(a:hover) {
        background: var(--ui-hover);
    }
    .admin-account-popover :global(.admin-control) > :global(.children) {
        justify-content: flex-start;
    }

    .admin-account-popover :global(.admin-control[data-variant]) {
        width: 100%;
        justify-content: flex-start;
        color: var(--ui-text);
    }
    .admin-account-popover :global(.admin-control[data-variant]) > :global(.children) {
        width: 100%;
        justify-content: flex-start;
    }
</style>
