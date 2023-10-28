<script>
    import {onMount} from "svelte";
    import {getAppVersion} from "../utils/dataFetching.js";

    let version = '';
    let lastCheck;
    let latestVersion;
    let newVersionAvailable = false;
    let updateUrl = '';

    onMount(() => {
        fetchVersion();
    })

    async function fetchVersion() {
        let res = await getAppVersion();
        if (res.ok) {
            let body = await res.json();
            version = body.current;
            lastCheck = body.last_check;
            latestVersion = body.latest;
            updateUrl = body.latest_url;
            newVersionAvailable = body.update_available;
        }
    }

</script>

{#if newVersionAvailable}
    <div class="ver upd">
        <a href={updateUrl} target="_blank">
            v{version}⚠️
        </a>
    </div>
{:else}
    <div class="ver">
        v{version}
    </div>
{/if}

<style>
    .upd {
        cursor: pointer;
    }

    .ver {
        margin: 0 .25rem;
        display: flex;
        align-items: center;
        font-size: .85rem;
    }

    .ver > a {
        text-decoration: none;
        color: var(--col-text);
    }
</style>
