<script lang="ts">
    import Button from "$lib5/button/Button.svelte";
    import {useI18n} from "$state/i18n.svelte.js";
    import {fetchPost} from "$api/fetch";
    import type {PamPasswordResponse, PamUserResponse} from "$api/types/pam";
    import LabeledValue from "$lib/LabeledValue.svelte";
    import InputPassword from "$lib/form/InputPassword.svelte"
    import {slide} from "svelte/transition";
    import {formatDateFromTs} from "$utils/helpers";

    let {
        pamUser,
    }: {
        pamUser: PamUserResponse,
    } = $props();

    let t = useI18n();

    let err = $state('');

    let password = $state('');
    let expTs: undefined | number = $state();
    let expSecs: undefined | number = $state();

    let interval: undefined | number;

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
    <LabeledValue label="UID">
        {pamUser.id}
    </LabeledValue>
    <LabeledValue label="GID">
        {pamUser.gid}
    </LabeledValue>
    <LabeledValue label={t.account.pam.username}>
        {pamUser.name}
    </LabeledValue>
    <LabeledValue label="Shell">
        <code>{pamUser.shell}</code>
    </LabeledValue>

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
        <div class="exp" transition:slide={{ duration: 150 }}>
            {t.account.pam.validFor.replace('{{ secs }}', expSecs.toString())}
        </div>
    {/if}

    <div class="btn">
        <Button ariaLabel={t.account.pam.generatePassword} onclick={generateNew}>
            {t.account.pam.generatePassword}
        </Button>

        {#if err}
            <div class="err">
                {err}
            </div>
        {/if}
    </div>
</div>

<style>
    .btn {
        margin-top: 1rem;
        display: flex;
        align-items: center;
        gap: .5rem;
    }

    .exp {
        margin-bottom: 1rem;
        color: hsl(var(--action));
    }
</style>
