<script lang="ts">
    import type { ClientResponse } from '#api/types/clients.ts';
    import { fetchDelete, fetchPost, fetchPut } from '#api/fetch.ts';
    import type { ThemeRequestResponse } from '#api/types/themes.ts';
    import Form from '#lib/form/Form.svelte';
    import { useI18n } from '#state/i18n.svelte.ts';
    import Button from '#lib/button/Button.svelte';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import Input from '#lib/form/Input.svelte';
    import { PATTERN_CSS_VALUE_LOOSE } from '#utils/patterns.ts';
    import BrandingMode from '#lib/admin/clients/branding/BrandingMode.svelte';
    import BrandingPreviewWrapper from '#lib/admin/clients/branding/BrandingPreviewWrapper.svelte';
    import { genKey } from '#utils/helpers.ts';
    import InputFile from '#lib/form/InputFile.svelte';
    import IconCheck from '#icons/IconCheck.svelte';
    import IconTrash from '#icons/IconTrash.svelte';

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

<div class="branding-layout">
    <Form action={url} {onSubmit}>
        <div class="layout">
            {#if theme}
                <section class="panel values">
                    <h2>{ta.nav.config}</h2>
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

                    <h3>Light Theme</h3>
                    <BrandingMode bind:values={theme.light} />

                    <h3>Dark Theme</h3>
                    <BrandingMode bind:values={theme.dark} />
                </section>
                <div class="side">
                    <section class="panel">
                        {#key logoKey}
                            <BrandingPreviewWrapper {logoUrl} {theme} />
                        {/key}
                    </section>

                    <section class="panel">
                        <h3>Logo Upload</h3>
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

                        <h3>{ta.clients.branding.faviconUpload}</h3>
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
                    </section>
                </div>
                <div class="buttons">
                    <Button type="submit">{t.common.save}</Button>
                    <Button level={-2} onclick={reset}>{ta.common.reset}</Button>
                    {#if success}<IconCheck />{/if}
                    {#if err}<span class="err" role="alert">{err}</span>{/if}
                </div>
            {:else if err}
                <div class="err" role="alert">{err}</div>
            {/if}
        </div>
    </Form>
</div>

<style>
    .buttons {
        position: sticky;
        bottom: 0;
        z-index: 2;
        grid-column: 1 / -1;
        display: flex;
        flex-wrap: wrap;
        align-items: center;
        gap: 0.5rem;
        padding-block: var(--ui-space);
        background: var(--ui-surface);
        border-top: 1px solid var(--ui-outline);
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

    .branding-layout {
        container-type: inline-size;
    }

    .layout,
    .side {
        display: grid;
        gap: var(--ui-space);
    }

    .layout {
        align-items: start;
        margin-top: var(--ui-space);
    }

    .panel {
        min-width: 0;
        padding: var(--ui-space);
        border: 1px solid var(--ui-outline);
        border-radius: var(--border-radius);
    }

    .panel > h2,
    .panel > h3:first-child {
        margin-top: 0;
    }

    .panel h3 {
        margin-block: 1.25rem 0.5rem;
    }

    @container (width >= 46rem) {
        .layout {
            grid-template-columns: minmax(0, 1fr) minmax(18rem, 22rem);
        }
    }
</style>
