<script lang="ts">
    import EntityHeader from '#lib/admin/EntityHeader.svelte';
    import EntityDelete from '#lib/admin/EntityDelete.svelte';
    import IconUser from '#icons/IconUser.svelte';
    import type {
        PamGroupUserLink,
        PamUserDetailsResponse,
        PamUserResponse,
        PamUserUpdateRequest,
    } from '#api/types/pam.ts';
    import LabeledValue from '#lib/LabeledValue.svelte';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import Button from '#lib/button/Button.svelte';
    import { useI18n } from '#state/i18n.svelte.ts';
    import { fetchGet, fetchPut } from '#api/fetch.ts';
    import Input from '#lib/form/Input.svelte';
    import Form from '#lib/form/Form.svelte';
    import Section from '#lib/Section.svelte';
    import type { PamGroupsSorted } from '#lib/admin/pam/types.js';
    import InputCheckbox from '#lib/form/InputCheckbox.svelte';
    import IconCheck from '#icons/IconCheck.svelte';
    import PamAuthorizedKeys from '#lib/PamAuthorizedKeys.svelte';

    let {
        user = $bindable(),
        groupsSorted,
        onDelete,
    }: {
        user: PamUserResponse;
        groupsSorted: PamGroupsSorted;
        onDelete: () => void;
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
            home_dir: user.home_dir,
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

<EntityHeader title={user.name} subtitle="PAM">
    {#snippet actions()}
        {#key user.id}
            <EntityDelete {url} name={user.name} message={ta.pam.deleteUser} {onDelete} />
        {/key}
    {/snippet}
    <IconUser width="28px" />
</EntityHeader>

{#snippet snipGroups(header: string, groups: LinkedGroup[], withWheel: boolean = false)}
    {#if groups.length > 0}
        {@const rowClass = withWheel ? 'row row-3' : 'row row-2'}
        <section class="typ">
            <h4>{header}</h4>
            <div class={`${rowClass} header`}>
                <div>{ta.pam.groupName}</div>
                <div class="center">{ta.pam.member}</div>
                {#if withWheel}
                    <div class="center">Wheel</div>
                {/if}
            </div>

            {#each groups as group (group.gid)}
                <div class={rowClass}>
                    <div>{group.name}</div>
                    <div class="center checkbox">
                        <InputCheckbox
                            ariaLabel={`${ta.pam.member} · ${group.name}`}
                            bind:checked={group.isMember}
                            disabled={group.name === user.name}
                        />
                    </div>
                    {#if withWheel}
                        <div class="center checkbox">
                            <InputCheckbox
                                ariaLabel={`Wheel · ${group.name}`}
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

<Form action={url} {onSubmit}>
    <div class="sections">
        <Section title={t.account.navInfo}>
            <div class="ui-field-grid">
                <LabeledValue label="UID" mono>{user.id}</LabeledValue>
                <LabeledValue label="GID" mono>{user.gid}</LabeledValue>
                <LabeledValue label={t.common.email}>{user.email}</LabeledValue>
            </div>
            <div class="ui-field-grid">
                <Input label="Shell" placeholder="Shell" required bind:value={user.shell} />
                <Input label="Home" placeholder="Home" required bind:value={user.home_dir} />
            </div>
        </Section>
        <Section title={ta.pam.groups}>
            {@render snipGroups('Host', groupsHost, true)}
            {@render snipGroups('Generic', groupsGeneric)}
            {@render snipGroups('Local', groupsLocal)}
            {@render snipGroups('User', groupsUser)}
        </Section>

        {#if details?.authorized_keys}
            <Section>
                <PamAuthorizedKeys
                    authorizedKeys={details.authorized_keys}
                    onSave={fetchDetails}
                    isAdmin
                    pamUid={user.id}
                />
            </Section>
        {/if}
    </div>

    <div class="submit">
        <Button type="submit">{t.common.save}</Button>
        {#if success}
            <IconCheck />
        {:else if err}
            <div class="err">{err}</div>
        {/if}
    </div>
</Form>

<style>
    .sections,
    .typ {
        display: grid;
        gap: var(--ui-space);
    }

    .typ {
        gap: 0;
    }

    h4 {
        font-size: var(--ui-control-font-size);
        margin-bottom: 0.375rem;
    }

    .center {
        display: flex;
        justify-content: center;
        align-items: center;
    }

    .header {
        color: var(--ui-muted);
        font-size: 0.8125rem;
        font-weight: 600;
    }

    .row {
        display: grid;
        align-items: center;
        gap: 0.5rem;
        min-height: 2.25rem;
        padding-inline: 0.5rem;
        border-bottom: 1px solid var(--ui-outline);
        font-size: var(--ui-control-font-size);
        overflow-wrap: anywhere;
    }

    .row-2 {
        grid-template-columns: minmax(0, 1fr) minmax(3rem, 18%);
    }

    .row-3 {
        grid-template-columns: minmax(0, 1fr) repeat(2, minmax(3rem, 18%));
    }

    .row:not(.header):hover {
        background: var(--ui-hover);
    }

    .submit {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        margin-top: var(--ui-space);
    }
</style>
