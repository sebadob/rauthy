<script lang="ts">
    import PasswordPolicy from "$lib5/PasswordPolicy.svelte";
    import {onMount} from "svelte";
    import Button from "$lib5/button/Button.svelte";
    import {generatePassword} from "$utils/helpers";
    import type {UpdateUserRequest, UserResponse} from "$api/types/user.ts";
    import type {PasswordPolicyResponse} from "$api/types/password_policy.ts";
    import InputPassword from "$lib5/form/InputPassword.svelte";
    import IconCheck from "$icons/IconCheck.svelte";
    import Form from "$lib5/form/Form.svelte";
    import {fetchGet, fetchPost, fetchPut} from "$api/fetch";
    import type {RequestResetRequest} from "$api/types/authorize.ts";
    import {useI18n} from "$state/i18n.svelte";
    import {useI18nAdmin} from "$state/i18n_admin.svelte";

    let {
        user,
        onSave,
    }: {
        user: UserResponse,
        onSave: () => void,
    } = $props();

    const inputWidth = 'min(20rem, calc(100dvw - .5rem))';

    let t = useI18n();
    let ta = useI18nAdmin();

    let isLoading = $state(false);
    let err = $state('');
    let success = $state(false);

    let policy: undefined | PasswordPolicyResponse = $state();
    let accepted = $state(false);
    let manualInit = $state(false);

    let pwdNew = $state('');
    let pwdVerify = $state('');
    let showCopy = $derived(pwdNew.length > (policy?.length_min || 8) && pwdNew === pwdVerify);

    let reportValidityNew: undefined | (() => void) = $state();
    let reportValidityConfirm: undefined | (() => void) = $state();

    onMount(async () => {
        let res = await fetchGet<PasswordPolicyResponse>('/auth/v1/password_policy');
        if (res.body) {
            policy = res.body;
        } else {
            err = res.error?.message || 'Error';
        }
    });

    $effect(() => {
        if (user.id) {
            pwdNew = '';
            pwdVerify = '';
            manualInit = false;
        }
    });

    async function sendEmail() {
        err = '';
        isLoading = true;

        let payload: RequestResetRequest = {
            email: user.email,
        };

        let res = await fetchPost('/auth/v1/users/request_reset', payload);
        if (res.error) {
            err = res.error.message;
        } else {
            success = true;
            setTimeout(() => {
                success = false;
            }, 3000);
        }

        isLoading = false;
    }

    async function savePwd(form: HTMLFormElement, params: URLSearchParams) {
        if (!accepted) {
            err = t.account.passwordPolicyFollow;
            return;
        }
        if (pwdNew !== pwdVerify) {
            err = t.account.passwordNoMatch;
            return;
        }

        let payload: UpdateUserRequest = {
            email: user.email,
            given_name: user.given_name,
            family_name: user.family_name,
            language: user.language,
            password: pwdNew,
            roles: user.roles,
            groups: user.groups,
            enabled: user.enabled,
            email_verified: user.email_verified,
            user_expires: user.user_expires,
        };

        err = '';
        isLoading = true;

        let res = await fetchPut(form.action, payload);
        if (res.error) {
            err = res.error.message;
        } else {
            success = true;
            setTimeout(() => {
                success = false;
                onSave();
            }, 3000);
        }

        isLoading = false;
    }

    function generate() {
        err = '';
        if (!policy) {
            return;
        }
        let pwd = generatePassword(policy);
        pwdNew = pwd;
        pwdVerify = pwd;

        requestAnimationFrame(() => {
            reportValidityNew?.();
            reportValidityConfirm?.();
        });
    }

</script>

{#if user.account_type === "new" && !manualInit}
    <p><b>{ta.users.pwdNoInit}</b></p>
    <p>{ta.users.pwdSendEmailDesc}</p>

    <Button onclick={sendEmail} {isLoading}>
        {ta.users.pwdSendEmailBtn}
    </Button>

    <p style:margin-top="1rem">{ta.users.manualInitDesc}</p>
    <Button level={2} onclick={() => manualInit = true}>
        {ta.users.manualInit}
    </Button>
{:else if user.account_type === "passkey" || user.account_type === 'federated_passkey'}
    <div class="desc">
        <p><b>{ta.users.pkOnly1}</b></p>
        <p>{ta.users.pkOnly2}</p>
        <p>{ta.users.pkOnly3}</p>
    </div>
{:else}
    <div style:margin-top=".5rem"></div>

    {#if policy}
        <PasswordPolicy password={pwdNew} bind:accepted {policy}/>
    {/if}

    <Form action={`/auth/v1/users/${user.id}`} onSubmit={savePwd}>
        <InputPassword
                bind:value={pwdNew}
                autocomplete="off"
                label={t.account.passwordNew}
                placeholder={t.account.passwordNew}
                {showCopy}
                bind:reportValidity={reportValidityNew}
                required
                maxLength={policy?.length_max || 256}
                width={inputWidth}
        />
        <InputPassword
                bind:value={pwdVerify}
                autocomplete="off"
                label={t.account.passwordConfirm}
                placeholder={t.account.passwordConfirm}
                bind:reportValidity={reportValidityConfirm}
                required
                maxLength={policy?.length_max || 256}
                width={inputWidth}
        />

        <Button level={2} onclick={generate}>
            {t.account.generateRandom}
        </Button>

        <div style:margin-top="1rem"></div>
        <p>{ta.users.selfServiceDesc}</p>

        <div class="flex gap-05">
            <Button onclick={sendEmail} {isLoading}>
                {ta.users.sendResetEmail}
            </Button>

            <Button type="submit" level={2} {isLoading}>
                {ta.users.savePassword}
            </Button>

            {#if success}
                <IconCheck/>
            {/if}
        </div>
    </Form>
{/if}

{#if err}
    <div class="err">
        {err}
    </div>
{/if}

<style>
</style>
