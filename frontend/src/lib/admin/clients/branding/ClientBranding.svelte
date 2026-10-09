<script lang="ts">
    import type { ClientResponse } from '$api/types/clients.ts';
    import { fetchDelete, fetchPost, fetchPut } from '$api/fetch';
    import type { ThemeRequestResponse } from '$api/types/themes.ts';
    import Form from '$lib5/form/Form.svelte';
    import { useI18n } from '$state/i18n.svelte';
    import Button from '$lib5/button/Button.svelte';
    import { useI18nAdmin } from '$state/i18n_admin.svelte';
    import Input from '$lib5/form/Input.svelte';
    import { PATTERN_CSS_VALUE_LOOSE } from '$utils/patterns';
    import BrandingMode from '$lib5/admin/clients/branding/BrandingMode.svelte';
    import BrandingPreviewWrapper from '$lib5/admin/clients/branding/BrandingPreviewWrapper.svelte';
    import { genKey } from '$utils/helpers';
    import InputFile from '$lib5/form/InputFile.svelte';
    import IconCheck from '$icons/IconCheck.svelte';
    import IconTrash from '$icons/IconTrash.svelte';

    let {
        client,
    }: {
        client: ClientResponse;
    } = $props();

    const inputWidth = '13rem';

    let t = useI18n();
    let ta = useI18nAdmin();

    let success = $state(false);
    let err = $state('');
    let theme: ThemeRequestResponse | undefined = $state();

    let logoKey = $state(genKey());
    let logoUrl = $derived(`/auth/v1/clients/${client.id}/logo?${logoKey}`);
    let faviconKey = $state(genKey());
    let faviconLoaded = $state(false);
    let faviconUrl = $derived(`/auth/v1/clients/${client.id}/favicon?${faviconKey}`);
    let url = $derived(`/auth/v1/theme/${client.id}`);

    $effect(() => {
        fetchTheme();
    });

    async function fetchTheme() {
        let res = await fetchPost<ThemeRequestResponse>(url);
        if (res.body) {
            theme = res.body;
        } else {
            err = res.error?.message || 'Error';
        }
    }

    async function onSubmit(form: HTMLFormElement, params: URLSearchParams) {
        if (!theme) {
            console.error('theme is undefined');
            return;
        }

        let payload = theme;
        // this could be `rauthy` the default has been used before
        payload.client_id = client.id;

        let res = await fetchPut(url, payload);
        if (res.error) {
            err = res.error.message;
        } else {
            success = true;
            setTimeout(() => {
                success = false;
            }, 2000);
        }
    }

    async function reset() {
        let res = await fetchDelete(url);
        if (res.error) {
            err = res.error.message;
        } else {
            await fetchDelete(`/auth/v1/clients/${client.id}/logo`);
            await fetchDelete(`/auth/v1/clients/${client.id}/favicon`);

            await fetchTheme();
            logoKey = genKey();
            faviconLoaded = false;
            faviconKey = genKey();
            success = true;
            setTimeout(() => {
                success = false;
            }, 2000);
        }
    }

    async function onUploadSuccess() {
        logoKey = genKey();
    }

    async function onLogoDelete() {
        let res = await fetchDelete(`/auth/v1/clients/${client.id}/logo`);
        if (res.error) {
            err = res.error.message;
        } else {
            logoKey = genKey();
        }
    }

    async function onFaviconDelete() {
        let res = await fetchDelete(`/auth/v1/clients/${client.id}/favicon`);
        if (res.error) {
            err = res.error.message;
        } else {
            faviconLoaded = false;
            faviconKey = genKey();
        }
    }

    function onFaviconUploadSuccess() {
        faviconLoaded = false;
        faviconKey = genKey();
    }
</script>

<Form action={url} {onSubmit}>
    <div class="container">
        {#if theme}
            <div class="values">
                <p>{@html ta.clients.branding.descVariables}</p>

                <Input
                    label="--border-radius"
                    placeholder="--border-radius"
                    errMsg={ta.validation.css}
                    width={inputWidth}
                    bind:value={theme.border_radius}
                    required
                    pattern={PATTERN_CSS_VALUE_LOOSE}
                />

                <h1>Light Theme</h1>
                <BrandingMode bind:values={theme.light} />

                <br />
                <h1>Dark Theme</h1>
                <BrandingMode bind:values={theme.dark} />
            </div>
            <div class="preview">
                {#key logoKey}
                    <BrandingPreviewWrapper {logoUrl} {theme} />
                {/key}

                <hr />

                <p>Logo Upload</p>
                <div class="logo">
                    <InputFile
                        method="PUT"
                        url={`/auth/v1/clients/${client.id}/logo`}
                        fileName="logo"
                        width="100%"
                        onSuccess={onUploadSuccess}
                    />
                    <Button ariaLabel={t.common.delete} invisible onclick={onLogoDelete}>
                        <IconTrash width="1.25rem" />
                    </Button>
                </div>

                <p>{ta.clients.branding.faviconUpload}</p>
                <div class="favicon">
                    {#key faviconKey}
                        <img
                            class:loaded={faviconLoaded}
                            src={faviconUrl}
                            alt={ta.clients.branding.faviconPreviewAlt}
                            onload={() => (faviconLoaded = true)}
                            onerror={() => (faviconLoaded = false)}
                        />
                    {/key}
                    <InputFile
                        method="PUT"
                        url={`/auth/v1/clients/${client.id}/favicon`}
                        fileName="favicon"
                        width="100%"
                        onSuccess={onFaviconUploadSuccess}
                    />
                    <Button ariaLabel={t.common.delete} invisible onclick={onFaviconDelete}>
                        <IconTrash width="1.25rem" />
                    </Button>
                </div>

                <div class="buttons">
                    <Button type="submit">
                        {t.common.save}
                    </Button>
                    <Button level={-2} onclick={reset}>
                        {ta.common.reset}
                    </Button>

                    {#if success}
                        <IconCheck />
                    {/if}
                </div>

                {#if err}
                    <div class="err">
                        {err}
                    </div>
                {/if}
            </div>
        {/if}
    </div>
</Form>

<style>
    .buttons {
        margin: 1rem 0;
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }

    .logo,
    .favicon {
        display: grid;
        grid-template-columns: minmax(0, 1fr) auto;
        align-items: center;
        gap: 0.5rem;
    }

    .favicon {
        grid-template-columns: 2rem minmax(0, 1fr) auto;
    }

    .favicon img {
        width: 2rem;
        height: 2rem;
        visibility: hidden;
        object-fit: contain;
    }

    .favicon img.loaded {
        visibility: visible;
    }

    .container {
        max-height: calc(100dvh - 4.5rem);
        display: grid;
        grid-template-columns: 1fr 17rem;
        gap: 0.5rem;
    }

    .values {
        max-height: calc(100dvh - 5.5rem);
        margin-top: 1rem;
        padding-right: 0.5rem;
        overflow: auto;
    }

    @media (max-width: 88rem) {
        .container {
            grid-template-columns: 1fr;
        }

        .values {
            max-height: inherit;
        }
    }
</style>
