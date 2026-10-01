<script lang="ts">
    import { genKey } from '$utils/helpers';

    let {
        name,
        value = $bindable(),
        label = '',
        disabled = false,
    }: {
        name?: string;
        value: string;
        label: string;
        disabled?: boolean | null | undefined;
    } = $props();

    const id = genKey();
</script>

<input
    type="color"
    {id}
    {name}
    title={label}
    aria-label={label}
    bind:value
    {disabled}
    aria-disabled={disabled}
/>

<style>
    input {
        width: 100%;
        height: 100%;
        margin: 0;
        padding: 0;
        border: none;
        background: none;
        cursor: pointer;
    }

    input::-webkit-color-swatch-wrapper {
        padding: 0;
    }

    input::-webkit-color-swatch {
        border: none;
    }

    input::-moz-color-swatch {
        border: none;
    }

    input[aria-disabled='true'] {
        cursor: not-allowed;
    }
</style>
