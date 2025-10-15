<script lang="ts">
    import Button from "$lib5/button/Button.svelte";
    import {IS_DEV, TPL_RESTRICTED_EMAIL_DOMAIN} from "$utils/constants";
    import Input from "$lib5/form/Input.svelte";
    import LangSelector from "$lib5/LangSelector.svelte";
    import Main from "$lib5/Main.svelte";
    import ContentCenter from "$lib5/ContentCenter.svelte";
    import {useI18n} from "$state/i18n.svelte";
    import Template from "$lib5/Template.svelte";
    import {useParam} from "$state/param.svelte";
    import ThemeSwitch from "$lib5/ThemeSwitch.svelte";
    import Form from "$lib5/form/Form.svelte";
    import {PATTERN_USER_NAME} from "$utils/patterns";
    import type {NewUserRegistrationRequest} from "$api/types/register.ts";
    import {fetchGet, fetchPost} from "$api/fetch";
    import type {ToSLatestResponse} from "$api/types/tos";
    import Modal from "$lib/Modal.svelte";
    import {onMount} from "svelte";
    import {fetchSolvePow} from "$utils/pow";

    let t = useI18n();

    let refToS: undefined | HTMLParagraphElement = $state();
    let showModal = $state(false);
    let closeModal: undefined | (() => void) = $state();

    let restrictedDomain = $state('');
    let redirectUri = useParam('redirect_uri');
    let isLoading = $state(false);
    let err = $state('');
    let success = $state(false);

    let tos: undefined | ToSLatestResponse = $state();
    let tosRead = $state(false);
    let noTosExists = $state(false);

    let email = $state('');
    let givenName = $state('');
    let familyName = $state('');

    let action = $derived(IS_DEV ? '/auth/v1/dev/register' : '/auth/v1/users/register');

    async function fetchTos() {
        let res = await fetchGet<ToSLatestResponse>('/auth/v1/tos/latest');
        if (res.body) {
            tos = res.body;
        } else if (res.status === 204) {
            noTosExists = true;
        }
    }

    function onScrollEndToS() {
        if (!refToS) {
            return false;
        }

        // allow 50px diff for better UX
        if (!tosRead && refToS.scrollHeight <= refToS.scrollTop + refToS.offsetHeight + 50) {
            tosRead = true;
        }
    }

    async function onSubmit(form: HTMLFormElement, params: URLSearchParams) {
        success = false;
        err = '';

        if (!email || !givenName) {
            console.error('email, given_name, pow missing');
            return;
        }

        if (restrictedDomain && !email.endsWith(restrictedDomain)) {
            err = t.register.domainErr;
            return;
        }

        if (!tos) {
            await fetchTos();
        }

        if (tos) {
            showModal = true;
        } else if (noTosExists) {
            await submitRegistration();
        } else {
            console.error('logic error in ToS fetch / accept');
        }
    }

    async function acceptToS() {
        closeModal?.();
        await submitRegistration();
    }

    async function submitRegistration() {
        isLoading = true;

        let payload: NewUserRegistrationRequest = {
            email,
            given_name: givenName,
            family_name: familyName || undefined,
            pow: await fetchSolvePow() || '',
            redirect_uri: redirectUri.get(),
        };

        const res = await fetchPost(action, payload);
        if (res.error) {
            let error = res.error.message || 'Error';
            if (error.includes("UNIQUE constraint")) {
                err = t.register.alreadyRegistered;
            } else {
                err = error;
            }
        } else {
            err = '';
            success = true;
            if (redirectUri) {
                setTimeout(() => {
                    window.location.replace(payload.redirect_uri || '/auth/v1/account');
                }, 3000);
            }
        }

        isLoading = false;
    }
</script>

<svelte:head>
    <title>{t?.register.register || 'Register'}</title>
</svelte:head>

<Template id={TPL_RESTRICTED_EMAIL_DOMAIN} bind:value={restrictedDomain}/>

<Main>
    <ContentCenter>
        <div class="container">
            <div class="domainTxt">
                <h1>{t.register.userReg}</h1>
                {#if restrictedDomain}
                    {t.register.domainRestricted}<br>
                    {t.register.domainAllowed} <code>@{restrictedDomain}</code>
                {/if}
            </div>

            <Form {action} {onSubmit}>
                <Input
                        typ="email"
                        name="email"
                        autocomplete="email"
                        label={t.common.email}
                        placeholder={t.common.email}
                        bind:value={email}
                        required
                />
                <Input
                        name="given_name"
                        autocomplete="given-name"
                        label={t.account.givenName}
                        placeholder={t.account.givenName}
                        bind:value={givenName}
                        pattern={PATTERN_USER_NAME}
                        required
                />
                <Input
                        name="family_name"
                        autocomplete="family-name"
                        label={t.account.familyName}
                        placeholder={t.account.familyName}
                        bind:value={familyName}
                        pattern={PATTERN_USER_NAME}
                />

                <div class="submit">
                    <Button type="submit" {isLoading}>{t.register.register}</Button>
                </div>
                {#if success}
                    <div class="success">
                        {t.register.success}<br/>
                        {t.register.emailCheck}
                    </div>
                {:else if err}
                    <div class="err">
                        {err}
                    </div>
                {/if}

            </Form>
        </div>

        {#if tos}
            <Modal bind:showModal bind:closeModal>
                <h1>{t.tos.tos}</h1>
                <p bind:this={refToS} class="tosContent" onscrollend={onScrollEndToS}>
                    {#if tos.is_html}
                        {@html tos.content}
                    {:else}
                        {tos.content}
                    {/if}
                </p>
                <Button
                        ariaLabel={t.common.accept}
                        onclick={acceptToS}
                        isDisabled={!tosRead}
                        {isLoading}
                >
                    {t.common.accept}
                </Button>
                <Button
                        level={-2}
                        ariaLabel={t.common.cancel}
                        onclick={() => closeModal?.()}
                        {isLoading}
                >
                    {t.common.cancel}
                </Button>
            </Modal>
        {/if}

        <ThemeSwitch absolute/>
        <LangSelector absolute/>
    </ContentCenter>
</Main>

<style>
    .submit {
        margin-top: .66rem;
    }

    .container {
        padding: 1rem;
        display: flex;
        flex-direction: column;
        justify-content: center;
        border-radius: var(--border-radius);
        background: hsla(var(--bg-high) / .25);
    }

    .err {
        max-width: 16rem;
    }

    .err, .success {
        margin-top: .5rem;
    }

    .domainTxt {
        margin: .5rem 0;
    }

    .tosContent {
        max-height: min(75dvh, 40rem);
        margin-bottom: 1rem;
        padding-right: 1rem;
        overflow-y: auto;
    }
</style>
