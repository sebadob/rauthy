<script>
    import Button from "$lib/Button.svelte";
    import Input from "$lib/inputs/Input.svelte";

    export let onSave = () => {};

    let err = '';
    let showInputs = true;

    let formValues = {
        ip: '',
        exp: new Date(),
    }
    let formErrors = {};

</script>


{#if showInputs}
    <div class="inpt">
        <Input
                width="9.5rem"
                bind:value={formValues.ip}
                bind:error={formErrors.ip}
                autocomplete="off"
                placeholder="IP"
        >
            IP
        </Input>
        <Input
                type="datetime-local"
                step="60"
                width="18rem"
                bind:value={formValues.exp}
                min={new Date().toISOString().split('.')[0]}
                max="2099-01-01T00:00"
        >
            EXPIRES
        </Input>
    </div>
{:else}
    <Button on:click={() => showInputs = true}>BLACKLIST IP</Button>
{/if}

{err}

<style>
    .inpt {
        display: inline-flex;
        margin-top: -1rem;
        border: 1px solid red;
    }
</style>
