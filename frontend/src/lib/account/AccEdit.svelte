<script lang="ts">
    import Button from '$lib5/button/Button.svelte';
    import { fade } from 'svelte/transition';
    import Input from '$lib5/form/Input.svelte';
    import { useI18n } from '$state/i18n.svelte.js';
    import type { UpdateUserSelfRequest, UserResponse } from '$api/types/user.ts';
    import Form from '$lib5/form/Form.svelte';
    import {
        PATTERN_ALNUM,
        PATTERN_CITY,
        PATTERN_PHONE,
        PATTERN_STREET,
        PATTERN_USER_NAME,
    } from '$utils/patterns';
    import IconCheck from '$icons/IconCheck.svelte';
    import { fetchDelete, fetchGet, fetchPut } from '$api/fetch';
    import InputDateTimeCombo from '$lib5/form/InputDateTimeCombo.svelte';
    import type { UserValuesConfig } from '$api/templates/UserValuesConfig';
    import { onMount } from 'svelte';
    import TZSelect from '$lib/TZSelect.svelte';
    import PreferredUsername from '$lib/PreferredUsername.svelte';

    let {
        config,
        user = $bindable(),
        viewModePhone,
    }: {
        config: undefined | UserValuesConfig;
        user: UserResponse;
        viewModePhone?: boolean;
    } = $props();

    if (!user.user_values.birthdate) {
        user.user_values.birthdate = '';
    }
    if (!user.user_values.preferred_username) {
        user.user_values.preferred_username = '';
    }
    if (!user.user_values.tz) {
        user.user_values.tz = 'UTC';
    }

    let t = useI18n();

    let err = $state('');
    let success = $state(false);
    let successEmailConfirm = $state(false);

    let canSelfDelete = $state(false);
    let showDeleteConfirm = $state(false);
    let deleteConfirmValue = $state('');

    onMount(() => {
        fetchSelfDeleteConfig();
    });

    $effect(() => {
        if (showDeleteConfirm) {
            deleteConfirmValue = '';
        }
    });

    async function fetchSelfDeleteConfig() {
        let res = await fetchGet(`/auth/v1/users/${user.id}/self/delete`);
        canSelfDelete = res.status === 202;
    }

    async function onSubmit(form: HTMLFormElement, params: URLSearchParams) {
        if (!config) {
            return;
        }

        const email = params.get('email');
        const familyName = params.get('family_name') || undefined;
        const givenName = params.get('given_name') || undefined;

        let userValues = {
            birthdate: params.get('birthdate') || undefined,
            phone: params.get('phone')?.replaceAll(' ', '') || undefined,
            street: params.get('street') || undefined,
            zip: params.get('zip') || undefined,
            city: params.get('city') || undefined,
            country: params.get('country') || undefined,
            tz: user.user_values.tz || undefined,
        };
        if (userValues.tz === 'Etc/UTC' || userValues.tz === 'UTC') {
            userValues.tz = undefined;
        }

        let payload: UpdateUserSelfRequest = {
            email,
            family_name: familyName,
            given_name: givenName,
            user_values: userValues,
        };

        let res = await fetchPut<UserResponse>(`/auth/v1/users/${user.id}/self`, payload);
        if (res.body) {
            success = true;
            if (!res.body.user_values?.birthdate) {
                res.body.user_values.birthdate = '';
            }
            if (!res.body.user_values?.tz) {
                res.body.user_values.tz = 'UTC';
            }
            user = res.body;

            let timer = 3000;
            if (res.status === 202) {
                successEmailConfirm = true;
                timer = 30000;
            }

            setTimeout(() => {
                success = false;
            }, timer);
        } else if (res.error) {
            console.error(res.error);
            err = res.error.message;
        }
    }

    async function onSubmitDelete() {
        let res = await fetchDelete(`/auth/v1/users/${user.id}/self/delete`);
        if (res.status === 204) {
            window.location.href = '/auth/v1';
        }
    }
</script>

<div class="container">
    {#if config}
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
                    {#if config.given_name !== 'hidden'}
                        <Input
                            name="given_name"
                            autocomplete="given-name"
                            label={t.account.givenName}
                            placeholder={t.account.givenName}
                            value={user.given_name}
                            required={config.given_name === 'required'}
                            maxLength={32}
                            pattern={PATTERN_USER_NAME}
                        />
                    {/if}
                    {#if config.family_name !== 'hidden'}
                        <Input
                            name="family_name"
                            autocomplete="family-name"
                            label={t.account.familyName}
                            placeholder={t.account.familyName}
                            value={user.family_name}
                            required={config.family_name === 'required'}
                            maxLength={32}
                            pattern={PATTERN_USER_NAME}
                        />
                    {/if}
                    {#if config.birthdate !== 'hidden'}
                        <InputDateTimeCombo
                            name="birthdate"
                            label={t.account.birthdate}
                            bind:value={user.user_values.birthdate}
                            required={config.birthdate === 'required'}
                            withDelete
                            openTop
                        />
                    {/if}
                    {#if config.tz !== 'hidden'}
                        <TZSelect bind:value={user.user_values.tz} />
                    {/if}
                    <PreferredUsername
                        userId={user.id}
                        bind:preferred_username={user.user_values.preferred_username}
                        config={config.preferred_username}
                    />
                </div>
                <div>
                    {#if config.street !== 'hidden'}
                        <Input
                            name="street"
                            autocomplete="street-address"
                            label={t.account.street}
                            placeholder={t.account.street}
                            value={user.user_values.street}
                            required={config.street === 'required'}
                            maxLength={48}
                            pattern={PATTERN_STREET}
                        />
                    {/if}
                    {#if config.zip !== 'hidden'}
                        <Input
                            name="zip"
                            autocomplete="postal-code"
                            label={t.account.zip}
                            placeholder={t.account.zip}
                            value={user.user_values.zip}
                            required={config.zip === 'required'}
                            maxLength={24}
                            pattern={PATTERN_ALNUM}
                        />
                    {/if}
                    {#if config.city !== 'hidden'}
                        <Input
                            name="city"
                            autocomplete="address-level2"
                            label={t.account.city}
                            placeholder={t.account.city}
                            value={user.user_values.city}
                            required={config.city === 'required'}
                            maxLength={48}
                            pattern={PATTERN_CITY}
                        />
                    {/if}
                    {#if config.country !== 'hidden'}
                        <Input
                            name="country"
                            autocomplete="country"
                            label={t.account.country}
                            placeholder={t.account.country}
                            value={user.user_values.country}
                            required={config.country === 'required'}
                            maxLength={48}
                            pattern={PATTERN_CITY}
                        />
                    {/if}
                    {#if config.phone !== 'hidden'}
                        <Input
                            name="phone"
                            autocomplete="tel"
                            label={t.account.phone}
                            placeholder={t.account.phone}
                            value={user.user_values.phone}
                            required={config.phone === 'required'}
                            maxLength={32}
                            pattern={PATTERN_PHONE}
                        />
                    {/if}
                </div>
            </div>

            <div class="bottom">
                <div>
                    <Button type="submit">
                        {t.common.save}
                    </Button>
                </div>
                {#if canSelfDelete}
                    <div>
                        <Button level={-3} onclick={() => (showDeleteConfirm = !showDeleteConfirm)}>
                            {t.account.deleteAccount.deleteAccount}
                        </Button>
                    </div>
                {/if}

                {#if success}
                    <div class="success" transition:fade>
                        <IconCheck />
                    </div>
                {/if}
            </div>
            {#if successEmailConfirm}
                <p>{t.account.emailUpdateConfirm}</p>
            {:else if err}
                <div class="err" transition:fade>
                    {err}
                </div>
            {/if}
        </Form>
    {/if}

    {#if showDeleteConfirm}
        <div class="selfDelete">
            <h5>{t.account.deleteAccount.deleteAccount}</h5>
            <p>
                {t.account.deleteAccount.deleteAccountDesc}
                <b>{t.common.delete.toUpperCase()}</b>
            </p>

            <Input
                label={t.common.delete.toUpperCase()}
                placeholder={t.common.delete.toUpperCase()}
                bind:value={deleteConfirmValue}
                width="15rem"
            />
            <Button
                level={deleteConfirmValue !== t.common.delete.toUpperCase() ? -3 : -1}
                onclick={onSubmitDelete}
                isDisabled={deleteConfirmValue !== t.common.delete.toUpperCase()}
            >
                {t.common.delete}
            </Button>
        </div>
    {/if}
</div>

<style>
    .bottom {
        margin-top: 0.35rem;
        display: flex;
        align-items: center;
        gap: 1rem;
    }

    .container {
        display: flex;
        flex-direction: column;
    }

    .err {
        margin-top: 0.5rem;
        color: hsl(var(--error));
    }

    .formInner {
        display: flex;
        gap: 1rem;
        flex-wrap: wrap;
    }

    .selfDelete {
        margin-top: 3rem;
        margin-bottom: 1rem;
        padding: 0 0.5rem 1rem 0.5rem;
        border: 1px solid hsla(var(--error) / 0.8);
        border-radius: var(--border-radius);
        background: hsla(var(--error) / 0.05);
    }

    .success {
        margin-bottom: -0.25rem;
    }
</style>
