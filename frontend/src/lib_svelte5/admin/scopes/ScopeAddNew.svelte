<script lang="ts">
    import Button from "$lib5/button/Button.svelte";
    import Input from "$lib5/form/Input.svelte";
    import {PATTERN_GROUP} from "$utils/patterns.ts";
    import Form from "$lib5/form/Form.svelte";
    import type {ScopeRequest, ScopeResponse} from "$api/types/scopes.ts";
    import {fetchPost} from "$api/fetch.ts";
    import {useI18n} from "$state/i18n.svelte.ts";
    import {useI18nAdmin} from "$state/i18n_admin.svelte.ts";

    let {
        onSave,
    }: {
        onSave: (id: string) => void,
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    let ref: undefined | HTMLInputElement = $state();

    let err = $state('');
    let name = $state('');

    $effect(() => {
        requestAnimationFrame(() => {
            ref?.focus();
        });
    });

    async function onSubmit(form: HTMLFormElement, params: URLSearchParams) {
        err = '';

        let payload: ScopeRequest = {
            scope: name,
        };
        let res = await fetchPost<ScopeResponse>(form.action, payload);
        if (res.body) {
            onSave(res.body.id);
        } else {
            err = res.error?.message || 'Error';
        }
    }
</script>

<div class="container">
    <Form action="/auth/v1/scopes" {onSubmit}>
        <Input
                bind:ref
                bind:value={name}
                autocomplete="off"
                label={ta.scopes.name}
                placeholder={ta.scopes.name}
                required
                pattern={PATTERN_GROUP}
        />

        <Button type="submit">
            {t.common.save}
        </Button>

        {#if err}
            <div class="err">
                {err}
            </div>
        {/if}
    </Form>
</div>

<style>
    .container {
        min-height: 7rem;
        text-align: left;
    }
</style>
