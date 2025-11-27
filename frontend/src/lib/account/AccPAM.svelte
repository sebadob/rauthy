<script lang="ts">
    import Button from '$lib5/button/Button.svelte';
    import { useI18n } from '$state/i18n.svelte.js';
    import { fetchGet, fetchPost } from '$api/fetch';
    import type {
        PamHostAccessResponse,
        PamHostDetailsResponse,
        PamPasswordResponse,
        PamUserResponse,
    } from '$api/types/pam';
    import LabeledValue from '$lib/LabeledValue.svelte';
    import InputPassword from '$lib/form/InputPassword.svelte';
    import { slide } from 'svelte/transition';
    import { formatDateFromTs } from '$utils/helpers';
    import { onMount } from 'svelte';
    import Tooltip from '$lib/Tooltip.svelte';
    import SearchBar from '$lib/search_bar/SearchBar.svelte';

    let {
        pamUser,
    }: {
        pamUser: PamUserResponse;
    } = $props();

    let t = useI18n();

    let err = $state('');

    let hosts: PamHostAccessResponse[] = $state([]);
    let hostsFiltered: PamHostAccessResponse[] = $state([]);
    let search = $state('');

    let password = $state('');
    let expTs: undefined | number = $state();
    let expSecs: undefined | number = $state();
    let interval: undefined | number;

    onMount(() => {
        fetchHosts();
    });

    $effect(() => {
        if (search.length > 0) {
            let lower = search.toLowerCase();
            hostsFiltered = hosts.filter(h => {
                if (h.hostname.toLowerCase().includes(lower)) {
                    return true;
                }
                for (let ip of h.ips) {
                    if (ip.includes(lower)) {
                        return true;
                    }
                }
                for (let alias of h.aliases) {
                    if (alias.toLowerCase().includes(lower)) {
                        return true;
                    }
                }
                return false;
            });
        } else {
            hostsFiltered = hosts;
        }
    });

    function calcExpSecs() {
        let secs = 0;
        if (expTs) {
            let ts = new Date().getTime() / 1000;
            secs = Math.floor(expTs - ts);
        }
        if (secs > 0) {
            expSecs = secs;
        } else {
            expSecs = undefined;
            expTs = undefined;
            password = '';
            clearInterval(interval);
            interval = undefined;
        }
    }

    async function fetchHosts() {
        let res = await fetchGet<PamHostAccessResponse[]>('/auth/v1/pam/hosts/user_access');
        if (res.body) {
            res.body.sort((a, b) => a.hostname.localeCompare(b.hostname));
            hosts = res.body;
        } else {
            err = res.error?.message || 'Error';
        }
    }

    async function generateNew() {
        let res = await fetchPost<PamPasswordResponse>('/auth/v1/pam/password');
        if (res.body) {
            password = res.body.password;
            expTs = res.body.exp;
            calcExpSecs();

            if (interval) {
                clearInterval(interval);
            }
            interval = setInterval(() => {
                calcExpSecs();
            }, 1000);
        } else {
            err = res.error?.message || 'Error';
        }
    }
</script>

<div class="container">
    <div class="info">
        <div>
            <LabeledValue label={t.account.pam.username}>
                {pamUser.name}
            </LabeledValue>
            <LabeledValue label="Shell">
                <code>{pamUser.shell}</code>
            </LabeledValue>
        </div>
        <div>
            <LabeledValue label="UID">
                {pamUser.id}
            </LabeledValue>
            <LabeledValue label="GID">
                {pamUser.gid}
            </LabeledValue>
        </div>
    </div>

    {#if password}
        <div transition:slide={{ duration: 150 }}>
            <InputPassword
                label={t.common.password}
                placeholder={t.common.password}
                value={password}
                disabled
                showCopy
            />
        </div>
    {/if}

    {#if expSecs}
        <div
            class="exp"
            transition:slide={{ duration: 150 }}
        >
            {t.account.pam.validFor.replace('{{ secs }}', expSecs.toString())}
        </div>
    {/if}

    {#if expSecs === undefined || expSecs < 30}
        <div
            class="btn"
            transition:slide={{ duration: 150 }}
        >
            <Button
                ariaLabel={t.account.pam.generatePassword}
                onclick={generateNew}
            >
                {t.account.pam.generatePassword}
            </Button>

            {#if err}
                <div class="err">
                    {err}
                </div>
            {/if}
        </div>
    {/if}

    {#snippet btnClip(value: string)}
        <Button
            invisible
            onclick={() => navigator.clipboard.writeText(value)}
        >
            <div
                title={value}
                class="clip"
            >
                {value}
            </div>
        </Button>
    {/snippet}

    {#snippet pamHost(host: PamHostAccessResponse)}
        <div class="host">
            <h4>
                {host.hostname}
            </h4>
            <div class="hostValue">
                <LabeledValue label="IPs">
                    <div class="items">
                        {#each host.ips as ip}
                            {@render btnClip(ip)}
                        {/each}
                    </div>
                </LabeledValue>
            </div>

            <div class="hostValue">
                <LabeledValue label="Aliases">
                    <div class="items">
                        {#each host.aliases as alias}
                            {@render btnClip(alias)}
                        {/each}
                    </div>
                </LabeledValue>
            </div>

            {#if host.notes}
                <hr />
                {host.notes}
            {/if}
        </div>
    {/snippet}

    {#if hosts.length > 0}
        <div class="hosts">
            <h3>Hosts</h3>

            <div class="search">
                <SearchBar bind:value={search} />
            </div>

            {#each hostsFiltered as host}
                {@render pamHost(host)}
            {/each}
        </div>
    {/if}
</div>

<style>
    h3 {
        font-size: 1.1rem;
    }

    h4 {
        margin: 0;
        font-size: 1rem;
    }

    .btn {
        margin-top: 1rem;
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }

    .clip {
        cursor: copy;
    }

    .exp {
        margin-bottom: 1rem;
        color: hsl(var(--action));
    }

    .host {
        margin-bottom: 1rem;
        padding: 0.25rem 0.5rem;
        background-color: hsla(var(--bg-high) / 0.25);
        border: 1px solid hsl(var(--bg-high));
        border-radius: var(--border-radius);
    }

    .hosts {
        margin-top: 1rem;
    }

    .hostValue {
        margin: -0.25rem 0;
    }

    .info {
        display: flex;
        flex-wrap: wrap;
    }

    .info > div {
        flex: 1;
    }

    .items {
        display: flex;
        align-items: center;
        column-gap: 0.5rem;
        row-gap: 0;
        flex-wrap: wrap;
    }

    .search {
        margin-bottom: 1rem;
    }
</style>
