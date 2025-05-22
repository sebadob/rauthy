<script lang="ts">
    import {formatDateFromTs} from "$utils/helpers.js";
    import Button from "$lib5/button/Button.svelte";
    import {untrack} from "svelte";
    import CheckIcon from "$lib5/CheckIcon.svelte";
    import Input from "$lib5/form/Input.svelte";
    import type {UpdateUserRequest, UserResponse, UserResponseSimple} from "$api/types/user.ts";
    import type {RoleResponse} from "$api/types/roles.ts";
    import type {GroupResponse} from "$api/types/groups.ts";
    import type {SelectItem} from "$lib5/select_list/props.ts";
    import {fmtDateInput, fmtTimeInput, unixTsFromLocalDateTime} from "$utils/form";
    import {fetchPatch, fetchPut} from "$api/fetch";
    import Form from "$lib5/form/Form.svelte";
    import IconCheck from "$icons/IconCheck.svelte";
    import {useI18n} from "$state/i18n.svelte";
    import {useI18nAdmin} from "$state/i18n_admin.svelte";
    import LabeledValue from "$lib5/LabeledValue.svelte";
    import InputCheckbox from "$lib5/form/InputCheckbox.svelte";
    import {PATTERN_CITY, PATTERN_PHONE, PATTERN_STREET, PATTERN_USER_NAME} from "$utils/patterns";
    import Options from "$lib5/Options.svelte";
    import SelectList from "$lib5/select_list/SelectList.svelte";
    import InputDateTimeCombo from "$lib5/form/InputDateTimeCombo.svelte";
    import {slide} from "svelte/transition";
    import type {Language} from "$api/types/i18n.ts";
    import {useI18nConfig} from "$state/i18n_config.svelte";
    import UserPicture from "$lib/UserPicture.svelte";
    import type {PatchOp} from "$api/types/generic";

    let {
        user = $bindable(),
        roles,
        groups,
        onSave,
    }: {
        user: UserResponse,
        roles: RoleResponse[],
        groups: GroupResponse[],
        onSave: () => void,
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    let err = $state('');
    let success = $state(false);

    let userOrig: undefined | UserResponse;

    let email = $state('');
    let givenName = $state('');
    let familyName = $state('');
    let languages = $derived(useI18nConfig().common()?.map(l => l as string));
    let language: Language = $state('en');

    let birthdate = $state('');
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

    let rolesItems: SelectItem[] = $state(untrack(() => roles.map(r => {
        let i: SelectItem = {
            name: r.name,
            selected: false,
        };
        return i;
    })));
    let groupsItems: SelectItem[] = $state(untrack(() => groups.map(g => {
        let i: SelectItem = {
            name: g.name,
            selected: false,
        };
        return i;
    })));

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
                user_values: {
                    birthdate: user.user_values?.birthdate || '',
                    phone: user.user_values?.phone || '',
                    street: user.user_values?.street || '',
                    zip: user.user_values?.zip,
                    city: user.user_values?.city || '',
                    country: user.user_values?.country || '',
                }
            }

            email = user.email;
            givenName = user.given_name;
            familyName = user.family_name || '';
            language = user.language;

            birthdate = user.user_values?.birthdate || '';
            phone = user.user_values?.phone || '';
            street = user.user_values?.street || '';
            zip = user.user_values?.zip?.toString() || '';
            city = user.user_values?.city || '';
            country = user.user_values?.country || '';

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
        let chars = user.given_name[0];
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
            payload.put.push({key: 'email', value: email});
        }
        if (givenName !== userOrig?.given_name) {
            payload.put.push({key: 'given_name', value: givenName});
        }
        if (familyName !== userOrig?.family_name) {
            if (familyName) {
                payload.put.push({key: 'family_name', value: familyName});
            } else {
                payload.del.push('family_name');
            }
        }
        if (language !== userOrig?.language) {
            if (language) {
                payload.put.push({key: 'language', value: language});
            } else {
                payload.del.push('language');
            }
        }
        let roles = rolesItems.filter(i => i.selected).map(i => i.name);
        if (roles.join(',') !== userOrig?.roles?.join(',')) {
            if (roles.length > 0) {
                payload.put.push({key: 'roles', value: roles});
            } else {
                payload.del.push('roles');
            }
        }
        let groups = groupsItems.filter(i => i.selected).map(i => i.name);
        if (groups.join(',') !== userOrig?.groups?.join(',')) {
            if (groups.length > 0) {
                payload.put.push({key: 'groups', value: groups});
            } else {
                payload.del.push('groups');
            }
        }
        if (enabled !== userOrig?.enabled) {
            payload.put.push({key: 'enabled', value: enabled});
        }
        if (emailVerified !== userOrig?.email_verified) {
            payload.put.push({key: 'email_verified', value: emailVerified});
        }
        let exp = unixTsFromLocalDateTime(expDate, expTime);
        if (exp !== userOrig?.user_expires) {
            if (expires) {
                payload.put.push({key: 'user_expires', value: exp});
            } else {
                payload.del.push('user_expires');
            }
        }

        if (birthdate !== userOrig?.user_values?.birthdate) {
            if (birthdate) {
                payload.put.push({key: 'user_values.birthdate', value: birthdate});
            } else {
                payload.del.push('user_values.birthdate');
            }
        }
        if (phone !== userOrig?.user_values?.phone) {
            if (phone) {
                payload.put.push({key: 'user_values.phone', value: phone});
            } else {
                payload.del.push('user_values.phone');
            }
        }
        if (street !== userOrig?.user_values?.street) {
            if (street) {
                payload.put.push({key: 'user_values.street', value: street});
            } else {
                payload.del.push('user_values.street');
            }
        }
        let zp = zip ? Number.parseInt(zip) : null;
        if (zp !== userOrig?.user_values?.zip) {
            if (zip) {
                payload.put.push({key: 'user_values.zip', value: zp});
            } else {
                payload.del.push('user_values.zip');
            }
        }
        if (city !== userOrig?.user_values?.city) {
            if (city) {
                payload.put.push({key: 'user_values.city', value: city});
            } else {
                payload.del.push('user_values.city');
            }
        }
        if (country !== userOrig?.user_values?.country) {
            if (country) {
                payload.put.push({key: 'user_values.country', value: country});
            } else {
                payload.del.push('user_values.country');
            }
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

    <Form action={`/auth/v1/users/${user.id}`} {onSubmit}>
        <LabeledValue label="ID" mono>
            {user.id}
        </LabeledValue>

        <div class="values">
            <div>
                <InputCheckbox ariaLabel={ta.common.enabled} bind:checked={enabled}>
                    {ta.common.enabled}
                </InputCheckbox>
            </div>
            <div>
                <InputCheckbox ariaLabel={t.account.emailVerified} bind:checked={emailVerified}>
                    {t.account.emailVerified}
                </InputCheckbox>
            </div>
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
                        required
                        pattern={PATTERN_USER_NAME}
                />
                <Input
                        bind:value={familyName}
                        autocomplete="off"
                        label={t.account.familyName}
                        placeholder={t.account.familyName}
                        pattern={PATTERN_USER_NAME}
                />

                <InputDateTimeCombo
                        label={t.account.birthdate}
                        bind:value={birthdate}
                        withDelete
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
                        pattern={PATTERN_STREET}
                />
                <Input
                        typ="number"
                        bind:value={zip}
                        autocomplete="off"
                        label={t.account.zip}
                        placeholder={t.account.zip}
                        min="1000"
                        max="9999999"
                />
                <Input
                        bind:value={city}
                        autocomplete="off"
                        label={t.account.city}
                        placeholder={t.account.city}
                        pattern={PATTERN_CITY}
                />
                <Input
                        bind:value={country}
                        autocomplete="off"
                        label={t.account.country}
                        placeholder={t.account.country}
                        pattern={PATTERN_CITY}
                />

                <Input
                        bind:value={phone}
                        autocomplete="off"
                        label={t.account.phone}
                        placeholder={t.account.phone}
                        pattern={PATTERN_PHONE}
                />
            </div>
        </div>

        <SelectList bind:items={rolesItems}>
            {t.account.roles}
        </SelectList>
        <SelectList bind:items={groupsItems}>
            {t.account.groups}
        </SelectList>

        <div class="values">
            <div style:margin-top=".5rem">
                <InputCheckbox ariaLabel={t.account.accessExp} bind:checked={expires}>
                    {t.account.accessExp}
                </InputCheckbox>
            </div>
            {#if expires}
                <div transition:slide={{duration: 150}}>
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
                    <CheckIcon checked={!!user.webauthn_user_id}/>
                </LabeledValue>
            </div>
        </div>

        <div class="btn">
            <Button type="submit">
                {t.common.save}
            </Button>

            {#if success}
                <IconCheck/>
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
        gap: .5rem;
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
        width: min(calc(234pt - .5rem), calc(100dvw - .5rem));
    }

    @media (max-width: 467pt) {
        .values > div {
            width: min(25rem, calc(100dvw - .5rem));
        }
    }
</style>
