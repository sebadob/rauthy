<script lang="ts">
    import { useTheme } from '$state/theme.svelte';

    let {
        width = '7rem',
        show,
    }: {
        width?: string;
        show?: 'light' | 'dark';
    } = $props();

    let theme = useTheme();
</script>

<div style:width style:height={width}>
    {#if show === 'dark' || (show === undefined && theme.isDark())}
        <img src="/auth/v1/assets/rauthy_light.svg" alt="Rauthy Logo" width="100%" height="100%" />
    {:else}
        <img src="/auth/v1/assets/rauthy_dark.svg" alt="Rauthy Logo" width="100%" height="100%" />
    {/if}
</div>
