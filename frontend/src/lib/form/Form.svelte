<script lang="ts">
    import type { Snippet } from 'svelte';
    import { fetchSolvePow } from '$utils/pow';
    import { unixTsFromLocalDate } from '$utils/form';
    import { errorFromResponse, fetchForm } from '$api/fetch';

    type T = $$Generic;

    let {
        action,
        method = 'POST',
        isError = $bindable(),
        // the given keys will be converted from date inputs to Unix TS in seconds
        convertDateToTs = [],
        resetOnSubmit,
        // if left empty, the form will handle the request itself
        onSubmit,
        // called after a successful request, will be ignored if onSubmit() is given
        onResponse,
        children,
        // fetch, solve and append a PoW with the given key
        withPowAs,
    }: {
        action: string;
        method?: 'GET' | 'POST';
        isError?: boolean;
        convertDateToTs?: string[];
        resetOnSubmit?: boolean;
        onSubmit?: (form: HTMLFormElement, params: URLSearchParams) => void;
        onResponse?: (res: Response) => void;
        children: Snippet;
        withPowAs?: string;
    } = $props();

    async function submit(ev: SubmitEvent & { currentTarget: EventTarget & HTMLFormElement }) {
        ev.preventDefault();
        const form = ev.currentTarget;

        const isValid = form.reportValidity();
        if (isValid) {
            isError = false;
        } else {
            isError = true;
            return;
        }

        const formData = new FormData(form);

        for (let key of convertDateToTs) {
            let data = formData.get(key)?.toString() || '';
            let ts = unixTsFromLocalDate(data) || '';
            formData.set(key, ts?.toString() || '');
        }

        if (withPowAs) {
            let pow = await fetchSolvePow();
            formData.set(withPowAs, pow || '');
        }

        let resp;
        let params = new URLSearchParams();
        formData.forEach((value, key) => {
            params.append(key, value.toString());
        });

        if (onSubmit) {
            onSubmit(form, params);
            return;
        }

        resp = await fetchForm(form, params);

        if (resp) {
            if (!resp.ok) {
                await errorFromResponse(resp, true);
            } else if (onResponse) {
                onResponse(resp);
                if (resetOnSubmit && resp.ok) {
                    form.reset();
                }
            }
        }
    }
</script>

<form
    {action}
    {method}
    onsubmit={submit}
>
    {@render children()}
</form>
