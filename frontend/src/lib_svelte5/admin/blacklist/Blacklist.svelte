<script lang="ts">
    import {onMount} from "svelte";
    import OrderSearchBar from "$lib5/search_bar/OrderSearchBar.svelte";
    import Button from "$lib5/button/Button.svelte";
    import Input from "$lib5/form/Input.svelte";
    import {formatDateFromTs, formatUtcTsFromDateInput} from "$utils/helpers.js";
    import {useI18n} from "$state/i18n.svelte.ts";
    import {useI18nAdmin} from "$state/i18n_admin.svelte.ts";
    import InputDateTimeCombo from "$lib5/form/InputDateTimeCombo.svelte";
    import {fmtDateInput, fmtTimeInput} from "$utils/form.ts";
    import IconStop from "$icons/IconStop.svelte";
    import {fetchDelete, fetchGet, fetchPost} from "$api/fetch.ts";
    import ContentAdmin from "$lib5/ContentAdmin.svelte";
    import type {BlacklistedIp, BlacklistResponse, IpBlacklistRequest} from "$api/types/blacklist.ts";
    import ButtonAddModal from "$lib5/button/ButtonAddModal.svelte";
    import {PATTERN_IPV4} from "$utils/patterns.ts";
    import Form from "$lib5/form/Form.svelte";
    import Tooltip from "$lib5/Tooltip.svelte";
    import Pagination from "$lib5/Pagination.svelte";

    let t = useI18n();
    let ta = useI18nAdmin();

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
        refIp?.focus();
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

        let exp = formatUtcTsFromDateInput(expDate, expTime);
        if (!exp) {
            errSave = 'Invalid Date Input: User Expires';
            return;
        }

        let payload: IpBlacklistRequest = {
            ip,
            exp,
        };
        let res = await fetchPost(form.action, payload);
        if (res.error) {
            errSave = res.error.message;
        } else {
            ip = '';
            closeModal?.();
            await fetchBlacklist();
        }
    }

    async function deleteIp(ip: string) {
        let res = await fetchDelete(`/auth/v1/blacklist/${ip}`);
        if (res.error) {
            err = res.error.message;
        } else {
            await fetchBlacklist();
        }
    }

    function onChangeOrder(option: string, direction: 'up' | 'down') {
        let up = direction === 'up';
        if (option === orderOptions[0]) {
            blacklist.sort((a, b) => up ? a.ip.localeCompare(b.ip) : b.ip.localeCompare(a.ip));
        }
    }
</script>

{err}

<ContentAdmin>
    <div class="top">
        <OrderSearchBar
                bind:value={searchValue}
                {orderOptions}
                {onChangeOrder}
                searchWidth="min(15.75rem, calc(100dvw - 1rem))"
        />

        <ButtonAddModal level={1} bind:closeModal>
            <div class="addNew">
                <Form action="/auth/v1/blacklist" {onSubmit}>
                    <Input
                            bind:ref={refIp}
                            bind:value={ip}
                            autocomplete="off"
                            label="IP"
                            placeholder="IPv4"
                            errMsg="IPv4"
                            required
                            maxLength={15}
                            pattern={PATTERN_IPV4}
                    />
                    <InputDateTimeCombo
                            label="Expiry"
                            bind:value={expDate}
                            bind:timeValue={expTime}
                            min={fmtDateInput()}
                            withTime
                    />

                    <div style:height=".5rem"></div>
                    <Button type="submit">
                        {t.common.save}
                    </Button>
                    <div class="err">
                        {errSave}
                    </div>
                </Form>
            </div>
        </ButtonAddModal>
    </div>

    <div id="blacklist">
        {#if blacklist.length === 0}
            <div>
                {ta.common.noEntries}
            </div>
        {:else}
            {#snippet row(entry: BlacklistedIp)}
                <div class="blacklisted">
                    <Button invisible onclick={() => navigator.clipboard.writeText(entry.ip)}>
                        <div class="ip">
                            <Tooltip text={t.common.copyToClip}>
                                {entry.ip}
                            </Tooltip>
                        </div>
                    </Button>
                    <div class="date">
                        {formatDateFromTs(entry.exp)}
                    </div>
                    <Button invisible onclick={() => deleteIp(entry.ip)}>
                        <Tooltip text={t.common.delete}>
                            <div class="delete">
                                <IconStop width="1.25rem"/>
                            </div>
                        </Tooltip>
                    </Button>
                </div>
            {/snippet}

            {#if blacklist.length > blacklistThreshold}
                {#each blacklistPaginated as entry (entry.ip)}
                    {@render row(entry)}
                {/each}
            {:else}
                {#each blacklistFiltered as entry (entry.ip)}
                    {@render row(entry)}
                {/each}
            {/if}
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
        min-height: 12rem;
    }

    .blacklisted {
        padding-left: .5rem;
        max-width: 22.5rem;
        margin: .25rem 0;
        display: grid;
        grid-template-columns: 10rem 10rem 1.5rem;
        border-radius: var(--border-radius);
    }

    .blacklisted, .blacklisted div {
        transition: all 150ms;
    }

    .blacklisted:nth-of-type(2n + 1) {
        background: hsla(var(--bg-high) / .3);
    }

    .blacklisted:hover, .blacklisted:hover div {
        color: hsl(var(--bg));
        background: hsl(var(--accent));
    }

    .date {
        color: hsla(var(--text) / .8);
    }

    .delete {
        height: 1.25rem;
        overflow: clip;
    }

    .ip {
        text-align: left;
        cursor: copy;
    }

    .top {
        margin-bottom: 1rem;
        display: flex;
        flex-wrap: wrap;
        gap: .5rem 1rem;
    }
</style>
