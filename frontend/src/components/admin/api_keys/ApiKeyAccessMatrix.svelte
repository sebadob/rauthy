<script>
    import {onMount} from "svelte";

    export let apiKey;
    export let accessMatrix;

    const GROUPS = [
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
    const OPS = [
        'create',
        'read',
        'update',
        'delete',
    ];

    // keep the state for whole row or column toggles
    let groupsToggle = new Array(GROUPS.length).fill(false);
    let opsToggle = new Array(OPS.length).fill(false);

    onMount(() => {
        buildArray();
        if (apiKey?.access) {
            buildFromKey();
        }
    });

    function buildArray() {
        let arr = new Array(GROUPS.length);
        for (let i = 0; i < GROUPS.length; i++) {
            arr[i] = {
                group: GROUPS[i],
            };
            for (let op of OPS) {
                arr[i][op] = false;
            }
        }
        accessMatrix = arr;
    }

    function buildFromKey() {
        for (let access of apiKey.access) {
            let idxGroup = GROUPS.findIndex(name => name === access.group);
            for (let rights of access.access_rights) {
                let idxRights = OPS.findIndex(name => name === rights);
                accessMatrix[idxGroup][OPS[idxRights]] = true;
            }
        }
    }

    export function finalize() {
        let access = [];
        for (let i = 0; i < GROUPS.length; i++) {
            let accessRights = [];
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

    function toggleGroup(idx) {
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

    function toggleOp(idx) {
        opsToggle[idx] = !opsToggle[idx];

        const toggleTo = opsToggle[idx];
        for (let i = 0; i < GROUPS.length; i++) {
            accessMatrix[i][OPS[idx]] = toggleTo;
        }
    }

</script>

<div class="matrix">
    <b>Access Rights</b>

    <div class="mr">
        <div class="label"></div>
        {#each OPS as op, i (i)}
            <div
                    role="button"
                    tabindex="0"
                    class="cell pointer"
                    on:click={toggleOp.bind(this, i)}
                    on:keypress={toggleOp.bind(this, i)}
            >
                {OPS[i]}
            </div>
        {/each}
    </div>

    {#if accessMatrix}
        {#each GROUPS as group, i (i)}
            <div class="mr">
                <div
                        role="button"
                        tabindex="0"
                        class="label pointer"
                        on:click={toggleGroup.bind(this, i)}
                        on:keypress={toggleGroup.bind(this, i)}
                >
                    {GROUPS[i]}
                </div>
                {#each OPS as op, j (j)}
                    <div class="cell">
                        <input
                                class="pointer"
                                type="checkbox"
                                bind:checked={accessMatrix[i][op]}
                        >
                    </div>
                {/each}
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
