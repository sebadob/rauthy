<script lang="ts">
    import type {
        PamGroupUserLink,
        PamUserDetailsResponse,
        PamUserResponse,
        PamUserUpdateRequest,
    } from '$api/types/pam';
    import LabeledValue from '$lib/LabeledValue.svelte';
    import { useI18nAdmin } from '$state/i18n_admin.svelte';
    import Button from '$lib/button/Button.svelte';
    import { useI18n } from '$state/i18n.svelte';
    import { fetchGet, fetchPut } from '$api/fetch';
    import Input from '$lib/form/Input.svelte';
    import Form from '$lib/form/Form.svelte';
    import type { PamGroupsSorted } from '$lib/admin/pam/types';
    import InputCheckbox from '$lib/form/InputCheckbox.svelte';
    import IconCheck from '$icons/IconCheck.svelte';
    import PamAuthorizedKeys from '$lib/PamAuthorizedKeys.svelte';

    let {
        user,
        groupsSorted,
    }: {
        user: PamUserResponse;
        groupsSorted: PamGroupsSorted;
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    let err = $state('');
    let success = $state(false);
    let details: undefined | PamUserDetailsResponse = $state();

    interface LinkedGroup {
        gid: number;
        name: string;
        isMember: boolean;
        isWheel: boolean;
    }

    let groupsGeneric: LinkedGroup[] = $state([]);
    let groupsHost: LinkedGroup[] = $state([]);
    let groupsLocal: LinkedGroup[] = $state([]);
    let groupsUser: LinkedGroup[] = $state([]);

    let url = $derived(`/auth/v1/pam/users/${user.id}`);

    $effect(() => {
        fetchDetails();
    });

    async function fetchDetails() {
        let res = await fetchGet<PamUserDetailsResponse>(url);
        if (res.body) {
            details = res.body;
            buildAccess(res.body.groups);
        } else {
            err = res.error?.message || 'Error';
        }
    }

    function buildAccess(links: PamGroupUserLink[]) {
        let generic: LinkedGroup[] = [];
        let host: LinkedGroup[] = [];
        let local: LinkedGroup[] = [];
        let user: LinkedGroup[] = [];

        for (let group of groupsSorted.generic) {
            let link = links.find(l => l.gid === group.id);
            generic.push({
                gid: group.id,
                name: group.name,
                isMember: !!link,
                isWheel: link?.wheel || false,
            });
        }

        for (let group of groupsSorted.host) {
            let link = links.find(l => l.gid === group.id);
            host.push({
                gid: group.id,
                name: group.name,
                isMember: !!link,
                isWheel: link?.wheel || false,
            });
        }

        for (let group of groupsSorted.local) {
            let link = links.find(l => l.gid === group.id);
            local.push({
                gid: group.id,
                name: group.name,
                isMember: !!link,
                isWheel: link?.wheel || false,
            });
        }

        for (let group of groupsSorted.user) {
            let link = links.find(l => l.gid === group.id);
            user.push({
                gid: group.id,
                name: group.name,
                isMember: !!link,
                isWheel: link?.wheel || false,
            });
        }

        groupsGeneric = generic;
        groupsHost = host;
        groupsLocal = local;
        groupsUser = user;
    }

    async function onSubmit(form: HTMLFormElement, params: URLSearchParams) {
        let links: PamGroupUserLink[] = [];

        for (let group of groupsHost) {
            if (group.isMember) {
                links.push({
                    uid: user.id,
                    gid: group.gid,
                    wheel: group.isWheel,
                });
            }
        }
        for (let group of groupsGeneric) {
            if (group.isMember) {
                links.push({
                    uid: user.id,
                    gid: group.gid,
                    wheel: group.isWheel,
                });
            }
        }
        for (let group of groupsLocal) {
            if (group.isMember) {
                links.push({
                    uid: user.id,
                    gid: group.gid,
                    wheel: group.isWheel,
                });
            }
        }
        for (let group of groupsUser) {
            if (group.isMember) {
                links.push({
                    uid: user.id,
                    gid: group.gid,
                    wheel: group.isWheel,
                });
            }
        }

        let payload: PamUserUpdateRequest = {
            shell: user.shell,
            groups: links,
        };
        let res = await fetchPut(url, payload);
        if (res.status === 200) {
            success = true;
            setTimeout(() => {
                success = false;
            }, 3000);
        } else {
            err = res.error?.message || 'Error';
        }
    }
</script>

<h1>{user.name}</h1>

<Form action={url} {onSubmit}>
    <LabeledValue label="UID">
        {user.id}
    </LabeledValue>
    <LabeledValue label="GID">
        {user.gid}
    </LabeledValue>
    <LabeledValue label={ta.pam.username}>
        {user.name}
    </LabeledValue>
    <LabeledValue label="E-Mail">
        {user.email}
    </LabeledValue>

    <Input label="Shell" placeholder="Shell" required bind:value={user.shell} width="10rem" />

    <h2>{ta.pam.groups}</h2>

    <section class="groups">
        {#snippet snipGroups(header: string, groups: LinkedGroup[], withWheel: boolean = false)}
            {#if groups.length > 0}
                {@const rowClass = withWheel ? 'row row-3' : 'row row-2'}
                <section class="typ">
                    <h3>{header}</h3>
                    <div class={`${rowClass} header`}>
                        <div>{ta.pam.groupName}</div>
                        <div class="center">{ta.pam.member}</div>
                        {#if withWheel}
                            <div class="center">Wheel</div>
                        {/if}
                    </div>

                    {#each groups as group}
                        <div class={rowClass}>
                            <div>{group.name}</div>
                            <div class="center checkbox">
                                <InputCheckbox
                                    ariaLabel={ta.pam.member}
                                    bind:checked={group.isMember}
                                    disabled={group.name === user.name}
                                />
                            </div>
                            {#if withWheel}
                                <div class="center checkbox">
                                    <InputCheckbox
                                        ariaLabel="Wheel"
                                        bind:checked={group.isWheel}
                                        disabled={!group.isMember}
                                    />
                                </div>
                            {/if}
                        </div>
                    {/each}
                </section>
            {/if}
        {/snippet}

        {@render snipGroups('Host', groupsHost, true)}
        {@render snipGroups('Generic', groupsGeneric)}
        {@render snipGroups('Local', groupsLocal)}
        {@render snipGroups('User', groupsUser)}
    </section>

    {#if details?.authorized_keys}
        <PamAuthorizedKeys
            authorizedKeys={details.authorized_keys}
            onSave={fetchDetails}
            isAdmin
            pamUid={user.id}
        />
    {/if}

    <div class="submit">
        <Button type="submit">
            {t.common.save}
        </Button>
        {#if success}
            <IconCheck />
        {:else if err}
            <div class="err">
                {err}
            </div>
        {/if}
    </div>
</Form>

<style>
    h1 {
        margin-top: 0;
    }

    h2 {
        font-size: 1.1rem;
    }

    h3 {
        font-size: 1.05rem;
    }

    .center {
        display: flex;
        justify-content: center;
        align-items: center;
    }

    .checkbox {
        margin-top: -0.25rem;
    }

    .header {
        margin-top: -0.66rem;
        font-weight: bold;
    }

    .row {
        padding: 0.15rem 0.3rem;
        max-width: min(25rem, calc(100dvw - 0.5rem));
        display: grid;
        grid-template-columns: 1fr 5rem 4rem;
        border-radius: var(--border-radius);
        transition: background-color 150ms ease-in-out;
    }

    .row-2 {
        grid-template-columns: 1fr 4.5rem;
    }

    .row-3 {
        grid-template-columns: 1fr 4.5rem 4.5rem;
    }

    .row:hover {
        background: hsla(var(--accent) / 0.3);
    }

    .submit {
        margin-top: 1rem;
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }

    .typ {
        margin: 1rem 0;
    }
</style>
