<script lang="ts">
    import { onMount } from 'svelte';
    import OrderSearchBar from '#lib/search_bar/OrderSearchBar.svelte';
    import Button from '#lib/button/Button.svelte';
    import Input from '#lib/form/Input.svelte';
    import { formatDateFromTs } from '#utils/helpers.js';
    import { useI18n } from '#state/i18n.svelte.ts';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import InputDateTimeCombo from '#lib/form/InputDateTimeCombo.svelte';
    import { fmtDateInput, fmtTimeInput, unixTsFromLocalDateTime } from '#utils/form.ts';
    import EntityDelete from '#lib/admin/EntityDelete.svelte';
    import { fetchGet, fetchPost } from '#api/fetch.ts';
    import ContentAdmin from '#lib/ContentAdmin.svelte';
    import type {
        BlacklistedIp,
        BlacklistResponse,
        IpBlacklistRequest,
    } from '#api/types/blacklist.ts';
    import ButtonAddModal from '#lib/button/ButtonAddModal.svelte';
    import Form from '#lib/form/Form.svelte';
    import Tooltip from '#lib/Tooltip.svelte';
    import Pagination from '#lib/pagination/Pagination.svelte';
    import { useSession } from '#state/session.svelte.ts';

    let t = useI18n();
    let ta = useI18nAdmin();

    // group admins get a read-only blacklist view for debugging; adding / removing entries
    // is full-admin only
    let session = useSession('admin');
    let isAdmin = $derived(session.isAdmin());

    const blacklistThreshold = 30;

    let refIp: undefined | HTMLInputElement = $state();
    let closeModal: undefined | (() => void) = $state();

    let err = $state('');
    let errSave = $state('');
    let blacklist: BlacklistedIp[] = $state([]);
    let blacklistFiltered: BlacklistedIp[] = $state([]);
    let blacklistPaginated: BlacklistedIp[] = $state([]);

    let ip = $state('');
    let expDate = $state(fmtDateInput());
    let expTime = $state(fmtTimeInput());

    let searchValue = $state('');
    let orderOptions = ['IP'];

    onMount(() => {
        fetchBlacklist();
    });

    $effect(() => {
        if (refIp) {
            const expiry = new Date(Date.now() + 60 * 60_000);
            expDate = fmtDateInput(expiry);
            expTime = fmtTimeInput(expiry);
            errSave = '';
            refIp.focus();
        }
    });

    $effect(() => {
        let search = searchValue.toLowerCase();
        if (!search) {
            blacklistFiltered = blacklist;
        } else {
            blacklistFiltered = blacklist.filter(e => e.ip?.includes(search));
        }
    });

    async function fetchBlacklist() {
        let res = await fetchGet<BlacklistResponse>('/auth/v1/blacklist');
        if (res.body) {
            blacklist = res.body.ips;
        } else {
            err = res.error?.message || 'Error';
        }
    }

    async function onSubmit(form: HTMLFormElement, params: URLSearchParams) {
        errSave = '';

        const exp = unixTsFromLocalDateTime(expDate, expTime);
        if (!exp || exp <= Date.now() / 1000) {
            errSave = t.common.invalidInput;
            return;
        }

        let payload: IpBlacklistRequest = {
            ip,
            exp,
        };

        // This is the only place we wrap the fetch in try, because we don't do validation
        // on the client side. A regex matching IPv4 + IPv6 is insanely huge.
        try {
            let res = await fetchPost(form.action, payload);
            if (res.error) {
                errSave = res.error.message;
            } else {
                ip = '';
                closeModal?.();
                await fetchBlacklist();
            }
        } catch (err) {
            errSave = 'Invalid IP Address';
        }
    }

    function onChangeOrder(option: string, direction: 'up' | 'down') {
        let up = direction === 'up';
        if (option === orderOptions[0]) {
            blacklist.sort((a, b) => (up ? a.ip.localeCompare(b.ip) : b.ip.localeCompare(a.ip)));
        }
    }
</script>

{err}

<ContentAdmin wide>
    <h2 class="admin-page-title">{ta.nav.blacklist}</h2>
    <div class="admin-page-toolbar">
        <OrderSearchBar
            bind:value={searchValue}
            {orderOptions}
            {onChangeOrder}
            searchWidth="100%"
            inline
        >
            {#snippet actions()}
                {#if isAdmin}
                    <ButtonAddModal level={1} bind:closeModal>
                        <div class="addNew">
                            <h3>{ta.common.addNew}</h3>
                            <Form action="/auth/v1/blacklist" {onSubmit}>
                                <Input
                                    bind:ref={refIp}
                                    bind:value={ip}
                                    autocomplete="off"
                                    label="IP"
                                    placeholder="IPv4"
                                    required
                                    maxLength={40}
                                />
                                <InputDateTimeCombo
                                    label={ta.options.expires}
                                    labelTime={ta.common.time}
                                    bind:value={expDate}
                                    bind:timeValue={expTime}
                                    min={fmtDateInput()}
                                    required
                                    withTime
                                />

                                <Button type="submit">
                                    {t.common.save}
                                </Button>
                                {#if errSave}<p class="err" role="alert">{errSave}</p>{/if}
                            </Form>
                        </div>
                    </ButtonAddModal>
                {/if}
            {/snippet}
        </OrderSearchBar>
    </div>

    <div id="blacklist">
        {#if blacklist.length === 0}
            <div>
                {ta.common.noEntries}
            </div>
        {:else}
            {#snippet row(entry: BlacklistedIp)}
                <div class="blacklisted">
                    <div>
                        <Button invisible onclick={() => navigator.clipboard.writeText(entry.ip)}>
                            <div class="ip">
                                <Tooltip text={t.common.copyToClip}>
                                    {entry.ip}
                                </Tooltip>
                            </div>
                        </Button>
                    </div>

                    <div class="date">
                        {formatDateFromTs(entry.exp)}
                    </div>
                    {#if isAdmin}
                        <div class="blacklist-action">
                            <EntityDelete
                                url={`/auth/v1/blacklist/${entry.ip}`}
                                name={entry.ip}
                                ariaLabel={`${t.common.delete}: ${entry.ip}`}
                                iconOnly
                                onDelete={fetchBlacklist}
                            />
                        </div>
                    {/if}
                </div>
            {/snippet}

            <div class="blacklist">
                {#if blacklist.length > blacklistThreshold}
                    {#each blacklistPaginated as entry (entry.ip)}
                        {@render row(entry)}
                    {/each}
                {:else}
                    {#each blacklistFiltered as entry (entry.ip)}
                        {@render row(entry)}
                    {/each}
                {/if}
            </div>
        {/if}
    </div>

    {#if blacklist.length > blacklistThreshold}
        <Pagination
            bind:items={blacklistFiltered}
            bind:itemsPaginated={blacklistPaginated}
            pageSize={30}
        />
    {/if}
</ContentAdmin>

<style>
    .addNew {
        width: min(25rem, calc(100dvw - 96px));
    }

    .addNew h3 {
        margin: 0 0 1rem;
        font-size: 1.25rem;
    }

    .addNew :global(button[type='submit']) {
        margin-top: 1rem;
    }

    .blacklist {
        display: grid;
        gap: 0.75rem;
    }

    .blacklisted {
        display: grid;
        grid-template-columns: minmax(0, 1fr) auto auto;
        align-items: center;
        gap: 0.5rem 1rem;
        padding: 0.5rem 1rem;
        border: 1px solid var(--ui-outline);
        border-radius: var(--border-radius);
        background: var(--ui-surface);
    }

    .date {
        font-size: 0.8125rem;
        color: var(--ui-muted);
    }

    .ip {
        overflow-wrap: anywhere;
        text-align: start;
        cursor: copy;
    }

    .blacklisted :global(button) {
        --ui-button-padding: 0 0.375rem;
    }

    @media (max-width: 520px) {
        .blacklisted {
            grid-template-columns: minmax(0, 1fr) auto;
            padding-inline: 0.625rem;
        }
        .date {
            grid-column: 1;
            grid-row: 2;
            padding-inline: 0.375rem;
        }
        .blacklist-action {
            grid-column: 2;
            grid-row: 1 / span 2;
        }
    }
</style>
