<script>
    import ExpandContainer from "$lib/ExpandContainer.svelte";
    import * as yup from "yup";
    import {REGEX_NAME} from "../../../utils/constants.js";
    import {globalGroupsNames, globalRolesNames} from "../../../stores/admin.js";
    import {extractFormErrors} from "../../../utils/helpers.js";
    import Button from "$lib/Button.svelte";
    import {postUser} from "../../../utils/dataFetchingAdmin.js";
    import ItemTiles from "$lib/itemTiles/ItemTiles.svelte";
    import Input from "$lib/inputs/Input.svelte";

    export let idx = -1;
    export let onSave;

    let err = '';
    let isLoading = false;
    let expandContainer;

    let formValues = {
        email: '',
        family_name: '',
        given_name: '',
        roles: [],
        groups: [],
    };
    let formErrors = {};

    const schema = yup.object().shape({
        email: yup.string().required('E-Mail is required').email("Bad E-Mail format"),
        given_name: yup.string().trim().required('Given Name is required').matches(REGEX_NAME, 'Invalid characters'),
        family_name: yup.string().trim().required('Family Name is required').matches(REGEX_NAME, 'Invalid characters'),
    });

    let allRoles;
    globalRolesNames.subscribe(rls => {
        allRoles = rls;
    })

    let allGroups;
    globalGroupsNames.subscribe(grps => {
        allGroups = grps;
    })

    async function onSubmit() {
        const valid = await validateForm();
        if (!valid) {
            return;
        }
        err = '';
        isLoading = true;

        let res = await postUser(formValues);
        if (res.ok) {
            formValues = {};
            expandContainer = false;
            onSave();
        } else {
            let body = await res.json();
            err = body.message;
        }

        isLoading = false;
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

<ExpandContainer bind:idx bind:show={expandContainer}>
    <div class="header font-label" slot="header">
        ADD NEW USER
    </div>

    <div class="container" slot="body">
        <Input
                type="email"
                bind:value={formValues.email}
                bind:error={formErrors.email}
                autocomplete="off"
                placeholder="E-Mail"
                on:input={validateForm}
        >
            E-MAIL
        </Input>
        <Input
                bind:value={formValues.given_name}
                bind:error={formErrors.given_name}
                autocomplete="off"
                placeholder="Given Name"
                on:input={validateForm}
        >
            GIVEN NAME
        </Input>
        <Input
                bind:value={formValues.family_name}
                bind:error={formErrors.family_name}
                autocomplete="off"
                placeholder="Family Name"
                on:input={validateForm}
        >
            FAMILY NAME
        </Input>
        <div class="tiles">
            <div class="label font-label">
                ROLES
            </div>
            <ItemTiles
                    options={allRoles}
                    bind:items={formValues.roles}
                    searchThreshold={4}
            />
        </div>
        <div class="tiles">
            <div class="label font-label">
                GROUPS
            </div>
            <ItemTiles
                    options={allGroups}
                    bind:items={formValues.groups}
                    searchThreshold={4}
            />
        </div>
        <div class="btn">
            <Button on:click={onSubmit} level={1}>SAVE</Button>

            {#if err}
                <div class="err">
                    {err}
                </div>
            {/if}
        </div>
    </div>
</ExpandContainer>

<style>
    .container {
        padding: 10px;
    }

    .btn {
        margin: 10px 0;
    }

    .header {
        display: flex;
        font-size: .9em;
        margin-left: 10px;
    }

    .err {
        margin: 0 7px;
        color: var(--col-err);
    }

    .label {
        margin: 10px 3px 5px 3px;
        font-size: .85em;
    }

    .tiles {
        margin-left: 5px;
    }
</style>
