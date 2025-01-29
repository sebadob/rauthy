<script lang="ts">
    import PasswordPolicy from "$lib5/PasswordPolicy.svelte";
    import {onMount} from "svelte";
    import {generatePassword} from "$utils/helpers.ts";
    import InputPassword from "$lib5/form/InputPassword.svelte";
    import Button from "$lib5/Button.svelte";
    import {useI18n} from "$state/i18n.svelte.js";
    import type {PasswordPolicyResponse} from "$api/types/password_policy.ts";
    import {fetchGet} from "$api/fetch.ts";
    import type {PropsPassword} from "./props.ts";

    let {
        passwords = $bindable(),
        hideCurrentPassword = false,
        inputWidth,
        isValid = $bindable(),
    }: {
        passwords: PropsPassword
        hideCurrentPassword?: boolean,
        inputWidth: string,
        isValid: undefined | (() => boolean),
    } = $props();

    let t = useI18n();

    isValid = isPwdValid;

    let accepted = $state(false);
    let err = $state('');
    let policy: undefined | PasswordPolicyResponse = $state();
    let showCopy = $derived(passwords.new?.length > 6 && passwords.new === passwords.newConfirm);

    onMount(async () => {
        let res = await fetchGet<PasswordPolicyResponse>('/auth/v1/password_policy');
        if (res.body) {
            policy = res.body;
        } else {
            console.error(res.error);
        }
    });

    export function isPwdValid(): boolean {
        err = '';


        if (!hideCurrentPassword && !passwords.current) {
            err = t.account.passwordCurrReq;
            return false;
        }
        if (!passwords.new) {
            err = t.account.passwordNewReq;
            return false;
        }
        if (!passwords.newConfirm) {
            err = t.account.passwordNewReq;
            return false;
        }

        if (!accepted) {
            err = t.account.passwordPolicyFollow;
            return false;
        }
        if (passwords.new.length > 256) {
            err = 'max 256';
            return false;
        }

        if (passwords.new !== passwords.newConfirm) {
            err = t.account.passwordNoMatch;
            return false;
        }

        return true;
    }

    function generate() {
        err = '';
        if (policy) {
            let pwd = generatePassword(policy);
            passwords.new = pwd;
            passwords.newConfirm = pwd;
        }
    }

</script>

{#if policy}
    <div class="container">
        <PasswordPolicy bind:accepted {policy} password={passwords.new}/>

        {#if !hideCurrentPassword}
            <InputPassword
                    bind:value={passwords.current}
                    autocomplete="current-password"
                    label={t.account.passwordCurr}
                    placeholder={t.account.passwordCurr}
                    onInput={isPwdValid}
                    width={inputWidth}
            />
        {/if}
        <InputPassword
                bind:value={passwords.new}
                autocomplete="new-password"
                label={t.account.passwordNew}
                placeholder={t.account.passwordNew}
                onInput={isPwdValid}
                {showCopy}
                width={inputWidth}
        />
        <InputPassword
                bind:value={passwords.newConfirm}
                autocomplete="new-password"
                label={t.account.passwordConfirm}
                placeholder={t.account.passwordConfirm}
                onInput={isPwdValid}
                {showCopy}
                width={inputWidth}
        />

        <div class="btn">
            <Button onclick={generate} level={2}>
                {t.account.generateRandom}
            </Button>
        </div>

        <div class="err">
            {err}
        </div>
    </div>
{/if}

<style>
    .btn {
        margin: .5rem 0;
    }

    .container {
        margin: .5rem 0;
    }

    .err {
        margin: .5rem 0;
        text-align: left;
        color: hsl(var(--error));
    }
</style>
