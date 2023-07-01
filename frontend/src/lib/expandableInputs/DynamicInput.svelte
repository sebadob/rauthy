<script>
    import {createEventDispatcher, tick} from "svelte";
    import {fade} from "svelte/transition";
    import Input from "../inputs/Input.svelte";

    export let validation = {};
    export let name;
    export let value;
    export let width;
    export let autocomplete = 'on';

    let error = '';

    const dispatch = createEventDispatcher();

    async function handleInput() {
        await tick();
        dispatch('input', true);
        validate();
    }

    async function handleBlur() {
        dispatch('blur', true);
        validate();
    }

    export function validate() {
        error = '';
        if (!value) {
            // 'required' will be validated in the wrapping component, since a single instance has not idea
            // about any possibly other values
            return true;
        }
        if (validation?.regex && !value.match(validation.regex)) {
            error = validation.errMsg || 'Invalid input';
            return false;
        }
        return true;
    }

</script>

<div transition:fade|global="{{ duration: 200 }}">
    <Input
            bind:width
            {name}
            bind:value
            bind:error
            bind:autocomplete
            on:input={handleInput}
            on:blur={handleBlur}
            {...$$restProps}
    >
        <slot></slot>
    </Input>
</div>
