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

</script>

<div class="matrix">
    <b>Access Rights</b>

    <div class="mr">
        <div class="label"></div>
        {#each OPS as op, i (i)}
            <div class="cell">{OPS[i]}</div>
        {/each}
    </div>

    {#if accessMatrix}
        {#each GROUPS as group, i (i)}
            <div class="mr">
                <div class="label">{GROUPS[i]}</div>
                {#each OPS as op, j (j)}
                    <div class="cell">
                        <input
                                class="chkbox"
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

    .chkbox {
        cursor: pointer;
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
</style>
