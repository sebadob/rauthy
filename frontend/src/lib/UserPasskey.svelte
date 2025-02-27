<script lang="ts">
    import type {PasskeyResponse} from "$api/types/webauthn.ts";
    import {formatDateFromTs} from "$utils/helpers.ts";
    import IconFingerprint from "$icons/IconFingerprint.svelte";
    import Tooltip from "$lib5/Tooltip.svelte";
    import Button from "$lib5/button/Button.svelte";
    import {useI18n} from "$state/i18n.svelte.ts";

    let {
        passkey,
        showDelete,
        onDelete,
    }: {
        passkey: PasskeyResponse,
        showDelete: boolean,
        onDelete: (name: string) => void,
    } = $props();

    let t = useI18n();
</script>

<div class="keyContainer">
    <div class="row">
        <span class="label">
            {t.mfa.passkeyName}
        </span>

        <div class="nameUv">
            <b>{passkey.name}</b>
            {#if passkey.user_verified}
                <Tooltip text={t.account.userVerifiedTooltip}>
                    <div style:margin-bottom="-.25rem">
                        <IconFingerprint width={18} color="var(--col-acnt)"/>
                    </div>
                </Tooltip>
            {/if}
        </div>
    </div>
    <div class="row">
        <span class="label">
            {t.mfa.registerd}
        </span>
        <span class="font-mono">{formatDateFromTs(passkey.registered)}</span>
    </div>
    <div class="row">
        <span class="label">
            {t.mfa.lastUsed}
        </span>
        <span class="font-mono">{formatDateFromTs(passkey.last_used)}</span>
    </div>

    {#if showDelete}
        <div class="row">
            <div></div>
            <div class="deleteBtn">
                <Button
                        level={-3}
                        onclick={() => onDelete(passkey.name)}
                >
                    {t.common.delete}
                </Button>
            </div>
        </div>
    {/if}
</div>

<style>
    .deleteBtn {
        margin-right: -.35rem;
    }

    .keyContainer {
        margin: .33rem 0;
        overflow: clip;
    }

    .label {
        color: hsla(var(--text) / .7);
    }

    .nameUv {
        display: inline-flex;
        align-items: center;
        gap: .25rem;
    }

    .row {
        width: min(95dvw, 21rem);
        display: flex;
        gap: .5rem;
        justify-content: space-between;
        align-items: center;
    }
</style>
