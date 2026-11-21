<script lang="ts">
    import type { BackupListing, BackupListings } from '#api/types/backup.ts';
    import { fetchGet, fetchPost } from '#api/fetch.ts';
    import { onMount } from 'svelte';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import { formatDateFromTs, prettyFileSize } from '#utils/helpers.ts';
    import Button from '#lib/button/Button.svelte';
    import Section from '#lib/Section.svelte';
    import { useI18n } from '#state/i18n.svelte.ts';
    import IconCheck from '#icons/IconCheck.svelte';
    import IconArrowPathSquare from '#icons/IconArrowPathSquare.svelte';
    import A from '#lib/A.svelte';

    let ta = useI18nAdmin();
    const t = useI18n();

    let local: BackupListing[] = $state([]);
    let s3: BackupListing[] = $state([]);

    let isDisabled = $state(false);
    let success = $state(false);

    onMount(() => {
        fetchListings();
    });

    async function postCreateBackup() {
        let res = await fetchPost('/auth/v1/backup');
        if (res.error) {
            console.error(res.error);
        } else {
            success = true;

            // S3 pushes are done async in the background
            setTimeout(() => {
                fetchListings();
            }, 1000);

            setTimeout(() => {
                success = false;
            }, 5000);
        }
    }

    async function fetchListings() {
        let res = await fetchGet<BackupListings>('/auth/v1/backup');
        if (res.body) {
            res.body.local.sort((a, b) => b.last_modified - a.last_modified);
            res.body.s3.sort((a, b) => b.last_modified - a.last_modified);

            local = res.body.local;
            s3 = res.body.s3;
        } else if (res.error) {
            if (res.error.error === 'NotFound') {
                isDisabled = true;
            } else {
                console.error(res);
            }
        }
    }
</script>

{#snippet header()}
    <div class="header row">
        <div>{ta.backup.name}</div>
        <div class="right">{ta.backup.lastModified}</div>
        <div class="right">{ta.backup.size}</div>
    </div>
{/snippet}

{#snippet backup(backup: BackupListing, hrefPrefix: string)}
    <div class="row">
        <A href={`${hrefPrefix}/${backup.name}`}>
            <span class="font-mono">
                {backup.name}
            </span>
        </A>
        <div class="font-mono muted right modified">
            <span class="meta-label">{ta.backup.lastModified}</span>
            {formatDateFromTs(backup.last_modified)}
        </div>
        <div class="font-mono muted right">
            <span class="meta-label">{ta.backup.size}</span>
            {#if backup.size}
                {prettyFileSize(backup.size)}
            {:else}
                -
            {/if}
        </div>
    </div>
{/snippet}

<div class="page-header">
    <h2 class="ui-page-title">{ta.backup.title ?? 'Backups'}</h2>
    {#if !isDisabled}
        <div class="actions">
            {#if success}<IconCheck />{/if}
            <Button level={2} ariaLabel={t.common.refresh} onclick={fetchListings}>
                <IconArrowPathSquare width="1rem" />
                {t.common.refresh}
            </Button>
            <Button onclick={postCreateBackup}>{ta.backup.createBackup}</Button>
        </div>
    {/if}
</div>

{#if isDisabled}
    <p class="ui-muted-copy">{ta.backup.disabledDesc}</p>
{:else if local.length === 0 && s3.length === 0}
    <Section><p class="ui-muted-copy">{ta.common.noEntries}</p></Section>
{:else}
    <div class="ui-section-stack">
        {#each [{ title: ta.backup.local, entries: local, prefix: '/auth/v1/backup/local' }, { title: 'S3', entries: s3, prefix: '/auth/v1/backup/s3' }] as { title, entries, prefix } (prefix)}
            <Section {title}>
                {#if entries.length > 0}
                    <div class="backup-list">
                        {@render header()}
                        {#each entries as b (b.name)}
                            {@render backup(b, prefix)}
                        {/each}
                    </div>
                {:else}
                    <p class="ui-muted-copy">{ta.common.noEntries}</p>
                {/if}
            </Section>
        {/each}
    </div>
{/if}

<style>
    .page-header,
    .actions {
        display: flex;
        flex-wrap: wrap;
        align-items: center;
        gap: 0.5rem;
    }

    .page-header {
        justify-content: space-between;
        margin-bottom: var(--ui-space);
    }

    .page-header h2 {
        margin: 0;
    }

    .header {
        font-weight: bold;
    }

    .muted {
        color: hsla(var(--text) / 0.7);
    }

    .right {
        text-align: right;
    }

    .backup-list {
        container-type: inline-size;
    }

    .row {
        display: grid;
        grid-template-columns: minmax(0, 2fr) minmax(10rem, 1fr) minmax(4.5rem, 0.6fr);
        gap: 0.75rem;
        align-items: baseline;
        padding-block: 0.5rem;
    }

    .row:not(.header) {
        border-top: 1px solid var(--ui-outline);
    }

    .row > :global(a) {
        min-width: 0;
        overflow-wrap: anywhere;
    }

    .meta-label {
        display: none;
    }

    @container (width < 35rem) {
        .row {
            grid-template-columns: minmax(0, 1fr) auto;
            row-gap: 0.25rem;
        }

        .row > :global(a) {
            grid-column: 1 / -1;
        }

        .header {
            display: none;
        }

        .modified {
            text-align: start;
        }

        .meta-label {
            display: block;
            font-family: system-ui, sans-serif;
            font-size: 0.75rem;
        }
    }
</style>
