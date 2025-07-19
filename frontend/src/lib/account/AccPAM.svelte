<script lang="ts">
    import Button from "$lib5/button/Button.svelte";
    import {onMount} from "svelte";
    import Input from "$lib5/form/Input.svelte";
    import {useI18n} from "$state/i18n.svelte.js";
    import {useSession} from "$state/session.svelte.js";
    import {fetchDelete, fetchGet, fetchPost} from "$api/fetch";
    import type {PasskeyResponse, WebauthnDeleteRequest} from "$api/types/webauthn.ts";
    import type {UserResponse} from "$api/types/user.ts";
    import {PATTERN_LINUX_USERNAME, PATTERN_USER_NAME} from "$utils/patterns";
    import {webauthnReg} from "$webauthn/registration";
    import WebauthnRequest from "$lib5/WebauthnRequest.svelte";
    import type {MfaPurpose, WebauthnAdditionalData, WebauthnServiceReq} from "$webauthn/types.ts";
    import UserPasskey from "$lib5/UserPasskey.svelte";
    import type {MfaModTokenResponse, UserMfaTokenRequest} from "$api/types/mfa_mod_token";
    import Modal from "$lib/Modal.svelte";
    import InputPassword from "$lib/form/InputPassword.svelte";
    import Form from "$lib/form/Form.svelte";
    import IconArrowPathSquare from "$icons/IconArrowPathSquare.svelte";
    import type {PamUsernameCheckRequest, PamUserResponse} from "$api/types/pam";
    import {fetchSolvePow} from "$utils/pow";

    let {user}: { user: UserResponse } = $props();

    let t = useI18n();

    let err = $state('');
    let isLoading = $state(false);

    let pamUser: undefined | PamUserResponse = $state();
    let noneExists = $state(false);

    let username = $state('');
    let showConfirm = $state(false);

    onMount(() => {
        fetchPamAccount();
    });

    async function checkUsername(form: HTMLFormElement, params: URLSearchParams) {
        showConfirm = false;
        isLoading = true;

        let pow = await fetchSolvePow() || '';
        let payload: PamUsernameCheckRequest = {
            username,
            pow
        };

        let res = await fetchPost(form.action, payload);
        if (res.status === 200) {
            showConfirm = true;
        } else {
            err = 'name taken';
        }

        isLoading = false;
    }

    async function fetchPamAccount() {
        let res = await fetchGet<PamUserResponse>(`/auth/v1/pam/users/${user.id}`);
        if (res.body) {
            pamUser = res.body;
        } else if (res.status === 404) {
            noneExists = true;
        }
    }
</script>

<div class="container">
    {#if noneExists}
        <p>{t.account.pam.create1}</p>
        <p>{t.account.pam.create2}</p>
        <p>{t.account.pam.create3}</p>
        <p class="err">{t.account.pam.create3}</p>

        {#if showConfirm}
            Available!
        {:else}
            <Form action="/auth/v1/pam/check" onSubmit={checkUsername}>
                <Input
                        autocomplete="off"
                        label={t.account.pam.username}
                        placeholder={t.account.pam.username}
                        width="20rem"
                        bind:value={username}
                        required
                        pattern={PATTERN_LINUX_USERNAME}
                />
                <Button type="submit" {isLoading}>
                    {t.account.pam.checkAvailability}
                </Button>
            </Form>
        {/if}
    {:else if pamUser}
        {pamUser}
    {/if}
</div>

<style>
    p {
        margin: .5rem 0;
    }
</style>
