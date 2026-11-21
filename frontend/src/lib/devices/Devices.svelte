<script lang="ts">
    import { useI18n } from '#state/i18n.svelte.js';
    import { fetchGet } from '#api/fetch.ts';
    import type { DeviceResponse } from '#api/types/device.ts';
    import Device from './Device.svelte';
    import Loading from '#lib/Loading.svelte';
    import Button from '#lib/button/Button.svelte';
    import IconArrowPathSquare from '#icons/IconArrowPathSquare.svelte';
    import { I18nEn } from '../../i18n/common/en';

    let {
        userId,
    }: {
        userId: string;
    } = $props();

    let t = useI18n();

    let devices: DeviceResponse[] | undefined = $state();
    let error = $state('');
    let refresh = $state(0);

    $effect(() => {
        void refresh;
        const url = `/auth/v1/users/${userId}/devices`;
        let current = true;
        devices = undefined;
        error = '';
        fetchGet<DeviceResponse[]>(url).then(res => {
            if (!current) return;
            devices = res.body;
            if (!res.body) error = res.error?.message || 'Error';
        });
        return () => {
            current = false;
        };
    });
</script>

<div class="devices" aria-busy={!devices && !error}>
    <div class="head">
        <p>{t.account.devicesDesc}</p>
        <Button ariaLabel={t.common.refresh} level={2} onclick={() => (refresh += 1)}>
            <IconArrowPathSquare width="1rem" />
            {t.common.refresh}
        </Button>
    </div>
    {#if error}
        <p class="err" role="alert">{error}</p>
    {:else if !devices}
        <Loading />
    {:else if devices.length === 0}
        <p>{t.account.devicesEmpty ?? I18nEn.account.devicesEmpty}</p>
    {:else}
        {#each devices as device, i (device.id)}
            <Device bind:device={devices[i]} {userId} />
        {/each}
    {/if}
</div>

<style>
    p {
        color: var(--ui-muted);
        font-size: var(--ui-control-font-size);
    }

    .head {
        display: flex;
        align-items: center;
        justify-content: space-between;
        flex-wrap: wrap;
        gap: 0.5rem;
    }

    .devices :global(.admin-expandable) {
        border: 1px solid var(--ui-outline);
        border-radius: var(--border-radius);
        background: var(--ui-surface);
    }

    .devices :global(.admin-expandable > .header) {
        min-height: 3rem;
        padding-inline: 0.5rem;
    }

    .devices :global(.admin-expandable[aria-expanded='true'] > .details) {
        padding: 1rem;
        border-top: 1px solid var(--ui-outline);
    }

    .devices {
        display: grid;
        gap: 0.75rem;
        min-width: 0;
    }
</style>
