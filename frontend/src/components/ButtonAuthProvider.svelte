<script lang="ts">
    import Button from "$lib5/button/Button.svelte";
    import {useI18n} from "$state/i18n.svelte.ts";
    import type {AuthProviderTemplate} from "$api/templates/AuthProvider.ts";

    let {
        ariaLabel,
        provider,
        onclick,
    }: {
        ariaLabel: string,
        provider: AuthProviderTemplate,
        onclick: (providerId: string) => void,
    } = $props();

    let t = useI18n();

    let showIcon = $state(false);
</script>

<Button {ariaLabel} level={2} onclick={() => onclick(provider.id)}>
    <div class="inline">
        {#if showIcon}
            <img
                    src="{`/auth/v1/providers/${provider.id}/img`}"
                    alt=""
                    width="20"
                    height="20"
                    onload={() => showIcon = true}
            />
        {/if}
        <span class="name">
            {provider.name}
        </span>
    </div>
</Button>

<style>
    .inline {
        display: flex;
        justify-content: center;
        gap: .5rem;
    }

    .name {
        margin-bottom: -.1rem;
    }
</style>