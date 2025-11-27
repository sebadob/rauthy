<script lang="ts">
    import { useI18nAdmin } from '$state/i18n_admin.svelte';
    import { useMarkdownWorker } from '$lib/text_edit/useWorker.svelte';

    let {
        content,
    }: {
        content: string;
    } = $props();

    let ta = useI18nAdmin();

    let worker = useMarkdownWorker();

    $effect(() => {
        worker.sanitizeHTML(content);
    });
</script>

<iframe
    title={ta.common.preview}
    class="md-global"
    srcdoc={worker.sanitizedHTML()}
    sandbox="allow-popups allow-popups-to-escape-sandbox allow-same-origin"
></iframe>

<style>
    iframe {
        width: 100%;
        height: 100%;
        flex: 1;
        border: 1px solid hsl(var(--bg-high));
        border-radius: var(--border-radius);
        overflow: auto;
    }
</style>
