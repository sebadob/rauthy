<script lang="ts">
    import {onMount} from "svelte";
    import type {AccessGroup, AccessRight, ApiKeyAccess, ApiKeyResponse} from "$api/types/api_keys.ts";
    import Button from "$lib5/button/Button.svelte";
    import InputCheckbox from "$lib5/form/InputCheckbox.svelte";

    let {
        key,
        finalize = $bindable(),
    }: {
        key: ApiKeyResponse,
        finalize: undefined | (() => ApiKeyAccess[]),
    } = $props();

    finalize = finalizeMatrix;

    const GROUPS: AccessGroup[] = [
        'Blacklist',
        'Clients',
        'Events',
        'Generic',
        'Groups',
        'Roles',
        'Secrets',
        'Sessions',
        'Scopes',
        'UserAttributes',
        'Users',
    ];
    const RIGHTS: AccessRight[] = [
        'create',
        'read',
        'update',
        'delete'
    ];

    let accessRights: [string, boolean][] = $state([[]]);

    // interface MatrixRow {
    //     group: AccessGroup,
    //     rights: [AccessRight, boolean][],
    // }
    //
    // let accessMatrix: MatrixRow[] = $state([]);
    // $inspect('accessMatrix', accessMatrix);
    // keep the state for whole row or column toggles
    // let groupsToggle = new Array(GROUPS.length).fill(false);
    // let opsToggle = new Array(4).fill(false);

    // onMount(() => {
    buildArray();
    //     if (key?.access) {
    //         buildFromKey();
    //     }
    // });

    $effect(() => {
        if (key.name) {
            // TODO
        }
    })

    function buildArray() {
        for (let group of GROUPS) {
            for (let right of RIGHTS) {
                let idx = group + '.' + right;
                accessRights[idx] = false;
            }
        }
        //
        //
        // let arr = new Array(GROUPS.length);
        // for (let i = 0; i < GROUPS.length; i++) {
        //     arr[i] = {
        //         group: GROUPS[i],
        //     };
        //     for (let op of OPS) {
        //         arr[i][op] = false;
        //     }
        // }
        // accessMatrix = arr;
    }

    function buildFromKey() {
        for (let access of key.access) {
            let idxGroup = GROUPS.findIndex(name => name === access.group);
            for (let rights of access.access_rights) {
                let idxRights = OPS.findIndex(name => name === rights);
                accessMatrix[idxGroup][OPS[idxRights]] = true;
            }
        }
    }

    export function finalizeMatrix(): ApiKeyAccess[] {
        let access = [];
        for (let i = 0; i < GROUPS.length; i++) {
            let accessRights: AccessRight[] = [];

            for (let op of OPS) {
                if (accessMatrix[i][op]) {
                    accessRights.push(op);
                }
            }
            if (accessRights.length > 0) {
                access.push({
                    group: GROUPS[i],
                    access_rights: accessRights,
                });
            }
        }
        return access;
    }

    function toggleGroup(g: AccessGroup) {
        groupsToggle[idx] = !groupsToggle[idx];

        const toggleTo = groupsToggle[idx];
        let row = accessMatrix[idx];
        for (let i = 0; i < OPS.length; i++) {
            for (let key of Object.keys(row)) {
                if (key !== 'group') {
                    row[key] = toggleTo;
                }
            }
        }
        accessMatrix[idx] = row;
    }

    function toggleOp(idx: AccessRight) {
        opsToggle[idx] = !opsToggle[idx];

        const toggleTo = opsToggle[idx];
        for (let i = 0; i < GROUPS.length; i++) {
            accessMatrix[i][OPS[idx]] = toggleTo;
        }
    }

</script>

<div class="matrix">
    <b>Access Rights</b>

    {#snippet accessRight(s: AccessRight)}
        <Button invisible onclick={() => toggleOp(s)}>
            {s}
        </Button>
    {/snippet}

    <div class="mr">
        <div class="label">
            {@render accessRight('create')}
            {@render accessRight('read')}
            {@render accessRight('update')}
            {@render accessRight('delete')}
        </div>
    </div>

    {#snippet accessGroup(g: AccessGroup)}
        <Button invisible onclick={() => toggleGroup(g)}>
            {g}
        </Button>
        <InputCheckbox ariaLabel="create" bind:checked={accessMatrix[i]['create']}></InputCheckbox>
        <InputCheckbox ariaLabel="read" bind:checked={accessMatrix[i]['read']}></InputCheckbox>
        <InputCheckbox ariaLabel="update" bind:checked={accessMatrix[i]['update']}></InputCheckbox>
        <InputCheckbox ariaLabel="delete" bind:checked={accessMatrix[i]['delete']}></InputCheckbox>
    {/snippet}

    {#if accessMatrix}
        {#each GROUPS as group}
            <div class="mr">
                {@render accessGroup(group)}
            </div>
        {/each}
    {/if}
</div>

<style>
    .cell {
        width: 3.5rem;
        text-align: center;
    }

    .label {
        width: 6.5rem;
    }

    .matrix {
        width: 20.5rem;
        margin: .5rem;
    }

    .mr {
        display: flex;
        align-items: center;
    }

    .mr:hover {
        background: var(--col-gmid);
    }

    .pointer {
        cursor: pointer;
    }
</style>
