<script lang="ts">
    import { useI18n } from "$state/i18n.svelte";
    import { onMount } from "svelte";
    import Loading from "./Loading.svelte";
    import type { MfaPurpose } from "$api/types/mfa";
    import type { OtpAdditionalData, OtpAuthFinishResult, OtpAuthStartResult } from "$mfa/otp/types";
    import { otpAuthFinish, otpAuthStart } from "$mfa/otp/authentication";
    import Input from "./form/Input.svelte";
    import { TPL_OTP_LENGTH } from "$utils/constants";
    import Template from "./Template.svelte";
    import { PATTERN_OTP_CODE } from "$utils/patterns";
    import Form from "./form/Form.svelte";
    import Button from "./button/Button.svelte";
    import type { ActiveOtp } from "$api/types/authorize";
    import type { OtpResponse } from "$api/types/otp";

    let {
        userId,
        activeOtps,
        purpose,
        onError,
        onSuccess,
    }: {
        userId: string;
        activeOtps: ActiveOtp[] | OtpResponse[];
        purpose: MfaPurpose;
        onError: (error: string) => void;
        onSuccess: (res?: OtpAdditionalData) => void;
    } = $props();

    let t = useI18n();
    let refInput: undefined | HTMLInputElement = $state();
    let isInputError = $state(false);

    let otpSize = $state(6);

    let otpStartRes: undefined | OtpAuthStartResult = $state();
    let otpFinishRes: undefined | OtpAuthFinishResult = $state();

    // todo: The current implementation only allows one kind of OTP to be active, and the only kind is email.
    // Since we could have multiple OTPs in the future, we should the allow users to select which OTP they want to use.
    onMount(async () => {
        let otpId;
        if ('otp_id' in activeOtps[0]) {
            otpId = activeOtps[0].otp_id;
        } else {
            otpId = activeOtps[0].id.toString();
        }
        otpStartRes = await otpAuthStart(
            userId,
            otpId,
            purpose,
        );
    })

    $effect(() => {
        if (otpStartRes) {
            if (otpStartRes.error) {
                setTimeout(() => {
                    onError(otpStartRes?.error || 'OTP Error');
                }, 3000)
            }
        }
    })

    $effect(() => {
        if (otpFinishRes) {
            if (otpFinishRes.error) {
                setTimeout(() => {
                    onError(otpFinishRes?.error || 'OTP Error');
                }, 3000)
            } else {
                onSuccess(otpFinishRes.data);
            }
        }
    })

    $effect(() => {
        refInput?.focus();
    });

    async function onLoginOtpSubmit(_form: HTMLFormElement, params: URLSearchParams) {
        let otpCode = params.get('otp');
        if (otpStartRes && otpStartRes.data && otpCode) {
            otpFinishRes = await otpAuthFinish(
                userId,
                otpStartRes.data.code,
                otpCode,
            );

        }
    }

</script>

<Template id={TPL_OTP_LENGTH} bind:value={otpSize} />

{#if purpose}
    <div class="wrapperOuter">
        <div class="wrapperInner">
            <div class="content">
                <div class="contentRow">
                    <div class="contentHeader">
                        {t.authorize.expectingOtp}
                    </div>
                </div>

                <div class="contentRow">
                    <div>
                        {#if !otpStartRes}
                            <Loading />
                        {/if}
                    </div>
                </div>

                <div class="contentRow">
                    {#if otpStartRes}
                        {#if otpStartRes.error}
                            <div class="err">
                                {otpStartRes.error}
                            </div>
                        {:else}
                            <div class="good">
                                <Form action="" onSubmit={onLoginOtpSubmit}>
                                    <Input
                                        bind:ref={refInput}
                                        name="otp"
                                        autocomplete='one-time-code'
                                        label={t.mfa.otp.code}
                                        placeholder={'0'.repeat(otpSize)}
                                        maxLength={otpSize}
                                        minLength={otpSize}
                                        pattern={PATTERN_OTP_CODE}
                                        bind:isError={isInputError}
                                    />
                                    <Button type='submit'>send</Button>
                                </Form>
                            </div>
                        {/if}
                    {/if}
                </div>
            </div>
        </div>
    </div>
{/if}

<style>
    .content {
        padding: 1rem;
        border: 1px solid hsl(var(--bg-high));
        border-radius: var(--border-radius);
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        color: hsl(var(--text-high));
        text-align: center;
        z-index: 99;
        background: hsla(var(--bg) / 0.9);
    }

    .contentRow {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        margin: 0.25em;
    }

    .contentHeader {
        margin-bottom: 0.2em;
        font-weight: bold;
    }

    .err,
    .good {
        font-weight: bold;
    }

    .good {
        color: hsl(var(--action));
    }

    /*.muted {*/
    /*    color: hsla(var(--text) / .8)*/
    /*}*/

    /*progress {*/
    /*    accent-color: hsl(var(--accent));*/
    /*}*/

    .wrapperOuter {
        position: absolute;
        top: 0;
        left: 0;
    }

    .wrapperInner {
        width: 100vw;
        height: 100vh;
        position: relative;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        background: rgba(0, 0, 0, 0.85);
        z-index: 20;
    }
</style>
