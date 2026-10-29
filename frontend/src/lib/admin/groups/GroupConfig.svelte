<script lang="ts">
    import Button from '#lib/button/Button.svelte';
    import Input from '#lib/form/Input.svelte';
    import IconCheck from '#icons/IconCheck.svelte';
    import {useI18n} from '#state/i18n.svelte.ts';
    import {useI18nAdmin} from '#state/i18n_admin.svelte.ts';
    import {fetchPut} from '#api/fetch.ts';
    import Form from '#lib/form/Form.svelte';
    import LabeledValue from '#lib/LabeledValue.svelte';
    import type {GroupResponse, GroupRequest} from '#api/types/groups.ts';
    import {PATTERN_GROUP} from '#utils/patterns.ts';
    import {untrack} from 'svelte';
    import {parseJsonValue, stringifyJsonValue} from '#utils/jsonValue.ts';
    import InputArea from '#lib/form/InputArea.svelte';

    let {
        group,
        groups,
        onSave,
    }: {
        group: GroupResponse;
        groups: GroupResponse[];
        onSave: () => void;
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    let err = $state('');
    let success = $state(false);
    let name = $state(untrack(() => group.name));
    let jsonMeta = $state(untrack(() => stringifyJsonValue(group.meta) || ''));

    $effect(() => {
        if (group.id) {
            name = group.name;
            jsonMeta = stringifyJsonValue(group.meta) || '';
        }
    });

    async function onSubmit(form: HTMLFormElement, params: URLSearchParams) {
        err = '';

        if (name !== group.name && groups.find(g => g.name === name)) {
            err = ta.common.nameExistsAlready;
            return;
        }

        let payload: GroupRequest = {
            group: name,
            meta: parseJsonValue(jsonMeta),
        };

        let res = await fetchPut(form.action, payload);
        if (res.error) {
            err = res.error.message;
        } else {
            success = true;
            onSave();
            setTimeout(() => {
                success = false;
            }, 2000);
        }
    }
</script>

<Form action={`/auth/v1/groups/${group.id}`} {onSubmit}>
    <LabeledValue label="ID" mono>
        {group.id}
    </LabeledValue>

    <Input
            bind:value={name}
            autocomplete="off"
            label={ta.groups.name}
            placeholder={ta.groups.name}
            width="14.5rem"
            required
            pattern={PATTERN_GROUP}
    />
    <div class="meta">
        <InputArea
                label={ta.common.jsonMeta}
                placeholder={ta.common.jsonMeta}
                rows={15}
                bind:value={jsonMeta}
        />
    </div>

    <div class="btn">
        <Button type="submit">
            {t.common.save}
        </Button>

        {#if success}
            <IconCheck/>
        {/if}
    </div>

    {#if err}
        <div class="err">
            {err}
        </div>
    {/if}
</Form>

<style>
    .btn {
        margin-top: 1rem;
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }

    .meta {
        max-width: 40rem;
    }
</style>
