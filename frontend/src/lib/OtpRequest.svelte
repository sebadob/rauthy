<script lang="ts">
    import { useI18n } from '$state/i18n.svelte';
    import { onMount } from 'svelte';
    import Loading from './Loading.svelte';
    import type { MfaPurpose } from '$api/types/mfa';
    import type {
        OtpAdditionalData,
        OtpAuthFinishResult,
        OtpAuthStartResult,
    } from '$mfa/otp/types';
    import { otpAuthFinish, otpAuthResend, otpAuthStart } from '$mfa/otp/authentication';
    import { TPL_OTP_LENGTH } from '$utils/constants';
    import Template from './Template.svelte';
    import Form from './form/Form.svelte';
    import Button from './button/Button.svelte';
    import type { ActiveOtp } from '$api/types/authorize';
    import type { OtpResponse } from '$api/types/otp';
    import InputOtp from './form/InputOtp.svelte';
    import Modal from './Modal.svelte';

    let {
        activeOtps,
        purpose,
        onError,
        onSuccess,
        // onCancel,
    }: {
        activeOtps: ActiveOtp[] | OtpResponse[];
        purpose: MfaPurpose;
        onError: (error: string) => void;
        onSuccess: (res?: OtpAdditionalData) => void;
        // onCancel: () => void;
    } = $props();

    let t = useI18n();
    let refInput: undefined | HTMLInputElement = $state();
    let isInputError = $state(false);

    let otpSize = $state(6);
    let otpValue = $state('');

    let otpStartRes: undefined | OtpAuthStartResult = $state();
    let otpFinishRes: undefined | OtpAuthFinishResult = $state();

    let requestCoolDown: boolean = $state(false);

    let showModal = $state(false);
    let closeModal: undefined | (() => void) = $state();
    let onModalClose = async () => {
        if (otpStartRes?.data) {
            otpStartRes.data = undefined;
        }
        otpStartRes = undefined;
        showModal = false;
        clearInterval(interval);
        interval = undefined;
        onError(t.mfa.requestCanceled);
    };

    // todo: The current implementation only allows one kind of OTP to be active, and the only kind is email.
    // Since we could have multiple OTPs in the future, we should the allow users to select which OTP they want to use.
    onMount(async () => {
        let otpId;
        if ('otp_id' in activeOtps[0]) {
            otpId = activeOtps[0].otp_id;
        } else {
            otpId = activeOtps[0].id.toString();
        }
        otpStartRes = await otpAuthStart(otpId, purpose);
        if (otpStartRes.data) {
            if (interval) {
                clearInterval(interval);
            }
            showModal = true;
            calcTimeoutSecs();
            interval = window.setInterval(() => {
                calcTimeoutSecs();
            }, 1000);
        }
    });

    $effect(() => {
        if (otpStartRes) {
            if (otpStartRes.error) {
                setTimeout(() => {
                    onError(otpStartRes?.error || 'OTP Error');
                }, 3000);
            }
        }
    });

    $effect(() => {
        if (otpFinishRes) {
            if (otpFinishRes.error) {
                setTimeout(() => {
                    onError(otpFinishRes?.error || 'OTP Error');
                }, 3000);
            } else {
                onSuccess(otpFinishRes.data);
            }
        }
    });

    $effect(() => {
        if (refInput) {
            requestAnimationFrame(() => {
                refInput?.focus();
            });
        }
    });

    $effect(() => {
        if (otpValue.replaceAll(' ', '').length === otpSize) {
            onLoginOtpSubmit();
        }
    });

    async function onRequestNewOtp() {
        if (otpStartRes && otpStartRes.data && !requestCoolDown) {
            let res = await otpAuthResend(otpStartRes.data.code);
            if (!res.error) {
                requestCoolDown = true;
                setTimeout(() => {
                    requestCoolDown = false;
                }, 6000);
            }
        }
    }

    async function onLoginOtpSubmit(form?: HTMLFormElement, params?: URLSearchParams) {
        let otpCode = otpValue.replace(' ', '');
        if (otpStartRes && otpStartRes.data && otpCode) {
            otpFinishRes = await otpAuthFinish(otpStartRes.data.code, otpCode);
        }
    }

    let otpTimeoutSecs: undefined | number = $state();
    let interval: number | undefined = $state();
    function calcTimeoutSecs() {
        let timeoutSecs = 0;
        if (otpStartRes?.data) {
            let ts = new Date().getTime() / 1000;
            timeoutSecs = Math.floor(otpStartRes.data.exp - ts);
        }
        if (timeoutSecs > 0) {
            otpTimeoutSecs = timeoutSecs;
        } else {
            otpTimeoutSecs = undefined;
            if (otpStartRes?.data) {
                otpStartRes.data = undefined;
            }
            showModal = false;
            clearInterval(interval);
            interval = undefined;
            onError(t.mfa.otp.sessionExpired);
        }
    }
</script>

<Template id={TPL_OTP_LENGTH} bind:value={otpSize} />

{#if purpose}
    <Modal bind:showModal bind:closeModal onClose={onModalClose}>
        <div class="content">
            <h2>
                {t.authorize.expectingOtp}
            </h2>

            <div>
                {#if !otpStartRes}
                    <Loading />
                {/if}
            </div>

            {#if otpStartRes && otpStartRes.error}
                <div class="err">
                    {otpStartRes.error}
                </div>
            {:else}
                <div class="good">
                    <Form action="" onSubmit={onLoginOtpSubmit}>
                        <div class="spacing">
                            <InputOtp
                                bind:ref={refInput}
                                bind:isError={isInputError}
                                bind:value={otpValue}
                            />
                        </div>
                        <div class="btns">
                            <Button level={1} type="submit">Ok</Button>
                            <Button level={2} onclick={onRequestNewOtp} isLoading={requestCoolDown}>
                                {t.mfa.otp.resendOtp}
                            </Button>
                        </div>
                    </Form>
                </div>
                {#if otpTimeoutSecs && otpTimeoutSecs > 0}
                    <div class="spacing">
                        {t.mfa.otp.sessionExpiresIn}
                        <span>
                            {otpTimeoutSecs}
                            {t.common.seconds}
                        </span>
                    </div>
                {/if}
            {/if}
        </div>
    </Modal>
{/if}

<style>
    .btns {
        margin: 1rem 0;
    }

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

    .spacing {
        margin: 1rem 0;
    }

    .err,
    .good {
        font-weight: bold;
    }

    .good {
        color: hsl(var(--action));
    }
</style>
