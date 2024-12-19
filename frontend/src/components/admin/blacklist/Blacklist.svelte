<script>
    import { run } from 'svelte/legacy';

    import {onMount} from "svelte";
    import OrderSearchBar from "$lib/search/OrderSearchBar.svelte";
    import Pagination from "$lib/Pagination.svelte";
    import {deleteBlacklistedIp, getBlacklist, postBlacklist} from "../../../utils/dataFetchingAdmin.js";
    import Button from "$lib/Button.svelte";
    import {slide} from "svelte/transition";
    import Input from "$lib/inputs/Input.svelte";
    import {REGEX_IP_V4} from "../../../utils/constants.js";
    import * as yup from "yup";
    import {extractFormErrors, formatDateFromTs, formatUtcTsFromDateInput} from "../../../utils/helpers.js";
    import IconStop from "$lib/icons/IconStop.svelte";
    import Tooltip from "$lib/Tooltip.svelte";

    let err = $state('');
    let errSave = $state('');
    let blacklist = $state([]);
    let resBlacklist = $state([]);
    let resBlacklistPaginated = $state([]);
    let refresh;
    let showInputs = $state(false);

    let formValues = $state({
        ip: '',
        exp: '',
    })
    let formErrors = $state({});
    const schema = yup.object().shape({
        ip: yup.string()
            .required('IP is required')
            .matches(REGEX_IP_V4, 'Invalid IPv4'),
    });

    const minDate = new Date().toISOString().split('.')[0];

    const searchOptions = [
        {
            label: 'IP',
            callback: (item, search) => item.ip.includes(search),
        },
    ];
    let orderOptions = [
        {
            label: 'IP',
            callback: (a, b) => a.ip.localeCompare(b.ip),
        },
    ];

    run(() => {
        if (showInputs) {
            errSave = '';
            formValues.exp = new Date().toISOString().split('.')[0];
        }
    });

    onMount(() => {
        fetchBlacklist();
    });

    async function fetchBlacklist() {
        let res = await getBlacklist();
        let body = await res.json();
        if (res.ok) {
            blacklist = body.ips;
        } else {
            err = body.message;
        }
    }

    async function onSubmit() {
        errSave = '';

        const isValid = await validateForm();
        if (!isValid) {
            return;
        }

        let exp = formatUtcTsFromDateInput(formValues.exp);
        if (!exp) {
            errSave = 'Invalid Date Input: User Expires';
            return;
        }

        let data = {
            ip: formValues.ip,
            exp,
        };

        let res = await postBlacklist(data);
        if (res.ok) {
            showInputs = false;
            formValues.ip = '';
            await fetchBlacklist();
        } else {
            let body = await res.json();
            errSave = body.message;
        }
    }

    async function deleteIp(ip) {
        let res = await deleteBlacklistedIp(ip);
        if (res.ok) {
            await fetchBlacklist();
        }
    }

    async function validateForm() {
        try {
            await schema.validate(formValues, {abortEarly: false});
            formErrors = {};
            return true;
        } catch (err) {
            formErrors = extractFormErrors(err);
            return false;
        }
    }

</script>

{err}

<div class="content">
    <div class="top">
        <OrderSearchBar
                items={blacklist}
                bind:resItems={resBlacklist}
                searchOptions={searchOptions}
                orderOptions={orderOptions}
        />

        <div class="addNew">
            <Button on:click={() => showInputs = !showInputs} level={3}>BLACKLIST IP</Button>
        </div>
    </div>
    {#if showInputs}
        <div transition:slide class="addNewInputs">
            <Input
                    width="9.5rem"
                    bind:value={formValues.ip}
                    bind:error={formErrors.ip}
                    autocomplete="off"
                    placeholder="IP"
            >
                IP
            </Input>
            <Input
                    type="datetime-local"
                    step="60"
                    width="18rem"
                    bind:value={formValues.exp}
                    min={minDate}
                    max="2099-01-01T00:00"
            >
                EXPIRES
            </Input>
            <div class="saveBtn">
                <Button on:click={onSubmit} level={1}>SAVE</Button>
            </div>
            <div class="err">
                {errSave}
            </div>
        </div>
    {/if}

    <div id="blacklist">
        {#if blacklist.length === 0}
            <div>
                No blacklisted IPs
            </div>
        {:else}
            {#each resBlacklistPaginated as entry (entry.ip)}
                <div class="blacklisted">
                    <div class="ip">
                        {entry.ip}
                    </div>
                    <div class="exp">
                        {formatDateFromTs(entry.exp)}
                    </div>
                    <Tooltip text="Delete IP">
                        <div
                                role="button"
                                tabindex="0"
                                class="delete"
                                onclick={() => deleteIp(entry.ip)}
                                onkeypress={() => deleteIp(entry.ip)}
                        >
                            <IconStop color="var(--col-err)"/>
                        </div>
                    </Tooltip>
                </div>
            {/each}
        {/if}
    </div>

    {#if blacklist.length > 0}
        <Pagination
                bind:items={resBlacklist}
                bind:resItems={resBlacklistPaginated}
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

    .blacklisted {
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
