<script lang="ts">
    import EntityHeader from '#lib/admin/EntityHeader.svelte';
    import EntityDelete from '#lib/admin/EntityDelete.svelte';
    import IconCommandLine from '#icons/IconCommandLine.svelte';
    import type {
        PamGroupResponse,
        PamHostDetailsResponse,
        PamHostSimpleResponse,
        PamHostUpdateRequest,
    } from '#api/types/pam.ts';
    import { fetchGet, fetchPut } from '#api/fetch.ts';
    import LabeledValue from '#lib/LabeledValue.svelte';
    import { PATTERN_LINUX_HOSTNAME } from '#utils/patterns.ts';
    import Input from '#lib/form/Input.svelte';
    import Options from '#lib/Options.svelte';
    import InputCheckbox from '#lib/form/InputCheckbox.svelte';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import InputArea from '#lib/form/InputArea.svelte';
    import InputTags from '#lib/form/InputTags.svelte';
    import Form from '#lib/form/Form.svelte';
    import Section from '#lib/Section.svelte';
    import Button from '#lib/button/Button.svelte';
    import { slide } from 'svelte/transition';
    import { useI18n } from '#state/i18n.svelte.ts';
    import IconCheck from '#icons/IconCheck.svelte';
    import PAMHostSecret from '#lib/admin/pam/hosts/PAMHostSecret.svelte';
    import { untrack } from 'svelte';

    let {
        hostSimple,
        groups,
        onDelete,
    }: {
        hostSimple: PamHostSimpleResponse;
        groups: PamGroupResponse[];
        onDelete: () => void;
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    const groupNames = untrack(() => groups.filter(g => g.typ === 'host').map(g => g.name));

    let success = $state(false);
    let err = $state('');
    let url = $derived(`/auth/v1/pam/hosts/${hostSimple.id}`);

    let host: undefined | PamHostDetailsResponse = $state();
    let groupName = $state(untrack(() => groups.find(g => g.id === host?.gid)?.name || ''));

    $effect(() => {
        fetchDetails();
    });

    async function fetchDetails() {
        let res = await fetchGet<PamHostDetailsResponse>(url);
        if (res.body) {
            if (res.body.notes) {
                res.body.notes = res.body.notes.trim();
            } else {
                res.body.notes = '';
            }
            host = res.body;
            groupName = groups.find(g => g.id === host?.gid)?.name || '';
        } else {
            err = res.error?.message || 'ERROR';
        }
    }

    async function onSubmit(form: HTMLFormElement, params: URLSearchParams) {
        err = '';

        let gid = groups.find(g => g.name === groupName)?.id;
        if (!gid || !host) {
            console.error('gid not found');
            return;
        }

        let notes = host.notes?.trim();
        let payload: PamHostUpdateRequest = {
            hostname: host.hostname,
            gid,
            force_mfa: host.force_mfa,
            local_password_only: host.local_password_only,
            notes: notes ? notes : undefined,
            ips: host.ips,
            aliases: host.aliases,
        };

        // The only reason we `try` here, is that we don't want to maintain a huge and error-prone
        // IPv4+IPv6 regex in the UI side. The backend parses into `IpAddr` and the only reason we
        // can `catch` here is because of an invalid IP address.
        try {
            let res = await fetchPut(form.action, payload);
            if (res.error) {
                err = res.error.message;
            } else if (res.status === 422) {
                err = 'Invalid IP Address';
            } else {
                success = true;
                setTimeout(() => {
                    success = false;
                }, 3000);
            }
        } catch (e) {
            err = 'Invalid IP Address';
        }
    }
</script>

<EntityHeader title={hostSimple.name} subtitle="PAM">
    {#snippet actions()}
        {#key hostSimple.id}
            <EntityDelete {url} name={hostSimple.name} message={ta.pam.deleteHost} {onDelete} />
        {/key}
    {/snippet}
    <IconCommandLine width="28px" />
</EntityHeader>

{#if host}
    <Form action={url} {onSubmit}>
        <div class="sections">
            <Section title={t.account.navInfo}>
                {#snippet actions()}<PAMHostSecret hostId={hostSimple.id} />{/snippet}
                <LabeledValue label="Host ID" copyToClip={host.id} mono>{host.id}</LabeledValue>
                <div class="ui-field-grid">
                    <Input
                        label="Hostname"
                        placeholder="Hostname"
                        bind:value={host.hostname}
                        required
                        pattern={PATTERN_LINUX_HOSTNAME}
                        maxLength={61}
                    />
                    <div>
                        <span class="admin-field-label">{ta.pam.groupName}</span>
                        <Options
                            ariaLabel={ta.pam.groupName}
                            options={groupNames}
                            bind:value={groupName}
                        />
                    </div>
                </div>
                <div class="checks">
                    <InputCheckbox ariaLabel={ta.clients.forceMfa} bind:checked={host.force_mfa}>
                        {ta.clients.forceMfa}
                    </InputCheckbox>
                    <InputCheckbox
                        ariaLabel={ta.pam.hostLocalPwdOnly}
                        bind:checked={host.local_password_only}
                    >
                        {ta.pam.hostLocalPwdOnly}
                    </InputCheckbox>
                </div>
                {#if host.local_password_only}
                    <p class="err" transition:slide={{ duration: 150 }}>
                        {ta.pam.hostLocalPwdOnlyInfo}
                    </p>
                {/if}
            </Section>
            <Section title={ta.pam.ipAddresses}>
                <div class="ui-field-grid">
                    <InputTags label={ta.pam.ipAddresses} bind:values={host.ips} />
                    <InputTags
                        label={ta.pam.hostAliases}
                        bind:values={host.aliases}
                        pattern={PATTERN_LINUX_HOSTNAME}
                    />
                </div>
                <InputArea label={ta.pam.notes} bind:value={host.notes} />
            </Section>
        </div>

        <div class="submit flex gap-05">
            <Button type="submit">
                {t.common.save}
            </Button>

            {#if success}
                <IconCheck />
            {/if}
        </div>
    </Form>
{/if}

{#if err}
    <div class="err">
        {err}
    </div>
{/if}

<style>
    .sections {
        display: grid;
        gap: var(--ui-space);
    }

    .checks {
        display: flex;
        flex-wrap: wrap;
        align-items: center;
        gap: 0.5rem 1rem;
    }

    .submit {
        margin-block: var(--ui-space);
    }
</style>
