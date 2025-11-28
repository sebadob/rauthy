<script lang="ts">
    import IconEnvelope from '$icons/IconEnvelope.svelte';
    import Button from '$lib/button/Button.svelte';
    import Modal from '$lib/Modal.svelte';
    import { useI18nAdmin } from '$state/i18n_admin.svelte';
    import Options from '$lib/Options.svelte';
    import EditorInteractive from '$lib/text_edit/EditorInteractive.svelte';
    import type { GroupResponse } from '$api/types/groups';
    import type { RoleResponse } from '$api/types/roles';
    import Input from '$lib/form/Input.svelte';
    import type { EmailContentType, EmailJobFilterType, EmailJobRequest } from '$api/types/email';

    let {
        groups,
        roles,
    }: {
        groups: GroupResponse[];
        roles: RoleResponse[];
    } = $props();

    let ta = useI18nAdmin();

    let showModal = $state(true);
    let closeModal: undefined | (() => void) = $state();

    let editorMode: 'Text' | 'Markdown' | 'HTML' = $state('Markdown');
    let filter = $state(ta.email.filterType[0]);
    let group = $state('');
    let role = $state('');
    let subject = $state('');
    let body = $state('');
    let error = $state('');

    let isFilterNone = $derived(filter === ta.email.filterType[0]);
    let isFilterGroup = $derived(
        filter === ta.email.filterType[1] || filter === ta.email.filterType[2],
    );
    let isFilterRole = $derived(
        filter === ta.email.filterType[3] || filter === ta.email.filterType[4],
    );

    $effect(() => {
        if (groups.length > 0) {
            group = groups[0].name;
        }
    });

    $effect(() => {
        if (roles.length > 0) {
            role = roles[0].name;
        }
    });

    async function submit() {
        let payload: EmailJobRequest = {
            // scheduled: number;
            filter_type: 'none',
            content_type: editorMode === 'Text' ? 'text' : 'markdown',
            subject,
            body,
        };

        if (isFilterGroup) {
            payload.filter_value = group;
            if (filter === ta.email.filterType[1]) {
                payload.filter_type = 'in_group';
            } else {
                payload.filter_type = 'not_in_group';
            }
        }
        if (isFilterRole) {
            payload.filter_value = role;
            if (filter === ta.email.filterType[3]) {
                payload.filter_type = 'has_role';
            } else {
                payload.filter_type = 'has_not_role';
            }
        }

        console.log(payload);
    }
</script>

<Button ariaLabel={ta.email.sendMail} onclick={() => (showModal = true)} invisible>
    <IconEnvelope />
</Button>

<Modal bind:showModal bind:closeModal>
    <h2>{ta.email.sendMail}</h2>

    <div class="filter">
        <div class="label">
            {ta.email.userFilter}
            <span style:margin=".25rem 0">:</span>
        </div>

        <Options
            ariaLabel={ta.email.userFilter}
            options={ta.email.filterType}
            bind:value={filter}
            borderless
        />

        {#if filter !== ta.email.filterType[0]}
            <div class="ms-05">:</div>
            {#if isFilterGroup}
                <Options
                    ariaLabel={ta.email.userFilter}
                    options={groups.map(g => g.name)}
                    bind:value={group}
                    borderless
                    withSearch={groups.length > 7}
                />
            {:else if isFilterRole}
                <Options
                    ariaLabel={ta.email.userFilter}
                    options={roles.map(g => g.name)}
                    bind:value={role}
                    borderless
                    withSearch={roles.length > 7}
                />
            {/if}
        {/if}
    </div>

    <div class="editor">
        <Input
            label={ta.email.subject}
            placeholder={ta.email.subject}
            bind:value={subject}
            width="min(40rem, 90dvw)"
            maxLength={1024}
            required
        />
        <EditorInteractive
            bind:mode={editorMode}
            bind:sanitizedValue={body}
            height="min(60dvh, 40rem)"
        />
    </div>

    <div class="send">
        <p>
            {#if isFilterNone}
                {ta.email.sendAllUsers}
            {:else if isFilterGroup}
                {ta.email.sendAllUsersFiltered}
                <b>{`${filter} ${group}`}</b>
            {:else if isFilterRole}
                {ta.email.sendAllUsersFiltered}
                <b>{`${filter} ${role}`}</b>
            {/if}
        </p>

        <Button onclick={submit} isDisabled={!body || !subject}>Send</Button>

        {#if error}
            <div class="err">
                {error}
            </div>
        {/if}
    </div>
</Modal>

<style>
    .editor {
        min-width: min(40rem, 93dvw);
    }

    .err {
        margin-top: 0.25rem;
    }

    .filter {
        margin-bottom: 0.5rem;
        display: flex;
        align-items: center;
    }

    .send {
        margin-top: 1rem;
    }

    @media (min-width: 1440px) {
        .editor {
            min-width: 80rem;
        }
    }
</style>
