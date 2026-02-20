<script lang="ts">
    import { onMount } from 'svelte';
    import { fetchGet } from '$api/fetch';
    import type { AppVersionResponse } from '$api/types/api_version.ts';

    let version: undefined | AppVersionResponse = $state();

    onMount(() => {
        fetchVersion();
    });

    async function fetchVersion() {
        let res = await fetchGet<AppVersionResponse>('/auth/v1/version');
        if (res.body) {
            version = res.body;
        } else {
            console.error(res.error);
        }
    }
</script>

{#if version}
    {#if version.update_available}
        <div class="ver upd">
            <a href={version.latest_url} target="_blank">
                v{version.current} ⚠️
            </a>
        </div>
    {:else}
        <div class="ver">
            v{version.current}
        </div>
    {/if}
{/if}

<style>
    .upd {
        cursor: pointer;
    }

    .ver {
        margin: 0.1rem 0.5rem;
        display: flex;
        align-items: center;
        font-size: 0.85rem;
        color: hsla(var(--text) / 0.5);
    }

    .ver > a {
        text-decoration: none;
        color: hsla(var(--text) / 0.65);
    }
</style>
