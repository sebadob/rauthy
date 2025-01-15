<script>
    import ExpandContainer from "$lib/ExpandContainer.svelte";
    import * as yup from "yup";
    import {LANGUAGES, REGEX_NAME} from "../../../utils/constants.js";
    import {globalGroupsNames, globalRolesNames} from "../../../stores/admin.js";
    import {extractFormErrors, formatUtcTsFromDateInput} from "../../../utils/helpers";
    import Button from "$lib/Button.svelte";
    import {postUser} from "../../../utils/dataFetchingAdmin.js";
    import ItemTiles from "$lib/itemTiles/ItemTiles.svelte";
    import Input from "$lib/inputs/Input.svelte";
    import OptionSelect from "$lib/OptionSelect.svelte";
    import Switch from "$lib/Switch.svelte";

    let {idx = $bindable(-1), onSave} = $props();

    let err = $state('');
    let isLoading = false;
    let expandContainer = $state();

    let language = $state('EN');
    let limitLifetime = $state(false);
    let userExpires = $state(undefined);
    let formValues = $state({
        email: '',
        family_name: '',
        given_name: '',
        roles: [],
        groups: [],
    });
    let formErrors = $state({});

    const schema = yup.object().shape({
        email: yup.string().required('E-Mail is required').email("Bad E-Mail format"),
        given_name: yup.string().trim().required('Given Name is required').matches(REGEX_NAME, 'Invalid characters'),
        family_name: yup.string().trim().required('Family Name is required').matches(REGEX_NAME, 'Invalid characters'),
    });

    let allRoles = $state();
    globalRolesNames.subscribe(rls => {
        allRoles = rls;
    })

    let allGroups = $state();
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

        let data = formValues;
        data.language = language.toLowerCase();

        if (limitLifetime) {
            let d = formatUtcTsFromDateInput(userExpires);
            if (!d) {
                err = 'Invalid Date Input: User Expires';
                return;
            }
            data.user_expires = d;
        }

        let res = await postUser(formValues);
        if (res.ok) {
            formValues = {};
            expandContainer = false;
            formValues = {
                email: '',
                family_name: '',
                given_name: '',
                roles: [],
                groups: [],
            };
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
    {#snippet header()}
        <div class="header font-label">
            ADD NEW USER
        </div>
    {/snippet}

    {#snippet body()}
        <div class="container">
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

            <div class="language">
                <div class="label">
                    LANGUAGE
                </div>
                <div style="margin-top: .4rem">
                    <OptionSelect bind:value={language} options={LANGUAGES}/>
                </div>
            </div>

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

            <div class="unit" style:margin-top="12px">
                <div class="label font-label">
                    LIMIT LIFETIME
                </div>
                <div class="value">
                    <Switch bind:selected={limitLifetime}/>
                </div>
            </div>
            {#if limitLifetime}
                <Input
                        type="datetime-local"
                        step="60"
                        width="18rem"
                        bind:value={userExpires}
                        on:input={validateForm}
                        min={new Date().toISOString().split('.')[0]}
                        max="2099-01-01T00:00"
                >
                    USER EXPIRES
                </Input>
            {/if}

            <div class="btn">
                <Button on:click={onSubmit} level={1}>SAVE</Button>

                {#if err}
                    <div class="err">
                        {err}
                    </div>
                {/if}
            </div>
        </div>
    {/snippet}
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
        font-size: .9rem;
        margin-left: 10px;
    }

    .err {
        margin: 0 7px;
        color: var(--col-err);
    }

    .label {
        margin: 10px 3px 5px 3px;
        font-size: .9rem;
    }

    .language {
        margin: 7px 5px;
        display: flex;
        gap: .33rem;
    }

    .unit {
        margin: 7px 5px;
    }

    .tiles {
        margin-left: 5px;
    }
</style>
