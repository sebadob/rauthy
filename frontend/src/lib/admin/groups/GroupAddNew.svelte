<script lang="ts">
    import Button from '$lib5/button/Button.svelte';
    import Input from '$lib5/form/Input.svelte';
    import { PATTERN_GROUP } from '$utils/patterns';
    import Form from '$lib5/form/Form.svelte';
    import { fetchPost } from '$api/fetch';
    import { useI18n } from '$state/i18n.svelte';
    import { useI18nAdmin } from '$state/i18n_admin.svelte';
    import type { GroupResponse, GroupRequest } from '$api/types/groups.ts';

    let {
        groups,
        onSave,
    }: {
        groups: GroupResponse[];
        onSave: (id: string) => void;
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
        if (groups.find(g => g.name === name)) {
            err = ta.common.nameExistsAlready;
            return;
        }
        err = '';

        let payload: GroupRequest = {
            group: name,
        };
        let res = await fetchPost<GroupResponse>(form.action, payload);
        if (res.body) {
            onSave(res.body.id);
        } else {
            err = res.error?.message || 'Error';
        }
    }
</script>

<div class="container">
    <Form
        action="/auth/v1/groups"
        {onSubmit}
    >
        <Input
            bind:ref
            bind:value={name}
            autocomplete="off"
            label={ta.groups.name}
            placeholder={ta.groups.name}
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
