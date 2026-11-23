<script lang="ts">
    import { fetchGet } from '#api/fetch.ts';
    import Button from '#lib/button/Button.svelte';
    import Input from '#lib/form/Input.svelte';
    import IconTrash from '#icons/IconTrash.svelte';
    import JsonPathDesc from '#lib/admin/providers/JsonPathDesc.svelte';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import { PATTERN_CLAIM_VALUE, PATTERN_JSON_PATH } from '#utils/patterns.ts';
    import type { ClaimMapping, ClaimMappingTyp } from '#api/types/auth_provider.ts';
    import type { GroupResponse } from '#api/types/groups.ts';
    import type { RoleResponse } from '#api/types/roles.ts';

    // Mirrors the backend `CLAIM_MAPPINGS_MAX_RULES`.
    const MAX_RULES = 32;
    // Mirrors the backend `RAUTHY_ADMIN_GROUP_PREFIX`.
    const GROUP_ADMIN_PREFIX = 'rauthy_admin:';

    let {
        claimMappings = $bindable(),
        inputWidth,
    }: {
        claimMappings: undefined | ClaimMapping[];
        inputWidth: string;
    } = $props();

    let ta = useI18nAdmin();

    let rules = $derived(claimMappings ?? []);
    let atMax = $derived(rules.length >= MAX_RULES);

    let roleNames = $state<string[]>([]);
    let groupNames = $state<string[]>([]);

    $effect(() => {
        void (async () => {
            let [roles, groups] = await Promise.all([
                fetchGet<RoleResponse[]>('/auth/v1/roles'),
                fetchGet<GroupResponse[]>('/auth/v1/groups'),
            ]);
            roleNames = (roles.body ?? []).map(r => r.name).sort();
            groupNames = (groups.body ?? []).map(g => g.name).sort();
        })();
    });

    // A `rauthy_admin:<group>` role is the delegated admin of that group. It is a role name that
    // never lives in the roles table, so its options come from the groups.
    let roleTargets = $derived([
        ...new Set([...roleNames, ...groupNames.map(g => `${GROUP_ADMIN_PREFIX}${g}`)]),
    ]);

    function targets(typ: ClaimMappingTyp): string[] {
        return typ === 'role' ? roleTargets : groupNames;
    }

    function targetLabel(name: string): string {
        return name.startsWith(GROUP_ADMIN_PREFIX)
            ? `${name} ${ta.providers.config.groupAdminTarget}`
            : name;
    }

    // A saved target that is not a role / group anymore selects nothing: no mapping can happen
    // for it, and the admin picks a real one or removes the rule.
    function selectedTarget(rule: ClaimMapping): string {
        return targets(rule.typ).includes(rule.target) ? rule.target : '';
    }

    function addRule() {
        if (atMax) {
            return;
        }
        if (!claimMappings) {
            claimMappings = [];
        }
        claimMappings.push({
            claim_path: '',
            claim_value: '',
            typ: 'role',
            target: '',
        });
    }

    function removeRule(idx: number) {
        claimMappings?.splice(idx, 1);
    }
</script>

<fieldset class="claim-mappings">
    <legend>{ta.providers.config.sections?.roleGroupMapping ?? 'Roles and Groups'}</legend>
    <p class="ui-muted-copy">{ta.providers.config.mapRoleGroups}</p>

    {#each rules as rule, idx (idx)}
        <div class="rule">
            <select
                class="ui-text-input"
                bind:value={rule.typ}
                aria-label={ta.providers.config.mappingTyp}
            >
                <option value="role">{ta.providers.config.typRole}</option>
                <option value="group">{ta.providers.config.typGroup}</option>
            </select>
            <Input
                bind:value={rule.claim_path}
                autocomplete="off"
                label={ta.providers.config.pathClaim}
                placeholder="$.groups[*]"
                pattern={PATTERN_JSON_PATH}
                width={inputWidth}
                required
            />
            <Input
                bind:value={rule.claim_value}
                autocomplete="off"
                label={ta.providers.config.valueClaim}
                placeholder="^tgchat:"
                pattern={PATTERN_CLAIM_VALUE}
                width={inputWidth}
                required
            />
            <select
                class="ui-text-input"
                value={selectedTarget(rule)}
                onchange={ev => (rule.target = ev.currentTarget.value)}
                aria-label={ta.providers.config.targetClaim}
                required
            >
                <option value="">{ta.providers.config.noTargetSelected}</option>
                {#each targets(rule.typ) as name (name)}
                    <option value={name}>{targetLabel(name)}</option>
                {/each}
            </select>
            <Button
                level={-2}
                invisibleOutline
                ariaLabel={ta.providers.config.removeMapping}
                onclick={() => removeRule(idx)}
            >
                <IconTrash />
            </Button>

            {#if rule.target && selectedTarget(rule) === ''}
                <p class="warn">
                    {ta.providers.config.targetMissing.replace('{{ target }}', rule.target)}
                </p>
            {/if}

            {#if rule.claim_value.startsWith('~')}
                <p class="warn">{ta.providers.config.regexCost}</p>
            {/if}
        </div>
    {/each}

    <div class="actions">
        <Button level={2} onclick={addRule} isDisabled={atMax}>
            {ta.providers.config.addMapping}
        </Button>
        {#if atMax}
            <span class="warn">{ta.providers.config.maxMappings}</span>
        {/if}
    </div>

    <JsonPathDesc />
</fieldset>

<style>
    .claim-mappings {
        min-inline-size: 0;
        padding: 0.75rem;
        border: 1px solid var(--ui-outline);
        border-radius: var(--border-radius);
        display: grid;
        gap: 0.75rem 1rem;
    }

    legend {
        padding-inline: 0.375rem;
        color: var(--ui-text);
        font-size: var(--ui-control-font-size);
        font-weight: 600;
    }

    .claim-mappings > p {
        grid-column: 1 / -1;
    }

    select {
        inline-size: 100%;
    }

    .rule {
        margin: 1.5rem 0;
        grid-column: 1 / -1;
        display: grid;
        gap: 0.5rem 1rem;
        align-items: start;
        --ui-field-margin: 0;
    }

    .rule .warn {
        grid-column: 1 / -1;
        color: hsl(var(--error));
        font-size: 0.9rem;
        opacity: 0.8;
    }

    .actions {
        grid-column: 1 / -1;
        display: flex;
        align-items: center;
        flex-wrap: wrap;
        gap: 0.75rem 1rem;
    }

    .warn {
        font-size: 0.9rem;
        opacity: 0.8;
    }

    @container (width >= 48rem) {
        .rule {
            grid-template-columns: 7rem minmax(0, 1fr) minmax(0, 1fr) minmax(0, 1fr) auto;
        }

        .rule .warn {
            grid-column: 2 / -1;
        }
    }
</style>
