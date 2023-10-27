<script>
    import {onMount} from "svelte";
    import OrderSearchBar from "$lib/search/OrderSearchBar.svelte";
    import Pagination from "$lib/Pagination.svelte";
    import Button from "$lib/Button.svelte";
    import {slide} from "svelte/transition";

    let err = '';
    let keys = [];
    let resKeys = [];
    let resKeysPaginated = [];
    let refresh;
    let showAddNew = false;

    const searchOptions = [
        {
            label: 'Name',
            callback: (item, search) => item.name.includes(search),
        },
    ];
    let orderOptions = [
        {
            label: 'Name',
            callback: (a, b) => a.name.localeCompare(b.name),
        },
    ];

    onMount(() => {
        fetchApiKeys();
    });

    async function fetchApiKeys() {
        // let res = await getBlacklist();
        // let body = await res.json();
        // if (res.ok) {
        //     blacklist = body.ips;
        // } else {
        //     err = body.message;
        // }
    }

</script>

{err}

<div class="content">
    <div class="top">
        <OrderSearchBar
                items={keys}
                bind:resItems={resKeys}
                searchOptions={searchOptions}
                orderOptions={orderOptions}
        />

        <div class="addNew">
            <Button on:click={() => showAddNew = !showAddNew} level={3}>NEW KEY</Button>
        </div>
    </div>
    {#if showAddNew}
        <div transition:slide class="addNewInputs">
<!--            <Input-->
<!--                    width="9.5rem"-->
<!--                    bind:value={formValues.ip}-->
<!--                    bind:error={formErrors.ip}-->
<!--                    autocomplete="off"-->
<!--                    placeholder="IP"-->
<!--            >-->
<!--                IP-->
<!--            </Input>-->
<!--            <Input-->
<!--                    type="datetime-local"-->
<!--                    step="60"-->
<!--                    width="18rem"-->
<!--                    bind:value={formValues.exp}-->
<!--                    min={minDate}-->
<!--                    max="2099-01-01T00:00"-->
<!--            >-->
<!--                EXPIRES-->
<!--            </Input>-->
<!--            <div class="saveBtn">-->
<!--                <Button on:click={onSubmit} level={1}>SAVE</Button>-->
<!--            </div>-->
<!--            <div class="err">-->
<!--                {errSave}-->
<!--            </div>-->
        </div>
    {/if}

    <div id="keys">
        {#if keys.length === 0}
            <div>
                No Api Keys
            </div>
        {:else}
            {#each resKeysPaginated as key (key.name)}
                {key.name}
<!--                <div class="blacklisted">-->
<!--                    <div class="ip">-->
<!--                        {entry.ip}-->
<!--                    </div>-->
<!--                    <div class="exp">-->
<!--                        {formatDateFromTs(entry.exp)}-->
<!--                    </div>-->
<!--                    <Tooltip text="Delete IP">-->
<!--                        <div-->
<!--                                role="button"-->
<!--                                tabindex="0"-->
<!--                                class="delete"-->
<!--                                on:click={() => deleteIp(entry.ip)}-->
<!--                                on:keypress={() => deleteIp(entry.ip)}-->
<!--                        >-->
<!--                            <IconStop color="var(&#45;&#45;col-err)"/>-->
<!--                        </div>-->
<!--                    </Tooltip>-->
<!--                </div>-->
            {/each}
        {/if}
    </div>

    {#if keys.length > 0}
        <Pagination
                bind:items={resKeys}
                bind:resItems={resKeysPaginated}
        />
    {/if}

    <div style="height: 20px"></div>
</div>

<style>
    #blacklist div:nth-of-type(2n + 1) {
        background: linear-gradient(90deg, var(--col-ghigh) 35rem, var(--col-bg) 50rem);
    }

    .addNew {
        margin-bottom: .6rem;
    }

    .addNewInputs {
        margin: 0 0 1rem -.25rem;
        display: flex;
        flex-direction: row;
        align-items: center;
    }

    .keys {
        display: flex;
        flex-direction: row;
        margin: .25rem .5rem;
    }

    .delete {
        cursor: pointer;
    }

    .err {
        color: var(--col-err);
    }

    .exp {
        width: 10rem;
    }

    .ip {
        width: 9rem;
    }

    .saveBtn {
        margin-top: .25rem;
    }

    .top {
        display: inline-flex;
        align-items: center;
        gap: 1rem;
    }
</style>
