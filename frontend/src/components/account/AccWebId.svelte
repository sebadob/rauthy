<script>
    import Button from "$lib/Button.svelte";
    import {fade, slide} from 'svelte/transition';
    import {putUserWebIdData} from "../../utils/dataFetching.js";
    import Switch from "$lib/Switch.svelte";
    import AccWebIdEntries from "./AccWebIdEntries.svelte";

    export let t;
    export let webIdData;
    export let viewModePhone;

    const btnWidth = "12rem";

    let err = '';
    let success = false;
    let successEmailConfirm = false;

    // do not set any value here - will be bound to validate function in <AccWebIdEntries/>
    let getData;
    let validateData;

    $: webIdLink = webIdData.is_open ?
        `${window.location.origin}/auth/v1/users/${webIdData.user_id}/webid`
        : '';

    $: if (success) {
        setTimeout(() => {
            success = false;
        }, 3000);
    }

    async function onSubmit() {
        err = '';

        let data = getData();
        console.log(data);

        const isValid = validateData();
        console.log(isValid);
        if (!isValid) {
            return;
        }

        const payload = {
            user_id: webIdData.user_id,
            is_open: webIdData.is_open,
            data,
        };

        console.log(payload);

        let res = await putUserWebIdData(payload.user_id, payload);
        if (res.ok) {
            success = true;
        } else {
            let body = await res.json();
            err = body.message;
        }
    }

</script>

<div class="wrapper">
    <div class="container">
        <p>{t.webIdDesc}</p>

        <div class="row">
            <div class="label">
                {t.webIdActive}
            </div>
            <Switch bind:selected={webIdData.is_open}/>
        </div>

        {#if webIdData.is_open}
            <div transition:slide>
                <p>{t.webIdLinkText}</p>
                <p><a href={webIdLink} target="_blank">{webIdLink}</a></p>

                <!-- Custom Data -->
                <p>{t.webIdDescData}</p>
                <AccWebIdEntries
                        bind:t
                        bind:webIdData
                        bind:viewModePhone
                        bind:getData
                        bind:validateData
                />
            </div>
        {/if}

        <Button width={btnWidth} on:click={onSubmit} level={1}>
            {t.save.toUpperCase()}
        </Button>

        <div class="bottom">
            {#if success}
                <div class="success" transition:fade>
                    Update successful
                </div>
            {:else if err}
                <div class="err" transition:fade>
                    {err}
                </div>
            {/if}
        </div>
    </div>
</div>

<style>
    p {
        margin: 0 .5rem .5rem .5rem;
    }

    .container {
        padding: 0 5px;
        display: flex;
        flex-direction: column;
    }

    .err {
        width: 230px;
        margin: -5px 10px 0 35px;
        padding-right: 5px;
        color: var(--col-err);
    }

    .err {
        margin: 5px;
        color: var(--col-err);
    }

    .bottom {
        height: 1em;
    }

    .row {
        margin: .5rem;
        display: flex;
        gap: .5rem;
    }

    .success {
        margin: 5px;
        color: var(--col-ok);
    }

    .wrapper {
        margin-top: 10px;
        display: flex;
        flex-direction: row;
    }
</style>
