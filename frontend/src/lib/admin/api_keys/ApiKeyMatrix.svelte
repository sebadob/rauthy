<script lang="ts">
    import type {
        AccessGroup,
        AccessRight,
        ApiKeyAccess,
        ApiKeyResponse,
    } from '$api/types/api_keys.ts';
    import Button from '$lib5/button/Button.svelte';
    import InputCheckbox from '$lib5/form/InputCheckbox.svelte';

    let {
        key,
        finalize = $bindable(),
    }: {
        key?: ApiKeyResponse;
        finalize: undefined | (() => ApiKeyAccess[]);
    } = $props();

    finalize = finalizeMatrix;

    // boolean order: create, read, update, delete
    let matrix: [AccessGroup, boolean, boolean, boolean, boolean][] = $state([
        ['Blacklist', false, false, false, false],
        ['Clients', false, false, false, false],
        ['Events', false, false, false, false],
        ['Generic', false, false, false, false],
        ['Groups', false, false, false, false],
        ['Pam', false, false, false, false],
        ['Roles', false, false, false, false],
        ['Secrets', false, false, false, false],
        ['Sessions', false, false, false, false],
        ['Scopes', false, false, false, false],
        ['UserAttributes', false, false, false, false],
        ['Users', false, false, false, false],
    ]);

    $effect(() => {
        reset();

        if (key) {
            for (let access of key.access) {
                let idx: number;
                switch (access.group) {
                    case 'Blacklist':
                        idx = 0;
                        break;
                    case 'Clients':
                        idx = 1;
                        break;
                    case 'Events':
                        idx = 2;
                        break;
                    case 'Generic':
                        idx = 3;
                        break;
                    case 'Groups':
                        idx = 4;
                        break;
                    case 'Pam':
                        idx = 5;
                        break;
                    case 'Roles':
                        idx = 6;
                        break;
                    case 'Secrets':
                        idx = 7;
                        break;
                    case 'Sessions':
                        idx = 8;
                        break;
                    case 'Scopes':
                        idx = 9;
                        break;
                    case 'UserAttributes':
                        idx = 10;
                        break;
                    case 'Users':
                        idx = 11;
                        break;
                }
                if (idx === undefined) {
                    console.error('invalid idx during access key buildup');
                    return;
                }
                if (access.access_rights.includes('create')) {
                    matrix[idx][1] = true;
                }
                if (access.access_rights.includes('read')) {
                    matrix[idx][2] = true;
                }
                if (access.access_rights.includes('update')) {
                    matrix[idx][3] = true;
                }
                if (access.access_rights.includes('delete')) {
                    matrix[idx][4] = true;
                }
            }
        }
    });

    function finalizeMatrix(): ApiKeyAccess[] {
        let access: ApiKeyAccess[] = [];

        for (let row of matrix) {
            let access_rights: AccessRight[] = [];
            if (row[1]) {
                access_rights.push('create');
            }
            if (row[2]) {
                access_rights.push('read');
            }
            if (row[3]) {
                access_rights.push('update');
            }
            if (row[4]) {
                access_rights.push('delete');
            }
            if (access_rights.length > 0) {
                access.push({
                    group: row[0] as AccessGroup,
                    access_rights,
                });
            }
        }

        return access;
    }

    function reset() {
        for (let row of matrix) {
            row[1] = false;
            row[2] = false;
            row[3] = false;
            row[4] = false;
        }
    }

    function toggleGroup(g: AccessGroup) {
        for (let row of matrix) {
            if (row[0] === g) {
                if (row[1] && row[2] && row[3] && row[4]) {
                    row[1] = false;
                    row[2] = false;
                    row[3] = false;
                    row[4] = false;
                } else {
                    row[1] = true;
                    row[2] = true;
                    row[3] = true;
                    row[4] = true;
                }
                break;
            }
        }
    }

    function toggleRight(r: AccessRight) {
        let idx: number;
        switch (r) {
            case 'create':
                idx = 1;
                break;
            case 'read':
                idx = 2;
                break;
            case 'update':
                idx = 3;
                break;
            case 'delete':
                idx = 4;
                break;
        }
        if (!idx) {
            console.error('logic error in toggleRight, idx is undefined');
            return;
        }

        let allTrue = true;
        for (let row of matrix) {
            if (!row[idx]) {
                allTrue = false;
                break;
            }
        }

        for (let row of matrix) {
            row[idx] = !allTrue;
        }
    }
</script>

<div class="matrix">
    {#snippet btnRight(r: AccessRight)}
        <div class="center">
            <Button ariaLabel={`Toggle: ${r}`} invisible onclick={() => toggleRight(r)}>
                {r}
            </Button>
        </div>
    {/snippet}
    <div class="row">
        <div></div>
        {@render btnRight('create')}
        {@render btnRight('read')}
        {@render btnRight('update')}
        {@render btnRight('delete')}
    </div>

    {#snippet btnGroup(g: AccessGroup)}
        <div>
            <Button ariaLabel={`Toggle: ${g}`} invisible onclick={() => toggleGroup(g)}>
                {g}
            </Button>
        </div>
    {/snippet}
    {#each matrix as row, i}
        <div class="row">
            {@render btnGroup(row[0])}
            <div class="center">
                <InputCheckbox ariaLabel={`${row[0]}: create`} bind:checked={matrix[i][1]} />
            </div>
            <div class="center">
                <InputCheckbox ariaLabel={`${row[0]}: read`} bind:checked={matrix[i][2]} />
            </div>
            <div class="center">
                <InputCheckbox ariaLabel={`${row[0]}: update`} bind:checked={matrix[i][3]} />
            </div>
            <div class="center">
                <InputCheckbox ariaLabel={`${row[0]}: delete`} bind:checked={matrix[i][4]} />
            </div>
        </div>
    {/each}
</div>

<style>
    .center {
        display: flex;
        justify-content: center;
    }

    .matrix {
        margin: 0.5rem 0;
    }

    .row {
        display: grid;
        grid-template-columns: 7.25rem 4rem 3rem 4rem 4rem;
    }
</style>
