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
    import BrandingPreview from '#lib/admin/clients/branding/BrandingPreview.svelte';
    import Section from '#lib/Section.svelte';
    import Tabs from '#lib/tabs/Tabs.svelte';
    import Modal from '#lib/Modal.svelte';
    import EntityDelete from '#lib/admin/EntityDelete.svelte';
    import { genKey } from '#utils/helpers.ts';
    import InputFile from '#lib/form/InputFile.svelte';
    import IconCheck from '#icons/IconCheck.svelte';

    let {
        client,
    }: {
        client: ClientResponse;
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    let success = $state(false);
    let err = $state('');
    let theme: ThemeRequestResponse | undefined = $state();
    const branding = ta.clients.branding;
    const modes = [branding.light, branding.dark];
    let selected = $state(modes[0]);
    const mode = $derived(selected === modes[0] ? 'light' : 'dark');
    let confirmReset = $state(false);
    let closeReset: (() => void) | undefined = $state();
    let pending = $state(false);
    const formId = genKey();

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

        pending = true;
        err = '';
        let res = await fetchPut(url, payload);
        pending = false;
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
        pending = true;
        err = '';
        const res = await fetchDelete(url);
        if (res.error) {
            err = res.error.message;
        } else {
            closeReset?.();
            await fetchTheme();
        }
        pending = false;
    }

    function onUploadSuccess() {
        logoKey = genKey();
    }

    function onFaviconUploadSuccess() {
        faviconLoaded = false;
        faviconKey = genKey();
    }
</script>

<div class="branding-layout">
    <div class="assets">
        <Section title={branding.assets}>
            <p>{branding.assetsImmediate}</p>
            <div class="asset-grid">
                <div>
                    <h4>{branding.logo}</h4>
                    <div class="upload">
                        <InputFile
                            url={`/auth/v1/clients/${client.id}/logo`}
                            ariaLabel={branding.logo}
                            fileName="logo"
                            width="100%"
                            onSuccess={onUploadSuccess}
                        />
                        <EntityDelete
                            url={`/auth/v1/clients/${client.id}/logo`}
                            label={`${t.common.delete} · ${branding.logo}`}
                            name={client.id}
                            iconOnly
                            onDelete={onUploadSuccess}
                        />
                    </div>
                </div>
                <div>
                    <h4>{branding.faviconUpload}</h4>
                    <div class="upload">
                        {#key faviconKey}
                            <img
                                class:loaded={faviconLoaded}
                                src={faviconUrl}
                                alt={branding.faviconPreviewAlt}
                                onload={() => (faviconLoaded = true)}
                                onerror={() => (faviconLoaded = false)}
                            />
                        {/key}
                        <InputFile
                            url={`/auth/v1/clients/${client.id}/favicon`}
                            ariaLabel={branding.faviconUpload}
                            fileName="favicon"
                            width="100%"
                            onSuccess={onFaviconUploadSuccess}
                        />
                        <EntityDelete
                            url={`/auth/v1/clients/${client.id}/favicon`}
                            label={`${t.common.delete} · ${branding.faviconUpload}`}
                            name={client.id}
                            iconOnly
                            onDelete={onFaviconUploadSuccess}
                        />
                    </div>
                </div>
            </div>
        </Section>
    </div>

    {#if theme}
        <Form id={formId} action={url} {onSubmit}>
            <div class="theme-editor">
                <div class="preview">
                    <Section title={branding.preview}>
                        {#key mode}
                            <BrandingPreview
                                {logoUrl}
                                borderRadius={theme.border_radius}
                                theme={theme[mode]}
                                typ={mode}
                            />
                        {/key}
                    </Section>
                </div>

                <div class="settings">
                    <Section title={branding.theme}>
                        <Input
                            label="--border-radius"
                            placeholder="--border-radius"
                            errMsg={ta.validation.css}
                            width="min(12rem, 100%)"
                            bind:value={theme.border_radius}
                            required
                            pattern={PATTERN_CSS_VALUE_LOOSE}
                        />
                        <Tabs tabs={modes} bind:selected />
                        {#if mode === 'light'}
                            <BrandingMode bind:values={theme.light} />
                        {:else}
                            <BrandingMode bind:values={theme.dark} />
                        {/if}
                    </Section>
                </div>
            </div>
        </Form>

        <div class="save-bar">
            <Button type="submit" form={formId} isLoading={pending}>{t.common.save}</Button>
            <Button level={2} onclick={() => (confirmReset = true)} isDisabled={pending}>
                {branding.resetTheme}
            </Button>
            {#if success}<IconCheck />{/if}
            {#if err && !confirmReset}<span class="err" role="alert">{err}</span>{/if}
        </div>

        <Modal
            bind:showModal={confirmReset}
            bind:closeModal={closeReset}
            ariaLabel={branding.resetTheme}
            strict
        >
            <div class="reset">
                <h3>{branding.resetTheme}</h3>
                <p>{branding.resetThemeConfirm}</p>
                {#if err}<p class="err" role="alert">{err}</p>{/if}
                <div class="reset-actions">
                    <Button level={2} onclick={() => closeReset?.()}>{t.common.cancel}</Button>
                    <Button level={-1} onclick={reset} isLoading={pending}
                        >{branding.resetTheme}</Button
                    >
                </div>
            </div>
        </Modal>
    {:else if err}
        <div class="err" role="alert">{err}</div>
    {/if}
</div>

<style>
    .branding-layout {
        container-type: inline-size;
        margin-bottom: calc(-1 * var(--admin-detail-padding-bottom, 0px));
    }

    .theme-editor {
        display: grid;
        gap: var(--ui-space);
        grid-template-areas: 'preview' 'settings';
        align-items: start;
    }

    .preview {
        grid-area: preview;
    }

    .settings {
        grid-area: settings;
        --ui-tabs-margin: 0;
        --ui-field-margin: 0;
    }

    .save-bar {
        position: sticky;
        bottom: calc(-1 * var(--admin-detail-padding-bottom, 0px));
        z-index: 2;
        display: flex;
        flex-wrap: wrap;
        align-items: center;
        gap: 0.5rem;
        margin-top: var(--ui-space);
        padding: 0.75rem 0;
        border-top: 1px solid var(--ui-outline);
        background: var(--ui-surface);
    }

    .assets {
        margin-bottom: var(--ui-space);
    }

    .assets p,
    .assets h4,
    .reset h3 {
        margin: 0;
    }

    .assets h4 {
        margin-bottom: 0.5rem;
        font-size: var(--ui-control-font-size);
    }

    .asset-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(min(100%, 16rem), 1fr));
        gap: var(--ui-space);
    }

    .upload {
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }

    .upload img {
        display: none;
        width: 2rem;
        height: 2rem;
        object-fit: contain;
    }

    .upload img.loaded {
        display: block;
    }

    .reset {
        width: min(24rem, calc(100dvw - 4rem));
    }

    .reset-actions {
        display: flex;
        flex-wrap: wrap;
        justify-content: end;
        gap: 0.5rem;
        margin-top: var(--ui-space);
    }

    @container (width >= 45rem) {
        .theme-editor {
            grid-template-areas: 'settings preview';
            grid-template-columns: minmax(0, 1fr) minmax(0, 20rem);
        }

        .preview {
            position: sticky;
            top: 0;
        }
    }
</style>
