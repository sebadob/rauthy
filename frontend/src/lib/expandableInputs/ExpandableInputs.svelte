<script>
    import DynamicInputRow from "./DynamicInput.svelte";
    import {createEventDispatcher, onMount, tick} from "svelte";
    import {getKey} from "../utils/helpers.js";

    export let validation = {
        required: false,
        regex: undefined,
        errMsg: '',
    };
    export let values = [];
    export let width = '260px';
    export let optional = false;
    export let autocomplete = 'on';

    let err = '';
    let inputs = [];

    const dispatch = createEventDispatcher();

    onMount(() => {
        for (let value of values) {
            inputs.push({
                name: getKey(),
                value,
            });
        }
        inputs.push({
            name: getKey(),
            value: '',
        });
    })

    async function handleInput() {
        await tick();

        if (inputs[inputs.length - 1].value) {
            inputs.push({
                name: getKey(),
                value: '',
            });
        } else if (inputs.length > 1 && !inputs[inputs.length - 2].value) {
            inputs = [...inputs.slice(0, inputs.length - 1)];
        }

        dispatch('input', true);
        validate();
        values = getValues();
    }

    // can be called from the outside to get the values as an array
    export function getValues() {
        validate();
        let res = [];
        for (let i = 0; i < inputs.length - 1; i++) {
            res.push(inputs[i].value);
        }
        return res;
    }

    // can be called from the outside to validate every input and returns true, if everything is ok
    export function validate() {
        if (validation.required && inputs.length === 1 && !inputs[0].value) {
            if (optional) {
                return true;
            }

            err = 'Required';
            return false;
        }

        let res = true;
        for (let i of inputs) {
            if (i.value && !i.validate()) {
                res = false;
            }
        }

        return res;
    }

</script>

<div class="container" style:width>
    {#each inputs as input}
        <DynamicInputRow
                width={"calc({width} - 20px)"}
                bind:validation
                bind:name={input.name}
                bind:value={input.value}
                bind:validate={input.validate}
                bind:autocomplete
                on:input={handleInput}
                on:blur={handleInput}
                {...$$restProps}
        >
            <slot></slot>
        </DynamicInputRow>
    {/each}

    {#if err}
        <div class="err">
            {err}
        </div>
    {/if}
</div>

<style>
    .container {
        margin: 5px 0;
    }

    .err {
        margin-top: -10px;
        margin-left: 5px;
        padding: 0 10px;
        font-size: .85rem;
        color: var(--col-err);
    }
</style>
