<script lang="ts">
    import { formatDateFromTs } from '$utils/helpers.js';
    import Button from '$lib5/button/Button.svelte';
    import { untrack } from 'svelte';
    import CheckIcon from '$lib5/CheckIcon.svelte';
    import Input from '$lib5/form/Input.svelte';
    import type { UserResponse, UserResponseSimple } from '$api/types/user.ts';
    import type { RoleResponse } from '$api/types/roles.ts';
    import type { GroupResponse } from '$api/types/groups.ts';
    import type { SelectItem } from '$lib5/select_list/props.ts';
    import { fmtDateInput, fmtTimeInput, unixTsFromLocalDateTime } from '$utils/form';
    import { fetchPatch } from '$api/fetch';
    import Form from '$lib5/form/Form.svelte';
    import IconCheck from '$icons/IconCheck.svelte';
    import { useI18n } from '$state/i18n.svelte';
    import { useI18nAdmin } from '$state/i18n_admin.svelte';
    import LabeledValue from '$lib5/LabeledValue.svelte';
    import InputCheckbox from '$lib5/form/InputCheckbox.svelte';
    import {
        PATTERN_ALNUM,
        PATTERN_CITY,
        PATTERN_PHONE,
        PATTERN_STREET,
        PATTERN_USER_NAME,
    } from '$utils/patterns';
    import Options from '$lib5/Options.svelte';
    import SelectList from '$lib5/select_list/SelectList.svelte';
    import InputDateTimeCombo from '$lib5/form/InputDateTimeCombo.svelte';
    import { slide } from 'svelte/transition';
    import type { Language } from '$api/types/i18n.ts';
    import { useI18nConfig } from '$state/i18n_config.svelte';
    import UserPicture from '$lib/UserPicture.svelte';
    import type { PatchOp } from '$api/types/generic';
    import type { AuthProviderTemplate } from '$api/templates/AuthProvider';
    import { useSession } from '$state/session.svelte';
    import Tooltip from '$lib/Tooltip.svelte';
    import TZSelect from '$lib/TZSelect.svelte';
    import PreferredUsername from '$lib/PreferredUsername.svelte';
    import type { UserValuesConfig } from '$api/templates/UserValuesConfig';

    let {
        user = $bindable(),
        config,
        roles,
        groups,
        providers,
        onSave,
    }: {
        user: UserResponse;
        config: UserValuesConfig;
        roles: RoleResponse[];
        groups: GroupResponse[];
        providers: AuthProviderTemplate[];
        onSave: () => void;
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();
    let session = useSession('admin');

    let err = $state('');
    let success = $state(false);

    let userOrig: undefined | UserResponse;
    let isSelf = $derived(session.get()?.user_id === user.id);

    let email = $state('');
    let givenName = $state('');
    let familyName = $state('');
    let languages = $derived(
        useI18nConfig()
            .common()
            ?.map(l => l as string),
    );
    let language: Language = $state('en');

    let birthdate = $state('');
    let tz = $state('UTC');
    let phone = $state('');
    let street = $state('');
    let zip = $state('');
    let city = $state('');
    let country = $state('');

    let enabled = $state(false);
    let emailVerified = $state(false);
    let expires = $state(false);
    let expDate = $state(fmtDateInput());
    let expTime = $state(fmtTimeInput());

    let rolesItems: SelectItem[] = $state(
        untrack(() =>
            roles.map(r => {
                let i: SelectItem = {
                    name: r.name,
                    selected: false,
                };
                return i;
            }),
        ),
    );
    let groupsItems: SelectItem[] = $state(
        untrack(() =>
            groups.map(g => {
                let i: SelectItem = {
                    name: g.name,
                    selected: false,
                };
                return i;
            }),
        ),
    );

    let providerName = $derived(
        user.account_type?.startsWith('federated')
            ? providers.filter(p => p.id == user.auth_provider_id)[0]?.name
            : '',
    );

    $effect(() => {
        if (user) {
            userOrig = {
                id: user.id,
                email: user.email,
                given_name: user.given_name,
                family_name: user.family_name || '',
                language: user.language,
                roles: user.roles,
                groups: user.groups || [],
                enabled: user.enabled,
                account_type: user.account_type,
                email_verified: user.email_verified,
                created_at: user.created_at,
                user_expires: user.user_expires,
                user_values: {
                    birthdate: user.user_values?.birthdate || '',
                    phone: user.user_values?.phone || '',
                    street: user.user_values?.street || '',
                    zip: user.user_values?.zip,
                    city: user.user_values?.city || '',
                    country: user.user_values?.country || '',
                    tz: user.user_values?.tz || 'UTC',
                },
            };

            email = user.email;
            givenName = user.given_name || '';
            familyName = user.family_name || '';
            language = user.language;

            birthdate = user.user_values?.birthdate || '';
            phone = user.user_values?.phone || '';
            street = user.user_values?.street || '';
            zip = user.user_values?.zip?.toString() || '';
            city = user.user_values?.city || '';
            country = user.user_values?.country || '';
            tz = user.user_values?.tz || 'UTC';

            enabled = user.enabled;
            emailVerified = user.email_verified;
            if (user.user_expires) {
                let d = new Date(user.user_expires * 1000);
                expires = true;
                expDate = fmtDateInput(d);
                expTime = fmtTimeInput(d);
            } else {
                expires = false;
                expDate = fmtDateInput();
                expTime = fmtTimeInput();
            }

            rolesItems = roles.map(r => {
                let i: SelectItem = {
                    name: r.name,
                    selected: user?.roles.includes(r.name) || false,
                };
                return i;
            });
            groupsItems = groups.map(g => {
                let i: SelectItem = {
                    name: g.name,
                    selected: user?.groups?.includes(g.name) || false,
                };
                return i;
            });
        }
    });

    function fallbackCharacters(user: UserResponseSimple) {
        let chars = '';
        if (user.given_name) {
            chars = user.given_name[0];
        } else {
            chars = user.email[0];
        }
        if (user.family_name) {
            chars += user.family_name[0];
        }
        return chars;
    }

    async function onSubmit(form: HTMLFormElement, params: URLSearchParams) {
        err = '';

        let payload: PatchOp = {
            put: [],
            del: [],
        };

        if (email !== userOrig?.email) {
            payload.put.push({ key: 'email', value: email });
        }
        if (givenName !== userOrig?.given_name) {
            if (givenName) {
                payload.put.push({ key: 'given_name', value: givenName });
            } else {
                payload.del.push('given_name');
            }
        }
        if (familyName !== userOrig?.family_name) {
            if (familyName) {
                payload.put.push({ key: 'family_name', value: familyName });
            } else {
                payload.del.push('family_name');
            }
        }
        if (language !== userOrig?.language) {
            if (language) {
                payload.put.push({ key: 'language', value: language });
            } else {
                payload.del.push('language');
            }
        }

        let roles = rolesItems.filter(i => i.selected).map(i => i.name);
        if (roles.join(',') !== userOrig?.roles?.join(',')) {
            if (roles.length > 0) {
                payload.put.push({ key: 'roles', value: roles });
            } else {
                payload.del.push('roles');
            }
        }
        // anti-lockout check for self
        if (isSelf && !roles.includes('rauthy_admin')) {
            err = `${ta.users.antiLockout.rule}: ${ta.users.antiLockout.rauthyAdmin}`;
            return;
        }

        let groups = groupsItems.filter(i => i.selected).map(i => i.name);
        if (groups.join(',') !== userOrig?.groups?.join(',')) {
            if (groups.length > 0) {
                payload.put.push({ key: 'groups', value: groups });
            } else {
                payload.del.push('groups');
            }
        }

        if (enabled !== userOrig?.enabled) {
            payload.put.push({ key: 'enabled', value: enabled });
        }
        if (emailVerified !== userOrig?.email_verified) {
            payload.put.push({ key: 'email_verified', value: emailVerified });
        }
        if (expires !== (!!userOrig?.user_expires || false)) {
            if (expires) {
                let exp = unixTsFromLocalDateTime(expDate, expTime);
                payload.put.push({ key: 'user_expires', value: exp });
            } else {
                payload.del.push('user_expires');
            }
        }

        if (birthdate !== userOrig?.user_values?.birthdate) {
            if (birthdate) {
                payload.put.push({
                    key: 'user_values.birthdate',
                    value: birthdate,
                });
            } else {
                payload.del.push('user_values.birthdate');
            }
        }
        if (tz !== userOrig?.user_values?.tz) {
            if (tz && tz !== 'Etc/UTC' && tz !== 'UTC') {
                payload.put.push({ key: 'user_values.tz', value: tz });
            } else {
                payload.del.push('user_values.tz');
            }
        }
        if (phone !== userOrig?.user_values?.phone) {
            if (phone) {
                payload.put.push({ key: 'user_values.phone', value: phone });
            } else {
                payload.del.push('user_values.phone');
            }
        }
        if (street !== userOrig?.user_values?.street) {
            if (street) {
                payload.put.push({ key: 'user_values.street', value: street });
            } else {
                payload.del.push('user_values.street');
            }
        }
        if (zip !== (userOrig?.user_values?.zip || '')) {
            if (zip) {
                payload.put.push({ key: 'user_values.zip', value: zip });
            } else {
                payload.del.push('user_values.zip');
            }
        }
        if (city !== userOrig?.user_values?.city) {
            if (city) {
                payload.put.push({ key: 'user_values.city', value: city });
            } else {
                payload.del.push('user_values.city');
            }
        }
        if (country !== userOrig?.user_values?.country) {
            if (country) {
                payload.put.push({
                    key: 'user_values.country',
                    value: country,
                });
            } else {
                payload.del.push('user_values.country');
            }
        }

        if (Object.entries(payload.del).length === 0 && Object.entries(payload.put).length === 0) {
            console.log('nothing to do');
            return;
        }

        let res = await fetchPatch<UserResponse>(form.action, payload);
        if (res.body) {
            success = true;
            user = res.body;
            onSave();

            setTimeout(() => {
                success = false;
            }, 3000);
        } else {
            err = res.error?.message || 'Error';
        }
    }
</script>

{#if user}
    <div class="picture">
        <UserPicture
            fallbackCharacters={fallbackCharacters(user)}
            userId={user.id}
            pictureId={user.picture_id}
            size="large"
        />
    </div>

    <Form
        action={`/auth/v1/users/${user.id}`}
        {onSubmit}
    >
        <div class="values">
            <div>
                <LabeledValue
                    label="ID"
                    mono
                >
                    {user.id}
                </LabeledValue>
            </div>
            <div>
                <LabeledValue label={t.account.accType}>
                    {user.account_type}
                    {#if providerName}
                        : {providerName}
                    {/if}
                </LabeledValue>
            </div>
        </div>

        <div class="values">
            {#if isSelf}
                <div>
                    <Tooltip text={`${ta.users.antiLockout.rule}: ${ta.users.antiLockout.disable}`}>
                        <InputCheckbox
                            ariaLabel={ta.common.enabled}
                            bind:checked={enabled}
                            disabled
                        >
                            {ta.common.enabled}
                        </InputCheckbox>
                    </Tooltip>
                </div>
                <div>
                    <Tooltip text={`${ta.users.antiLockout.rule}: ${ta.users.antiLockout.disable}`}>
                        <InputCheckbox
                            ariaLabel={t.account.emailVerified}
                            bind:checked={emailVerified}
                            disabled
                        >
                            {t.account.emailVerified}
                        </InputCheckbox>
                    </Tooltip>
                </div>
            {:else}
                <div>
                    <InputCheckbox
                        ariaLabel={ta.common.enabled}
                        bind:checked={enabled}
                    >
                        {ta.common.enabled}
                    </InputCheckbox>
                </div>
                <div>
                    <InputCheckbox
                        ariaLabel={t.account.emailVerified}
                        bind:checked={emailVerified}
                    >
                        {t.account.emailVerified}
                    </InputCheckbox>
                </div>
            {/if}
        </div>

        <div class="values">
            <div>
                <Input
                    typ="email"
                    bind:value={email}
                    autocomplete="off"
                    label="E-Mail"
                    placeholder="E-Mail"
                    required
                />

                <Input
                    bind:value={givenName}
                    autocomplete="off"
                    label={t.account.givenName}
                    placeholder={t.account.givenName}
                    required={config.given_name === 'required'}
                    pattern={PATTERN_USER_NAME}
                />
                <Input
                    bind:value={familyName}
                    autocomplete="off"
                    label={t.account.familyName}
                    placeholder={t.account.familyName}
                    required={config.family_name === 'required'}
                    pattern={PATTERN_USER_NAME}
                />

                <InputDateTimeCombo
                    label={t.account.birthdate}
                    bind:value={birthdate}
                    required={config.birthdate === 'required'}
                    withDelete
                />
                <TZSelect bind:value={tz} />
                <PreferredUsername
                    userId={user.id}
                    bind:preferred_username={user.user_values.preferred_username}
                    config={config.preferred_username}
                    isAdmin
                />

                {#if languages}
                    <div style:padding=".25rem">
                        <LabeledValue label={ta.common.language}>
                            <Options
                                ariaLabel={t.common.selectI18n}
                                options={languages}
                                bind:value={language}
                                borderless
                            />
                        </LabeledValue>
                    </div>
                {/if}
            </div>

            <div>
                <Input
                    bind:value={street}
                    autocomplete="off"
                    label={t.account.street}
                    placeholder={t.account.street}
                    required={config.street === 'required'}
                    pattern={PATTERN_STREET}
                />
                <Input
                    bind:value={zip}
                    autocomplete="off"
                    label={t.account.zip}
                    placeholder={t.account.zip}
                    required={config.zip === 'required'}
                    maxLength={24}
                    pattern={PATTERN_ALNUM}
                />
                <Input
                    bind:value={city}
                    autocomplete="off"
                    label={t.account.city}
                    placeholder={t.account.city}
                    required={config.city === 'required'}
                    pattern={PATTERN_CITY}
                />
                <Input
                    bind:value={country}
                    autocomplete="off"
                    label={t.account.country}
                    placeholder={t.account.country}
                    required={config.country === 'required'}
                    pattern={PATTERN_CITY}
                />

                <Input
                    bind:value={phone}
                    autocomplete="off"
                    label={t.account.phone}
                    placeholder={t.account.phone}
                    required={config.phone === 'required'}
                    pattern={PATTERN_PHONE}
                />
            </div>
        </div>

        <SelectList bind:items={rolesItems}>
            {t.account.roles.replaceAll(',', ' ')}
        </SelectList>
        <SelectList bind:items={groupsItems}>
            {t.account.groups.replaceAll(',', ' ')}
        </SelectList>

        <div class="values">
            <div style:margin-top=".5rem">
                {#if !expires && isSelf}
                    <Tooltip text={`${ta.users.antiLockout.rule}: ${ta.users.antiLockout.disable}`}>
                        <InputCheckbox
                            ariaLabel={t.account.accessExp}
                            bind:checked={expires}
                            disabled
                        >
                            {t.account.accessExp}
                        </InputCheckbox>
                    </Tooltip>
                {:else}
                    <InputCheckbox
                        ariaLabel={t.account.accessExp}
                        bind:checked={expires}
                    >
                        {t.account.accessExp}
                    </InputCheckbox>
                {/if}
            </div>
            {#if expires}
                <div transition:slide={{ duration: 150 }}>
                    <InputDateTimeCombo
                        label={t.account.accessExp}
                        bind:value={expDate}
                        bind:timeValue={expTime}
                        withTime
                        min={fmtDateInput()}
                        required
                    />
                </div>
            {/if}
        </div>

        <div class="values">
            <div>
                <LabeledValue label={t.account.userCreated}>
                    {formatDateFromTs(user.created_at)}
                </LabeledValue>
                <LabeledValue label={ta.users.lastLogin}>
                    {#if user.last_login}
                        {formatDateFromTs(user.last_login)}
                    {:else}
                        {t.common.never}
                    {/if}
                </LabeledValue>
            </div>

            <div>
                <LabeledValue label={t.account.passwordExpiry}>
                    {#if user.password_expires}
                        {formatDateFromTs(user.password_expires)}
                    {:else}
                        {t.common.never}
                    {/if}
                </LabeledValue>
                <LabeledValue label={t.account.mfaActivated}>
                    <CheckIcon checked={!!user.webauthn_user_id} />
                </LabeledValue>
            </div>
        </div>

        <div class="btn">
            <Button type="submit">
                {t.common.save}
            </Button>

            {#if success}
                <IconCheck />
            {/if}
        </div>

        {#if err}
            <div class="err">
                {err}
            </div>
        {/if}
    </Form>
{/if}

<style>
    .btn {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        margin-bottom: 1rem;
    }

    .picture {
        margin: 1rem 0;
    }

    .values {
        display: flex;
        gap: 1rem;
        flex-wrap: wrap;
    }

    .values > div {
        /* half the size of <p> max-width */
        width: min(calc(234pt - 0.5rem), calc(100dvw - 0.5rem));
    }

    @media (max-width: 467pt) {
        .values > div {
            width: min(25rem, calc(100dvw - 0.5rem));
        }
    }
</style>
