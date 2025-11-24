<script lang="ts">
    import Input from "$lib/form/Input.svelte";
    import type {UserValuesPreferredUsername} from "$api/templates/UserValuesConfig";
    import {useI18n} from "$state/i18n.svelte";
    import Button from "$lib/button/Button.svelte";
    import Modal from "$lib/Modal.svelte";
    import {fetchPut} from "$api/fetch";
    import type {PreferredUsernameRequest} from "$api/types/user";
    import {untrack} from "svelte";
    import Tooltip from "$lib/Tooltip.svelte";
    import IconInfo from "$icons/IconInfo.svelte";

    let {
        userId,
        preferred_username = $bindable(''),
        config,
    }: {
        userId: string,
        preferred_username: string | undefined,
        config: UserValuesPreferredUsername,
    } = $props();

    let t = useI18n();

    let ref: undefined | HTMLInputElement = $state();
    let showModal = $state(false);
    let closeModal: undefined | (() => void) = $state();

    let username = $state(untrack(() => preferred_username));
    let required = $derived(config.preferred_username === 'required');

    $effect(() => {
        if (ref) {
            requestAnimationFrame(() => {
                ref?.focus();
            });
        }
    });

    async function save() {
        let payload: PreferredUsernameRequest = {
            preferred_username: username || undefined,
        };
        let res = await fetchPut(`/auth/v1/users/${userId}/self/preferred_username`, payload);
        if (res.status === 200) {
            closeModal?.();
            preferred_username = username;
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
                {#if preferred_username.length === 0 || !config.immutable}
                    <Button
                            ariaLabel={t.common.edit}
                            level={required ? -1 : 3}
                            onclick={() => showModal = true}
                    >
                        {t.common.edit}
                    </Button>

                    <Modal bind:showModal bind:closeModal>
                        <h3>{t.account.preferredUsername}</h3>

                        <Input
                                bind:ref
                                name="preferred_username"
                                autocomplete="off"
                                label={t.account.preferredUsername}
                                placeholder={t.account.preferredUsername}
                                bind:value={username}
                                required
                                pattern={config.pattern_html}
                                width="15rem"
                        />

                        <div class="desc">
                            <p>{t.account.preferredUsernameDesc}</p>
                            {#if config.immutable}
                                <p class="caution">{t.account.preferredUsernameImmutable}</p>
                            {/if}
                        </div>

                        <div class="btnModal">
                            <Button onclick={save}>
                                {t.common.save}
                            </Button>
                            <Button level={-3} onclick={() => closeModal?.()}>
                                {t.common.cancel}
                            </Button>
                        </div>
                    </Modal>
                {:else}
                    <Tooltip text={t.account.preferredUsernameImmutableInfo}>
                        <div class="immutableInfo">
                            <IconInfo width="1.2rem"/>
                        </div>
                    </Tooltip>
                {/if}
            </div>
        </div>

        <div class="label" data-required={required}>
            {t.account.preferredUsername}
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
