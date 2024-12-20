<script>
    import {createEventDispatcher, tick} from "svelte";
    import {fade} from "svelte/transition";
    import Input from "../inputs/Input.svelte";

    /**
     * @typedef {Object} Props
     * @property {any} [validation]
     * @property {any} name
     * @property {any} value
     * @property {any} width
     * @property {string} [autocomplete]
     * @property {import('svelte').Snippet} [children]
     */

    /** @type {Props & { [key: string]: any }} */
    let {
        validation = {},
        name,
        value = $bindable(),
        width = $bindable(),
        autocomplete = $bindable('on'),
        children,
        validate = $bindable(),
        ...rest
    } = $props();
    
    validate = validateValue;

    let error = $state('');

    const dispatch = createEventDispatcher();

    async function handleInput() {
        await tick();
        dispatch('input', true);
        validateValue();
    }

    async function handleBlur() {
        dispatch('blur', true);
        validateValue();
    }

    export function validateValue() {
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
            {width}
            {name}
            bind:value
            bind:error
            {autocomplete}
            on:input={handleInput}
            on:blur={handleBlur}
            {...rest}
    >
        {@render children?.()}
    </Input>
</div>
