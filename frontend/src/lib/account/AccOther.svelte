<script lang="ts">
    import Button from '$lib5/button/Button.svelte';
    import { fade } from 'svelte/transition';
    import Input from '$lib5/form/Input.svelte';
    import { useI18n } from '$state/i18n.svelte.js';
    import type {
        UpdateUserSelfRequest,
        UserAttrValueRequest,
        UserAttrValuesUpdateRequest,
        UserResponse,
    } from '$api/types/user.ts';
    import Form from '$lib5/form/Form.svelte';
    import {
        PATTERN_CITY,
        PATTERN_PHONE,
        PATTERN_STREET,
        PATTERN_USER_NAME,
    } from '$utils/patterns';
    import IconCheck from '$icons/IconCheck.svelte';
    import { errorFromResponse, fetchGet, fetchPut } from '$api/fetch';
    import InputDateTimeCombo from '$lib5/form/InputDateTimeCombo.svelte';
    import type {
        UserEditableAttrResponse,
        UserEditableAttrsResponse,
    } from '$api/types/user_attrs';
    import { onMount } from 'svelte';

    let {
        user = $bindable(),
        viewModePhone,
    }: {
        user: UserResponse;
        viewModePhone?: boolean;
    } = $props();

    let t = useI18n();

    let err = $state('');
    let success = $state(false);

    let attrs: UserEditableAttrResponse[] = $state([]);

    onMount(() => {
        fetchAttrs();
    });

    async function fetchAttrs() {
        let res = await fetchGet<UserEditableAttrsResponse>(
            `/auth/v1/users/${user.id}/attr/editable`,
        );
        if (res.body) {
            attrs = res.body.values;
        } else {
            err = res.error?.message || 'Error fetching User Attributes';
        }
    }

    async function onSubmit() {
        err = '';

        let userKeys = attrs.map(a => a.name);
        let payload: UserAttrValuesUpdateRequest = {
            values: attrs
                .filter(a => (a.value && a.value.trim()) || userKeys.includes(a.name))
                .map(a => {
                    let v: UserAttrValueRequest = {
                        key: a.name,
                        value: a.value?.trim() || '',
                    };
                    return v;
                }),
        };

        let res = await fetchPut(`/auth/v1/users/${user.id}/attr`, payload);
        if (res.error) {
            err = res.error.message;
        } else {
            success = true;
            setTimeout(() => {
                success = false;
            }, 3000);

            await fetchAttrs();
        }
    }
</script>

<div class="container">
    <div>
        {#each attrs as a (a.name)}
            <Input
                bind:value={a.value}
                autocomplete="off"
                label={a.name}
                placeholder={a.default_value ? a.default_value : a.desc || ''}
                width="100%"
                onEnter={onSubmit}
            />
        {/each}
    </div>

    {#if attrs.length > 0}
        <div class="bottom">
            <div>
                <Button onclick={onSubmit}>
                    {t.common.save}
                </Button>
            </div>

            {#if success}
                <div
                    class="success"
                    transition:fade
                >
                    <IconCheck />
                </div>
            {/if}
        </div>
        {#if err}
            <div
                class="err"
                transition:fade
            >
                {err}
            </div>
        {/if}
    {/if}
</div>

<style>
    .bottom {
        display: flex;
        align-items: center;
        gap: 1rem;
    }

    .container {
        display: flex;
        flex-direction: column;
    }

    .err {
        margin-top: 0.5rem;
        color: hsl(var(--error));
    }

    .success {
        margin-bottom: -0.25rem;
    }
</style>
