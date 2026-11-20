<script lang="ts">
    import IconShieldCheck from '#icons/IconShieldCheck.svelte';
    import ImageEdit from '#lib/ImageEdit.svelte';
    import { uploadFile } from '#api/fetch.ts';
    import { useI18n } from '#state/i18n.svelte.ts';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';

    let {
        providerId,
        revision,
        exists = $bindable(false),
        error = $bindable(''),
        onUpload,
    }: {
        providerId: string;
        revision: string;
        exists?: boolean;
        error?: string;
        onUpload: () => void;
    } = $props();

    const t = useI18n();
    const ta = useI18nAdmin();
    const accepted = ['image/jpeg', 'image/png', 'image/svg+xml'];
    const url = $derived(`/auth/v1/providers/${providerId}/img`);
    let files: File[] | undefined = $state();
    let uploading = $state(false);

    $effect(() => {
        const file = files?.[0];
        if (file && !uploading) {
            upload(file);
        }
    });

    async function upload(file: File) {
        files = undefined;
        error = '';
        if (!accepted.includes(file.type)) {
            error = t.common.invalidInput;
            return;
        }

        uploading = true;
        try {
            const result = await uploadFile('PUT', url, file, 'logo');
            if (result.error) {
                error = result.error.message;
            } else {
                onUpload();
            }
        } finally {
            uploading = false;
        }
    }
</script>

<div class="provider-logo">
    {#if !exists}
        <IconShieldCheck width="1.75rem" />
    {/if}
    <img
        src={`${url}?${revision}`}
        alt=""
        class:loaded={exists}
        onload={() => (exists = true)}
        onerror={() => (exists = false)}
    />
    <ImageEdit
        bind:files
        accept={accepted.join(', ')}
        ariaLabel={`${t.common.edit} · ${ta.clients.branding.logo}`}
        disabled={uploading}
    />
</div>

<style>
    .provider-logo {
        position: relative;
        display: grid;
        place-items: center;
        inline-size: 100%;
        block-size: 100%;
    }

    img {
        position: absolute;
        inline-size: 70%;
        block-size: 70%;
        object-fit: contain;
        visibility: hidden;
    }

    img.loaded {
        visibility: visible;
    }
</style>
