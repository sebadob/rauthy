<script>
    import {onMount} from "svelte";
    import FederationTileAddNew from "./ProviderTileAddNew.svelte";
    import ProviderTile from "./ProviderTile.svelte";
    import {postProviders} from "../../../utils/dataFetchingAdmin.js";

    let err = '';
    let providers = [];

    onMount(async () => {
        fetchData();
    });

    async function fetchData() {
        let res = await postProviders();
        let body = await res.json();
        if (res.ok) {
            providers = [...body];
        } else {
            err = body.message;
        }
    }

    function onSave() {
        fetchData();
    }

</script>

{err}

<div class="content">
    <FederationTileAddNew onSave={onSave}/>

    <div id="federation">
        {#each providers as provider (provider.id)}
            <div>
                <ProviderTile bind:provider onSave={onSave}/>
            </div>
        {/each}
    </div>

    <div style="height: 20px"></div>
</div>

<style>
    #federation div:nth-of-type(2n + 1) {
        background: linear-gradient(90deg, var(--col-ghigh) 35rem, var(--col-bg) 50rem);
    }
</style>
