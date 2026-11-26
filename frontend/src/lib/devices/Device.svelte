<script lang="ts">
    import type { DeviceRequest, DeviceResponse } from '#api/types/device.ts';
    import { formatDateFromTs } from '#utils/helpers.ts';
    import IconStop from '#icons/IconStop.svelte';
    import Input from '#lib/form/Input.svelte';
    import { useI18n } from '#state/i18n.svelte.ts';
    import { untrack } from 'svelte';
    import Expandable from '#lib/Expandable.svelte';
    import { PATTERN_CLIENT_NAME } from '#utils/patterns.ts';
    import LabeledValue from '#lib/LabeledValue.svelte';
    import Button from '#lib/button/Button.svelte';
    import Modal from '#lib/Modal.svelte';
    import { fetchDelete, fetchPut } from '#api/fetch.ts';

    let {
        device = $bindable(),
        userId,
    }: {
        device: DeviceResponse;
        userId: string;
    } = $props();

    let t = useI18n();

    let name = $state(untrack(() => device.name));
    let error = $state('');
    let busy = $state(false);
    let showRevoke = $state(false);
    let closeRevoke = $state<() => void>();

    async function onSaveName() {
        if (busy || name === device.name) return;
        error = '';
        busy = true;
        const submittedName = name;
        const payload: DeviceRequest = {
            device_id: device.id,
            name: submittedName,
        };
        let res = await fetchPut(`/auth/v1/users/${userId}/devices`, payload);
        if (!res.error) {
            device.name = submittedName;
        } else {
            error = res.error.message;
        }
        busy = false;
    }

    async function onRevokeRefresh() {
        error = '';
        busy = true;
        let payload: DeviceRequest = {
            device_id: device.id,
        };
        let res = await fetchDelete(`/auth/v1/users/${userId}/devices`, payload);
        if (!res.error) {
            device.refresh_exp = undefined;
            closeRevoke?.();
        } else {
            error = res.error.message;
        }
        busy = false;
    }
</script>

<Expandable>
    {#snippet summary()}
        <span class="device-name">{device.name}</span>
    {/snippet}

    {#snippet details()}
        <div class="device">
            <LabeledValue label={t.account.deviceId} mono>
                {device.id}
            </LabeledValue>

            <div class="row">
                <Input
                    bind:value={name}
                    autocomplete="off"
                    label={t.account.deviceName}
                    placeholder={t.account.deviceName}
                    onEnter={onSaveName}
                    pattern={PATTERN_CLIENT_NAME}
                    required
                />
                {#if name !== device.name}
                    <Button onclick={onSaveName} isLoading={busy}>{t.common.save}</Button>
                {/if}
            </div>

            <LabeledValue label={t.account.regDate}>
                {formatDateFromTs(device.created)}
            </LabeledValue>

            <LabeledValue label={t.account.accessExp}>
                {formatDateFromTs(device.access_exp)}
            </LabeledValue>

            {#if device.refresh_exp}
                <LabeledValue label={t.account.accessRenew}>
                    <div>
                        {formatDateFromTs(device.refresh_exp)}
                    </div>
                    {#snippet button()}
                        <Button
                            ariaLabel={t.account.accessRenewDelete}
                            level={-3}
                            onclick={() => {
                                error = '';
                                showRevoke = true;
                            }}
                        >
                            <IconStop width="1rem" />
                        </Button>
                    {/snippet}
                </LabeledValue>
            {/if}

            <LabeledValue label={t.account.regIp}>
                {device.peer_ip}
            </LabeledValue>
            {#if error && !showRevoke}
                <p class="err" role="alert">{error}</p>
            {/if}
        </div>
    {/snippet}
</Expandable>

<Modal
    bind:showModal={showRevoke}
    bind:closeModal={closeRevoke}
    ariaLabel={t.account.accessRenewDelete}
    strict
>
    <h3>{t.account.accessRenewDelete}</h3>
    <p>{device.name}</p>
    {#if error}<p class="err" role="alert">{error}</p>{/if}
    <div class="confirm-actions">
        <Button level={2} onclick={() => closeRevoke?.()} isDisabled={busy}
            >{t.common.cancel}</Button
        >
        <Button level={-1} onclick={onRevokeRefresh} isLoading={busy}>{t.common.delete}</Button>
    </div>
</Modal>

<style>
    .device {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(min(100%, 15rem), 1fr));
        align-items: start;
        gap: 0.75rem 1rem;
    }

    .device :global(.admin-labeled-value) {
        margin: 0;
    }

    .device-name {
        font-size: var(--ui-control-font-size);
        font-weight: 600;
        overflow-wrap: anywhere;
    }

    .row,
    .confirm-actions {
        display: flex;
        align-items: end;
        flex-wrap: wrap;
        gap: 0.5rem;
    }

    .row {
        --ui-field-margin: 0;
        grid-column: 1 / -1;
    }

    .confirm-actions {
        justify-content: flex-end;
        margin-top: var(--ui-space);
    }
</style>
