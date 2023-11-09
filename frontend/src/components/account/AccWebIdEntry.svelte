<script>
    import * as yup from "yup";
    import {REGEX_NAME} from "../../utils/constants.js";
    import Input from "$lib/inputs/Input.svelte";
    import IconStop from "$lib/icons/IconStop.svelte";

    export let t;

    export let entry = {
        key: '',
        value: '',
    };
    export let isKeyUnique;
    export let removeKey;
    export let viewModePhone = false;

    $: inputWidth = viewModePhone ? 'calc(100vw - 1.5rem)' : '350px';
    $: inputWidthValue = viewModePhone ? 'calc(100vw - 1.5rem)' : 'calc(350px + 3rem)';

    let formErrors = {};
    const schema = yup.object().shape({
        key: yup.string()
            .required('*')
            .email(t.validEmail),
        value: yup.string()
            .required('*')
            .matches(REGEX_NAME, t.validGivenName),
    });

    async function validateForm() {
        formErrors = {};

        if (!isKeyUnique(entry.key)) {
            formErrors.key = t.keyUnique;
        }
        // TODO
    }

</script>

<div class="entry">
    <div class="row1">
        <Input
                bind:value={entry.key}
                bind:error={formErrors.key}
                autocomplete="off"
                placeholder={t.key}
                on:input={validateForm}
                width={inputWidth}
        >
            {t.key.toUpperCase()}
        </Input>

        <div
                role="button"
                tabindex="0"
                class="delete"
                on:click={() => removeKey(entry.key)}
                on:keypress={() => removeKey(entry.key)}
        >
            <IconStop color="var(--col-err)" width="1.75rem" />
        </div>
    </div>

    <Input
            bind:value={entry.value}
            bind:error={formErrors.value}
            autocomplete="off"
            placeholder={entry.key}
            on:input={validateForm}
            width={inputWidthValue}
    >
        {entry.key}
    </Input>
</div>

<style>
    .delete {
        margin-top: -.15rem;
        margin-left: .15rem;
        cursor: pointer;
    }

    .entry {
        margin: 1rem 0;
        /*border: 1px solid red;*/
    }

    .row1 {
        display: flex;
        align-items: center;
        gap: 1rem;
    }
</style>
