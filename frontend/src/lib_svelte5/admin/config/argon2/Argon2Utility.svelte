<script lang="ts">
    import Button from "$lib5/Button.svelte";
    import Argon2Results from "./Argon2Results.svelte";
    import Input from "$lib5/form/Input.svelte";
    import Form from "$lib5/form/Form.svelte";
    import {fetchPost} from "$api/fetch.ts";
    import type {PasswordHashTime, PasswordHashTimes, PasswordHashTimesRequest} from "$api/types/password_hashing.ts";
    import {useI18nAdmin} from "$state/i18n_admin.svelte.ts";
    import type {Argon2ParamsResponse} from "$api/types/login_times.ts";

    let {
        params,
    }: {
        params: Argon2ParamsResponse,
    } = $props();

    const inputWidth = '10rem';

    let ta = useI18nAdmin();

    let isLoading = $state(false);
    let err = $state('');
    let hashTimes: PasswordHashTime[] = $state([]);
    let scrollY: undefined | number = $state();

    let values = $state({
        targetTime: '1000',
        mCost: params.m_cost.toString() || '32768',
        pCost: params.p_cost.toString() || '2',
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
            setTimeout(() => {
                scrollY = window.innerHeight;
            }, 50);
        } else {
            err = res.error?.message || 'Error';
        }

        isLoading = false;
    }
</script>

<svelte:window bind:scrollY={scrollY}/>

<div class="wrapper">
    <div class="container">
        <Form action="/auth/v1/password_hash_times" {onSubmit}>
            <Input
                    typ="number"
                    bind:value={values.targetTime}
                    autocomplete="off"
                    label={`${ta.docs.hashing.targetTime} (ms)`}
                    placeholder={`${ta.docs.hashing.targetTime} (ms)`}
                    min="500"
                    max="99999"
                    required
                    width={inputWidth}
            />
            <Input
                    typ="number"
                    bind:value={values.mCost}
                    autocomplete="off"
                    label="m_cost (kB)"
                    placeholder="m_cost (kB)"
                    min="32768"
                    width={inputWidth}
            />
            <Input
                    typ="number"
                    bind:value={values.pCost}
                    autocomplete="off"
                    label="p_cost"
                    placeholder="p_cost"
                    min="2"
                    max="256"
                    width={inputWidth}
            />
            <Button type="submit" isLoading={isLoading}>
                {ta.docs.hashing.calculate}
            </Button>
        </Form>
    </div>

    {#if hashTimes.length > 0}
        <div class="result">
            <Argon2Results res={hashTimes}/>
        </div>
    {/if}

    {#if err}
        <div class="err">
            {err}
        </div>
    {/if}
</div>

<style>
    .container {
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: flex-start;
    }

    .result {
        margin-top: 5px;
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: flex-start;
    }

    .wrapper {
        display: flex;
        flex-wrap: wrap;
        gap: 1rem;
    }
</style>
