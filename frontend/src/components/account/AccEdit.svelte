<script lang="ts">
    import Button from "$lib5/Button.svelte";
    import {fade} from 'svelte/transition';
    import Input from "$lib5/form/Input.svelte";
    import {useI18n} from "$state/i18n.svelte";
    import type {UpdateUserSelfRequest, UserResponse} from "$api/response/common/user.ts";
    import Form from "$lib5/form/Form.svelte";
    import {PATTERN_CITY, PATTERN_PHONE, PATTERN_STREET, PATTERN_USER_NAME} from "$utils/patterns.ts";
    import InputDate from "$lib5/form/InputDate.svelte";
    import IconCheck from "$icons/IconCheck.svelte";
    import {fetchPut} from "$api/fetch.ts";

    let {
        user = $bindable(),
        viewModePhone,
    }: {
        user: UserResponse,
        viewModePhone?: boolean,
    } = $props();

    let t = useI18n();

    let err = $state('');
    let success = $state(false);
    let successEmailConfirm = $state(false);

    async function onSubmit(form: HTMLFormElement, params: URLSearchParams) {
        const email = params.get('email');
        const familyName = params.get('family_name');
        const givenName = params.get('given_name') || undefined;
        const birthdate = params.get('birthdate') || undefined;
        const phone = params.get('phone')?.replaceAll(' ', '') || undefined;
        const street = params.get('street') || undefined;
        const zipStr = params.get('zip') || undefined;
        const zip = zipStr ? Number.parseInt(zipStr) : undefined;
        const city = params.get('city') || undefined;
        const country = params.get('country') || undefined;

        let payload: UpdateUserSelfRequest = {
            email,
            family_name: familyName,
            given_name: givenName,
            user_values: {
                birthdate,
                phone,
                street,
                zip,
                city,
                country
            }
        };

        let res = await fetchPut<UserResponse>(`/auth/v1/users/${user.id}/self`, payload);
        if (res.body) {
            success = true;
            user = res.body;

            if (res.status === 202) {
                successEmailConfirm = true;
            }

            setTimeout(() => {
                success = false;
            }, 3000);
        } else if (res.error) {
            console.error(res.error);
            err = res.error.message;
        }
    }
</script>

<div class="wrapper">
    <div class="container">
        <Form action={`/auth/v1/users/${user.id}/self`} {onSubmit}>
            <div class="formInner">
                <div>
                    <Input
                            typ="email"
                            name="email"
                            label={t.common.email}
                            placeholder={t.common.email}
                            value={user.email}
                            required
                    />
                    <Input
                            name="given_name"
                            autocomplete="given-name"
                            label={t.account.givenName}
                            placeholder={t.account.givenName}
                            value={user.given_name}
                            required
                            maxLength={32}
                            pattern={PATTERN_USER_NAME}
                    />
                    <Input
                            name="family_name"
                            autocomplete="family-name"
                            label={t.account.familyName}
                            placeholder={t.account.familyName}
                            value={user.family_name}
                            maxLength={32}
                            pattern={PATTERN_USER_NAME}
                    />
                    <InputDate
                            name="birthdate"
                            label={t.account.birthdate}
                            value={user.user_values.birthdate}
                    />
                </div>
                <div>
                    <Input
                            name="street"
                            autocomplete="street-address"
                            label={t.account.street}
                            placeholder={t.account.street}
                            value={user.user_values.street}
                            maxLength={48}
                            pattern={PATTERN_STREET}
                    />
                    <Input
                            typ="number"
                            name="zip"
                            autocomplete="postal-code"
                            label={t.account.zip}
                            placeholder={t.account.zip}
                            value={user.user_values.zip}
                            min="1000"
                            max="9999999"
                    />
                    <Input
                            name="city"
                            autocomplete="address-level2"
                            label={t.account.city}
                            placeholder={t.account.city}
                            value={user.user_values.city}
                            maxLength={48}
                            pattern={PATTERN_CITY}
                    />
                    <Input
                            name="country"
                            autocomplete="country"
                            label={t.account.country}
                            placeholder={t.account.country}
                            value={user.user_values.country}
                            maxLength={48}
                            pattern={PATTERN_CITY}
                    />

                    <Input
                            name="phone"
                            autocomplete="tel"
                            label={t.account.phone}
                            placeholder={t.account.phone}
                            value={user.user_values.phone}
                            maxLength={32}
                            pattern={PATTERN_PHONE}
                    />
                </div>
            </div>

            <div class="bottom">
                <div>
                    <Button type="submit">
                        {t.common.save}
                    </Button>
                </div>

                {#if success}
                    <div class="success" transition:fade>
                        <IconCheck/>
                        {#if successEmailConfirm}
                            <p>{t.account.emailUpdateConfirm}</p>
                        {/if}
                    </div>
                {:else if err}
                    <div class="err" transition:fade>
                        {err}
                    </div>
                {/if}
            </div>
        </Form>
    </div>
</div>

<style>
    .bottom {
        display: flex;
        align-items: center;
        gap: 1rem;
    }

    .container {
        padding: 0 5px;
        display: flex;
        flex-direction: column;
    }

    .formInner {
        display: flex;
        gap: 1rem;
        flex-wrap: wrap;
    }

    .success {
        margin-bottom: -.25rem;
        color: var(--col-ok);
    }

    .wrapper {
        margin-top: 10px;
        display: flex;
        flex-direction: row;
    }
</style>
