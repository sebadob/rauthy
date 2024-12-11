<script>
    import {onMount} from "svelte";
    import {getAttr, getUserAttr, updateUserAttr} from "../../../utils/dataFetchingAdmin.js";
    import Button from "$lib/Button.svelte";
    import Input from "$lib/inputs/Input.svelte";

    let {onSave = $bindable(), user = $bindable()} = $props();

    let err = $state('');
    let attr = $state([]);
    let success = $state(false);
    let isLoading = $state(false);

    onMount(async () => {
        fetchBuildAttr();
    })

    async function fetchAttr() {
        let res = await getAttr();
        if (!res.ok) {
            err = 'Error fetching attr: ' + res.body.message;
        } else {
            return await res.json();
        }
    }

    async function fetchUserAttr() {
        let res = await getUserAttr(user.id);
        if (!res.ok) {
            err = 'Error fetching user attr: ' + res.body.message;
        } else {
            return await res.json();
        }
    }

    async function fetchBuildAttr() {
        let attrs = await fetchAttr();
        let all = attrs.values;
        if (attrs) {
            let userAttrs = await fetchUserAttr();
            if (userAttrs) {
                let uaMap = new Map();
                for (let ua of userAttrs.values) {
                    uaMap.set(ua.key, ua.value || '');
                }

                for (let a of all) {
                    let uaValue = uaMap.get(a.name);
                    if (uaValue) {
                        a.value = uaValue;
                    } else {
                        a.value = '';
                    }
                }

                attr = [...all];
            }
        }
    }

    async function onSubmit() {
        err = '';
        isLoading = true;

        let values = [];
        for (let a of attr) {
            let v = {
                key: a.name,
                value: a.value,
            }
            values.push(v);
        }

        let data = {
            values
        };

        let res = await updateUserAttr(user.id, data);
        let body = await res.json();
        if (res.ok) {
            success = true;
        } else {
            err = body.message;
        }

        onSave();

        isLoading = false;
    }

</script>

<div class="container">
    <div class="desc">
        Set custom user attributes.<br>
        All Key / Value pairs will be handled as String / String.
    </div>

    {#each attr as a}
        <Input
                style="width: 350px"
                bind:value={a.value}
                autocomplete="off"
                placeholder={a.desc || 'JSON Value'}
        >
            {a.name}
        </Input>
    {/each}

    <Button on:click={onSubmit} isLoading={isLoading} level={1} width="4rem">SAVE</Button>

    {#if success}
        <div class="success">
            Success
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

    .desc {
        margin: 5px 7px;
    }

    .err {
        color: var(--col-err);
    }

    .err, .success {
        margin: 0 7px;
    }

    .success {
        color: var(--col-ok);
    }
</style>
