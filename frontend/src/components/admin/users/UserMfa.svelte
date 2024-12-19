<script>
    import {onMount} from "svelte";
    import {getUserPasskeys, webauthnDelete} from "../../../utils/dataFetching.js";
    import Button from "$lib/Button.svelte";
    import {formatDateFromTs} from "../../../utils/helpers.js";

    let {user = $bindable({}), onSave = $bindable()} = $props();

    let err = $state('');
    let passkeys = $state([]);

    const btnWidth = "inherit";

    onMount(async () => {
        await fetchPasskeys();
    });

    async function fetchPasskeys() {
        let res = await getUserPasskeys(user.id);
        let body = await res.json();
        if (res.ok) {
            passkeys = body;
        } else {
            console.error('error fetching passkeys: ' + body.message);

        }
    }

    async function handleDeleteKey(name) {
        let isLastKey = passkeys.length === 1;

        let res = await webauthnDelete(user.id, name);
        if (res.status === 200) {
            await fetchPasskeys();

            // if this was the last key, we need to re-fetch the user to show the
            // correct "mfa enabled" status
            if (isLastKey) {
                onSave();
            }

            return true;
        } else {
            let body = await res.json();
            err = body.message;
            return false;
        }
    }

</script>

<div class="container">
    {#if passkeys.length < 1}
        <div class="desc">
            This user does not have any active MFA keys.
        </div>
    {:else}
        <div class="desc">
            You can delete the users MFA / Security Keys.<br>
            Be careful though, since this <b>cannot be reverted</b> without user interaction.<br>
            This is useful, if a user lost his keys and he is not able to log in any more.
        </div>

        <div class="keysContainer">
            {#each passkeys as passkey (passkey.name)}
                <div class="keyContainer">
                    <div class="row">
                        <div class="label">
                            Passkey Name:
                        </div>
                        <b>{passkey.name}</b>
                    </div>
                    <div class="row">
                        <div class="label">
                            Key Registered:
                        </div>
                        <span class="font-mono">{formatDateFromTs(passkey.registered)}</span>
                    </div>
                    <div class="row">
                        <div class="label">
                            Last Usage:
                        </div>
                        <span class="font-mono">{formatDateFromTs(passkey.last_used)}</span>
                    </div>
                    <div class="row">
                        <div class="label"></div>
                        <div class="deleteBtn">
                            <Button on:click={() => handleDeleteKey(passkey.name)} level={4}>
                                DELETE
                            </Button>
                        </div>
                    </div>
                </div>
            {/each}
        </div>
    {/if}

    {#if err}
        <div class="err">
            {err}
        </div>
    {/if}
</div>

<style>
    .container {
        margin: 0 10px 10px 10px;
    }

    .deleteBtn {
        text-align: right;
        margin: -.33rem 0 0 -.8rem;
    }

    .desc {
        margin: 7px;
    }

    .err {
        margin: .5rem;
        color: var(--col-err);
    }

    .keyContainer {
        margin: .33rem 0;
    }

    .keysContainer {
        margin: .5rem;
        gap: .5rem;
        padding-right: 2rem;
        overflow-y: auto;
    }

    .label {
        width: 7rem;
    }

    .row {
        display: flex;
        gap: .5rem;
    }
</style>
