<script lang="ts">
    import { useI18nAdmin } from '$state/i18n_admin.svelte';
    import Form from '$lib/form/Form.svelte';
    import Button from '$lib/button/Button.svelte';
    import { useI18n } from '$state/i18n.svelte';
    import Input from '$lib/form/Input.svelte';
    import type { PamGroupResponse, PamHostCreateRequest, PamHostSimpleResponse } from '$api/types/pam';
    import { PATTERN_LINUX_HOSTNAME } from '$utils/patterns';
    import Options from '$lib/Options.svelte';
    import InputCheckbox from '$lib/form/InputCheckbox.svelte';
    import { fetchPost } from '$api/fetch';
    import { slide } from 'svelte/transition';

    let {
        groups,
        onCreate,
    }: {
        groups: PamGroupResponse[];
        onCreate: (host: PamHostSimpleResponse) => void;
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    let ref: undefined | HTMLInputElement = $state();
    let err = $state('');

    const options = groups.filter(g => g.typ === 'host').map(g => g.name);
    let groupName = $state(options[0]);

    let host: PamHostCreateRequest = $state({
        hostname: '',
        gid: 0,
        force_mfa: true,
        local_password_only: false,
    });

    $effect(() => {
        requestAnimationFrame(() => {
            ref?.focus();
        });
    });

    async function onSubmit(form: HTMLFormElement, params: URLSearchParams) {
        let gid = groups.find(g => g.name === groupName)?.id;
        if (!gid) {
            console.error('gid not found');
            return;
        }
        err = '';

        host.gid = gid;
        let res = await fetchPost<PamHostSimpleResponse>(form.action, host);
        if (res.error) {
            if (res.error.message.includes('UNIQUE constraint')) {
                err = ta.pam.nameExistsAlready;
            } else {
                err = res.error.message;
            }
        } else if (res.body) {
            onCreate(res.body);
        }
    }
</script>

<div class="container">
    <h1>{ta.pam.addHost}</h1>

    <Form
        action="/auth/v1/pam/hosts"
        {onSubmit}
    >
        <Input
            bind:ref
            label="Hostname"
            placeholder="Hostname"
            bind:value={host.hostname}
            required
            pattern={PATTERN_LINUX_HOSTNAME}
            maxLength={61}
        />

        <div class="row">
            <div class="label">
                {ta.pam.groupName}
            </div>
            <Options
                ariaLabel={ta.pam.groupName}
                {options}
                bind:value={groupName}
            />
        </div>

        <div class="row">
            <div class="label">
                {ta.clients.forceMfa}
            </div>
            <InputCheckbox
                ariaLabel={ta.clients.forceMfa}
                bind:checked={host.force_mfa}
            />
        </div>

        <div class="row">
            <div class="label">
                {ta.pam.hostLocalPwdOnly}
            </div>
            <InputCheckbox
                ariaLabel={ta.pam.hostLocalPwdOnly}
                bind:checked={host.local_password_only}
            />
        </div>
        {#if host.local_password_only}
            <div
                class="err"
                transition:slide={{ duration: 150 }}
            >
                <p>
                    {ta.pam.hostLocalPwdOnlyInfo}
                </p>
            </div>
        {/if}

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

    p {
        max-width: 20rem;
    }

    .btn {
        margin-top: 1rem;
    }

    .container {
        width: 20rem;
        text-align: left;
    }

    .label {
        margin-bottom: -0.2rem;
        color: hsla(var(--text) / 0.8);
    }

    .row {
        margin: 0.5rem 0;
        display: grid;
        grid-template-columns: 12rem 5.5rem;
        align-items: center;
    }
</style>
