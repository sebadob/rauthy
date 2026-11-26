<script lang="ts">
    import Button from '#lib/button/Button.svelte';
    import Argon2Results from './Argon2Results.svelte';
    import Input from '#lib/form/Input.svelte';
    import Form from '#lib/form/Form.svelte';
    import { fetchPost } from '#api/fetch.ts';
    import type {
        PasswordHashTime,
        PasswordHashTimes,
        PasswordHashTimesRequest,
    } from '#api/types/password_hashing.ts';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import type { Argon2ParamsResponse } from '#api/types/login_times.ts';
    import { tick, untrack } from 'svelte';

    let {
        params,
    }: {
        params: Argon2ParamsResponse;
    } = $props();

    let ta = useI18nAdmin();

    let isLoading = $state(false);
    let err = $state('');
    let hashTimes: PasswordHashTime[] = $state([]);
    let result: HTMLDivElement | undefined = $state();

    let values = $state({
        targetTime: '1000',
        mCost: untrack(() => params.m_cost.toString() || '32768'),
        pCost: untrack(() => params.p_cost.toString() || '2'),
    });

    async function onSubmit(form: HTMLFormElement, params: URLSearchParams) {
        isLoading = true;
        hashTimes = [];

        const payload: PasswordHashTimesRequest = {
            target_time: Number.parseInt(values.targetTime),
            m_cost: Number.parseInt(values.mCost),
            p_cost: Number.parseInt(values.pCost),
        };

        let res = await fetchPost<PasswordHashTimes>(form.action, payload);
        if (res.body) {
            hashTimes = res.body.results;
            err = '';
            await tick();
            result?.scrollIntoView({ block: 'nearest' });
        } else {
            err = res.error?.message || 'Error';
        }

        isLoading = false;
    }
</script>

<div class="calculator">
    <Form action="/auth/v1/password_hash_times" {onSubmit}>
        <div class="parameters">
            <Input
                typ="number"
                bind:value={values.targetTime}
                autocomplete="off"
                label={`${ta.docs.hashing.targetTime} (ms)`}
                placeholder={`${ta.docs.hashing.targetTime} (ms)`}
                min="500"
                max="99999"
                required
                width="100%"
            />
            <Input
                typ="number"
                bind:value={values.mCost}
                autocomplete="off"
                label="m_cost (kB)"
                placeholder="m_cost (kB)"
                min="32768"
                width="100%"
            />
            <Input
                typ="number"
                bind:value={values.pCost}
                autocomplete="off"
                label="p_cost"
                placeholder="p_cost"
                min="2"
                max="256"
                width="100%"
            />
            <Button type="submit" {isLoading}>
                {ta.docs.hashing.calculate}
            </Button>
        </div>
    </Form>

    {#if hashTimes.length > 0}
        <div bind:this={result}>
            <Argon2Results res={hashTimes} />
        </div>
    {/if}
    {#if err}<p class="err" role="alert">{err}</p>{/if}
</div>

<style>
    .calculator {
        display: grid;
        gap: var(--ui-space);
    }

    .parameters {
        --ui-field-margin: 0;
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(min(100%, 11rem), 1fr));
        align-items: end;
        gap: 0.75rem;
    }

    .parameters :global(button) {
        justify-self: start;
        min-height: var(--ui-field-height);
    }

    .err {
        margin: 0;
    }
</style>
