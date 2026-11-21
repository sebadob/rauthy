<script lang="ts">
    import Button from '#lib/button/Button.svelte';
    import { TPL_CLIENT_LOGO_UPDATED } from '#utils/constants.ts';
    import Input from '#lib/form/Input.svelte';
    import PublicPage from '#lib/PublicPage.svelte';
    import { useI18n } from '#state/i18n.svelte.js';
    import Template from '#lib/Template.svelte';
    import { useParam } from '#state/param.svelte.js';
    import Form from '#lib/form/Form.svelte';
    import ClientLogo from '#lib/ClientLogo.svelte';
    import { fetchSolvePow } from '#utils/pow.ts';
    import type { RequestResetRequest } from '#api/types/authorize.ts';
    import { fetchPost } from '#api/fetch.ts';

    let t = useI18n();

    let email_hint = useParam('email_hint');

    let clientLogoUpdated = $state(-1);
    let isLoading = $state(false);
    let err = $state('');
    let success = $state(false);

    async function onSubmit(form: HTMLFormElement, params: URLSearchParams) {
        isLoading = true;

        let email = params.get('email');
        if (!email) {
            console.error('email is empty');
            return;
        }
        let pow = (await fetchSolvePow()) || '';
        let payload: RequestResetRequest = { email, pow };

        let res = await fetchPost(form.action, payload);
        if (res.error) {
            err = res.error.message;
        } else {
            success = true;
        }

        isLoading = false;
    }
</script>

<svelte:head>
    <title>{t.passwordReset.passwordReset}</title>
</svelte:head>

<Template id={TPL_CLIENT_LOGO_UPDATED} bind:value={clientLogoUpdated} />

<PublicPage>
    <div class="public-card">
        <ClientLogo
            clientId="rauthy"
            updated={clientLogoUpdated > -1 ? clientLogoUpdated : undefined}
        />

        <div class="desc">
            <h1>{t.passwordReset.passwordReset}</h1>
            <p>{t.authorize.passwordResetDesc}</p>
        </div>

        <Form action="/auth/v1/users/request_reset" {onSubmit} withPowAs="pow">
            <Input
                typ="email"
                name="email"
                autocomplete="email"
                label={t.common.email}
                placeholder={t.common.email}
                required
                value={email_hint.get()}
            />

            <div class="submit">
                <Button type="submit" {isLoading}>{t.authorize.passwordRequest}</Button>
            </div>
            {#if success}
                <div class="success">
                    {t.authorize.passwordResetSuccess}<br />
                </div>
            {:else if err}
                <div class="err">
                    {err}
                </div>
            {/if}
        </Form>
    </div>
</PublicPage>

<style>
    .submit {
        margin-top: 0.66rem;
    }

    .err {
        max-width: 16rem;
    }

    .err,
    .success {
        margin-top: 0.5rem;
    }

    .success {
        color: hsl(var(--action));
    }

    .desc {
        margin: 0.5rem 0;
    }
</style>
