<script lang="ts">
    import Input from "$lib/form/Input.svelte";
    import type {UserValuesPreferredUsername} from "$api/templates/UserValuesConfig";
    import {useI18n} from "$state/i18n.svelte";
    import Button from "$lib/button/Button.svelte";
    import Modal from "$lib/Modal.svelte";
    import {fetchPut} from "$api/fetch";
    import type {PreferredUsernameRequest} from "$api/types/user";
    import {slide} from "svelte/transition";
    import Tooltip from "$lib/Tooltip.svelte";
    import IconInfo from "$icons/IconInfo.svelte";
    import InputCheckbox from "$lib/form/InputCheckbox.svelte";
    import Form from "$lib/form/Form.svelte";

    let {
        userId,
        preferred_username = $bindable(),
        config,
        isAdmin
    }: {
        userId: string,
        preferred_username: string | undefined,
        config: UserValuesPreferredUsername,
        isAdmin?: boolean,
    } = $props();

    let t = useI18n();

    let ref: undefined | HTMLInputElement = $state();
    let showModal = $state(false);
    let closeModal: undefined | (() => void) = $state();

    let username = $state('');
    let forceOverwrite = $state(false);
    let nameExists = $state(false);
    let required = $derived(config.preferred_username === 'required');

    $effect(() => {
        username = preferred_username || '';
        forceOverwrite = false;
    });

    $effect(() => {
        if (ref) {
            requestAnimationFrame(() => {
                ref?.focus();
            });
        }
    });

    async function onSubmit(form: HTMLFormElement, params: URLSearchParams) {
        if (config.immutable && !forceOverwrite) {
            return;
        }

        let payload: PreferredUsernameRequest = {
            preferred_username: username || undefined,
            force_overwrite: forceOverwrite || undefined,
        };
        let res = await fetchPut(form.action, payload);
        if (res.status === 200) {
            closeModal?.();
            preferred_username = username;
            forceOverwrite = false;
        } else if (res.status === 406) {
            nameExists = true;
        } else {
            console.error(res);
        }
    }

</script>

{#if config.preferred_username !== 'hidden'}
    <div class="container">
        <div class="value" data-required={required && !preferred_username}>
            <div class="name">
                {preferred_username || '-'}
            </div>

            <div class="btn">
                {#if isAdmin || preferred_username?.length === 0 || !config.immutable}
                    <Button
                            ariaLabel={t.common.edit}
                            level={required && !preferred_username ? -1 : 3}
                            onclick={() => showModal = true}
                    >
                        {t.common.edit}
                    </Button>

                    <Modal bind:showModal bind:closeModal>
                        <h3>{t.account.preferredUsername.preferredUsername}</h3>

                        <Form action={`/auth/v1/users/${userId}/self/preferred_username`} {onSubmit}>
                            <Input
                                    bind:ref
                                    name="preferred_username"
                                    autocomplete="off"
                                    label={t.account.preferredUsername.preferredUsername}
                                    placeholder={t.account.preferredUsername.preferredUsername}
                                    bind:value={username}
                                    required={required}
                                    isError={nameExists}
                                    errMsg={nameExists ? t.account.preferredUsername.notAvailable : ''}
                                    pattern={config.pattern_html}
                                    width="15rem"
                                    onInput={() => nameExists = false}
                            />

                            <div class="desc">
                                <p>{t.account.preferredUsername.desc}</p>
                                {#if config.immutable}
                                    <p class="caution">{t.account.preferredUsername.immutable}</p>
                                {/if}
                            </div>

                            {#if config.immutable && isAdmin}
                                <InputCheckbox
                                        ariaLabel={t.account.preferredUsername.forceOverwrite}
                                        bind:checked={forceOverwrite}
                                >
                                    {t.account.preferredUsername.forceOverwrite}
                                </InputCheckbox>
                            {/if}

                            {#if !config.immutable || forceOverwrite && isAdmin}
                                <div class="btnModal" transition:slide={{ duration: 150 }}>
                                    <Button type="submit">
                                        {t.common.save}
                                    </Button>
                                    <Button level={-3} onclick={() => closeModal?.()}>
                                        {t.common.cancel}
                                    </Button>
                                </div>
                            {/if}
                        </Form>
                    </Modal>
                {:else}
                    <Tooltip text={t.account.preferredUsername.immutableInfo}>
                        <div class="immutableInfo">
                            <IconInfo width="1.2rem"/>
                        </div>
                    </Tooltip>
                {/if}
            </div>
        </div>

        <div class="label" data-required={required}>
            {t.account.preferredUsername.preferredUsername}
        </div>
    </div>
{/if}

<style>
    .btnModal {
        margin-top: 1rem;
        display: flex;
        gap: .5rem;
    }

    .caution {
        font-weight: bold;
        color: hsl(var(--error));
    }

    .container {
        margin: .75rem 0;
    }

    .desc {
        width: 15rem;
    }

    .immutableInfo {
        color: hsla(var(--text) / .5);
        transform: translateY(.25rem);
    }

    .label {
        margin-top: -.25rem;
        margin-left: .5rem;
        color: hsl(var(--text) / .8);
        font-size: .9rem;
    }

    .label[data-required="true"]:after {
        content: ' *';
        color: hsla(var(--error) / .9);
    }

    .name {
        padding-left: .5rem;
    }

    .value {
        height: 2rem;
        display: flex;
        justify-content: space-between;
        border-bottom: 1px solid hsl(var(--bg-high));
    }

    .value[data-required=true] {
        border-bottom: 1px solid hsl(var(--error));
    }
</style>
