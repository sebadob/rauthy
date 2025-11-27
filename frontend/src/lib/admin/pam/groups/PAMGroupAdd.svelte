<script lang="ts">
    import { useI18nAdmin } from '$state/i18n_admin.svelte';
    import Form from '$lib/form/Form.svelte';
    import Button from '$lib/button/Button.svelte';
    import { useI18n } from '$state/i18n.svelte';
    import Input from '$lib/form/Input.svelte';
    import type {
        PamGroupCreateRequest,
        PamGroupResponse,
        PamGroupType,
        PamHostCreateRequest,
        PamHostSimpleResponse,
    } from '$api/types/pam';
    import { PATTERN_LINUX_HOSTNAME, PATTERN_LINUX_USERNAME } from '$utils/patterns';
    import Options from '$lib/Options.svelte';
    import InputCheckbox from '$lib/form/InputCheckbox.svelte';
    import { fetchPost } from '$api/fetch';

    let {
        groups,
        onCreate,
    }: {
        groups: PamGroupResponse[];
        onCreate: (group: PamGroupResponse) => void;
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    let ref: undefined | HTMLInputElement = $state();
    let err = $state('');

    const options: PamGroupType[] = ['host', 'generic', 'local'];

    let group: PamGroupCreateRequest = $state({
        name: '',
        typ: options[0],
    });

    $effect(() => {
        requestAnimationFrame(() => {
            ref?.focus();
        });
    });

    function isNameTaken() {
        let isTaken = groups.find(g => g.name === group.name);

        if (isTaken) {
            err = ta.pam.nameExistsAlready;
            return true;
        }
        return false;
    }

    async function onSubmit(form: HTMLFormElement, params: URLSearchParams) {
        if (isNameTaken()) {
            return;
        }
        err = '';

        let res = await fetchPost<PamGroupResponse>(form.action, group);
        if (res.error) {
            err = res.error.message;
        } else if (res.body) {
            onCreate(res.body);
        }
    }
</script>

<div class="container">
    <h1>{ta.pam.addGroup}</h1>

    <Form action="/auth/v1/pam/groups" {onSubmit}>
        <Input
            bind:ref
            label={ta.common.name}
            placeholder={ta.common.name}
            bind:value={group.name}
            required
            pattern={PATTERN_LINUX_USERNAME}
            width="min(22rem, 100%)"
            onBlur={isNameTaken}
        />

        <div class="row">
            <div class="label">
                {ta.pam.groupType}
            </div>
            <Options ariaLabel={ta.pam.groupType} {options} bind:value={group.typ} />
        </div>

        <p>
            {#if group.typ === 'host'}
                {ta.pam.groupDescHost}
            {:else if group.typ === 'generic'}
                {ta.pam.groupDescGeneric}
            {:else if group.typ === 'local'}
                {ta.pam.groupDescLocal}
            {/if}
        </p>

        <div class="btn">
            <Button type="submit">
                {t.common.save}
            </Button>
        </div>

        {#if err}
            <div clasS="err">
                {err}
            </div>
        {/if}
    </Form>
</div>

<style>
    h1 {
        font-size: 1.1rem;
    }

    .btn {
        margin-top: 1rem;
    }

    .container {
        width: min(30rem, calc(100dvw - 2rem));
        text-align: left;
    }

    .label {
        color: hsla(var(--text) / 0.8);
    }

    .row {
        margin: 0.5rem 0;
        display: grid;
        grid-template-columns: 7rem 5rem;
        align-items: center;
    }
</style>
