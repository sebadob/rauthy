<script lang="ts">
    import Button from '#lib/button/Button.svelte';
    import { onMount, untrack } from 'svelte';
    import MfaMethod from '#lib/MfaMethod.svelte';
    import { I18nEn } from '../../i18n/common/en';
    import Input from '#lib/form/Input.svelte';
    import { useI18n } from '#state/i18n.svelte.js';
    import { useSession } from '#state/session.svelte.js';
    import { fetchDelete, fetchGet, fetchPost } from '#api/fetch.ts';
    import type { PasskeyResponse, WebauthnDeleteRequest } from '#api/types/webauthn.ts';
    import type { UserResponse } from '#api/types/user.ts';
    import { PATTERN_USER_NAME } from '#utils/patterns.ts';
    import { webauthnReg } from '#mfa/webauthn/registration.ts';
    import WebauthnRequest from '#lib/WebauthnRequest.svelte';
    import type { WebauthnAdditionalData, WebauthnServiceReq } from '#mfa/webauthn/types.ts';
    import UserPasskey from '#lib/UserPasskey.svelte';
    import type { MfaModTokenResponse, UserMfaTokenRequest } from '#api/types/mfa_mod_token.ts';
    import Modal from '#lib/Modal.svelte';
    import InputPassword from '#lib/form/InputPassword.svelte';
    import Form from '#lib/form/Form.svelte';
    import IconArrowPathSquare from '#icons/IconArrowPathSquare.svelte';
    import Template from '#lib/Template.svelte';
    import { TPL_OTP_LENGTH, TPL_IS_OTP_ENABLED } from '#utils/constants.ts';
    import type { OtpResponse } from '#api/types/otp.ts';
    import { deleteOtp, postOtp, putOtp } from '#mfa/otp/mod.ts';
    import type { MfaPurpose } from '#api/types/mfa.ts';
    import type { OtpAdditionalData, OtpKind, OtpServiceReq } from '#mfa/otp/types.ts';
    import UserOtp from '#lib/UserOtp.svelte';
    import OtpRequest from '#lib/OtpRequest.svelte';
    import InputOtp from '#lib/form/InputOtp.svelte';
    import IconCheck from '#icons/IconCheck.svelte';
    import PasskeyTypeSelect from '#lib/PasskeyTypeSelect.svelte';

    let { user = $bindable() }: { user: UserResponse } = $props();

    const isWebauthnSupported = 'credentials' in navigator;

    let t = useI18n();
    let session = useSession('account');
    let userId = $derived(session.get()?.user_id);

    let refInput: undefined | HTMLInputElement = $state();
    let refPkAuthBtn: undefined | HTMLButtonElement = $state();

    let err = $state(false);
    let pwdErr = $state('');
    let msg = $state('');
    let showPasskeyInput = $state(false);
    let showOtpInput = $state(false);
    let showDelete = $state(untrack(() => user.account_type) === 'password');

    let mfaPurpose: undefined | MfaPurpose = $state();
    let mfaKind: undefined | 'webauthn' | 'otp' = $state();
    let passkeyName = $state('');
    let isInputError = $state(false);
    let isLoading = $state(false);
    let testSuccess = $state(false);

    let showModal = $state(false);
    let closeModal: undefined | (() => void) = $state();

    let passkeys: PasskeyResponse[] = $state([]);
    let loadingPasskeys = $state(true);
    let passkeyError = $state<string>();
    let residentKey = $state(false);
    let mfaModToken: undefined | MfaModTokenResponse = $state();
    let mfaModSecs: undefined | number = $state();
    let interval: undefined | number;

    let isOtpEnabled = $state(false);
    let otpSize = $state(6);
    let otps: OtpResponse[] = $state([]);
    let loadingOtps = $state(true);
    let otpError = $state<string>();
    let otpKind: undefined | OtpKind = $state();
    let otpName: undefined | string = $state();
    let otpId: undefined | string = $state();
    let otpValueNew = $state('');
    let hasOtp = $derived(otps.some(otp => otp.is_active));

    let isUserFederatedOnly = $derived(user.account_type === 'federated');

    type RegType = undefined | 'passkey' | 'otp';
    // Saves the type of the reg button the user clicked before fetching an MFA mod token, so that
    // we can resume that action after a successful fetch. It is consumed by the resume itself and
    // dropped when the re-authentication is aborted, so that an old click cannot trigger an action
    // after an unrelated successful fetch.
    let requestedBeforeTokenFetch: RegType = $state(undefined);

    onMount(() => {
        fetchPasskeys();
        return () => clearInterval(interval);
    });

    $effect(() => {
        if (isOtpEnabled) {
            fetchOtps();
        }
    });

    $effect(() => {
        if (passkeys.length > 0 && user.account_type === 'passkey') {
            showDelete = passkeys.length > 1;
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
        if (refPkAuthBtn) {
            requestAnimationFrame(() => {
                refPkAuthBtn?.focus();
            });
        }
    });

    let submittedOtp = '';

    $effect(() => {
        const code = otpValueNew.replaceAll(' ', '');
        if (mfaModToken && code.length === otpSize && code !== submittedOtp) {
            submittedOtp = code;
            handleActivateOtp();
        }
    });

    function calcModSecs() {
        let modSecs = 0;
        if (mfaModToken) {
            let ts = new Date().getTime() / 1000;
            modSecs = Math.floor(mfaModToken.exp - ts);
        }
        if (modSecs > 0) {
            mfaModSecs = modSecs;
        } else {
            mfaModSecs = undefined;
            mfaModToken = undefined;
            clearInterval(interval);
            interval = undefined;
        }
    }

    function resetMsgErr() {
        err = false;
        msg = '';
    }

    async function fetchPasskeys() {
        let res = await fetchGet<PasskeyResponse[]>(`/auth/v1/users/${user.id}/webauthn`);
        loadingPasskeys = false;
        passkeyError = res.body ? undefined : res.error?.message || 'Error';
        if (res.body) passkeys = res.body;
    }

    // `webauthn_user_id` is set by the backend with the very first passkey, and reset with the
    // last deletion. Refetch so the MFA indicator updates without a manual page reload.
    async function fetchUser() {
        let res = await fetchGet<UserResponse>(`/auth/v1/users/${user.id}`);
        if (res.body) {
            user = res.body;
        }
    }

    async function handleRegisterPasskey() {
        resetMsgErr();

        if (isInputError || !userId) {
            return;
        }
        if (passkeyName.length < 1) {
            err = true;
            msg = t.mfa.passkeyNameErr;
            return;
        }

        let tokenId = mfaModToken?.id;
        if (!tokenId) {
            showModal = true;
            return;
        }

        let res = await webauthnReg(
            userId,
            passkeyName,
            t.authorize.invalidKeyUsed,
            t.authorize.requestExpired,
            undefined,
            undefined,
            tokenId,
            residentKey,
        );
        if (res.error) {
            console.log(res.error.toLowerCase());
            // Either completely missing attestation or no matching requirements.
            if (res.error.toLowerCase().includes('missing attestation')) {
                msg = t.account.passkeys.missingAttestation;
            } else {
                msg = `${t.mfa.errorReg} - ${res.error}`;
            }
            err = true;
        } else {
            showPasskeyInput = false;
            passkeyName = '';
            await fetchPasskeys();
            await fetchUser();
        }
    }

    async function handleDeletePasskey(name: string) {
        if (!mfaModToken) {
            showModal = true;
            return;
        }

        let payload: WebauthnDeleteRequest = {
            mfa_mod_token_id: mfaModToken.id,
        };
        let res = await fetchDelete(
            `/auth/v1/users/${user.id}/webauthn/delete/${encodeURIComponent(name)}`,
            payload,
        );
        if (res.status === 200) {
            await fetchPasskeys();
            await fetchUser();
        } else {
            err = true;
            msg = res.error?.message || 'Error';
        }
    }

    async function onRegisterPasskeyClick(ty: RegType) {
        isInputError = false;
        requestedBeforeTokenFetch = ty;

        if (mfaModToken) {
            requestedBeforeTokenFetch = undefined;
            showPasskeyInput = true;
        } else if (isUserFederatedOnly) {
            // A federated user account does neither have a password, nor a passkey. We cannot
            // request additional authentication to get an MFA mod token, because there is none.
            // In this special case, the user is allowed to fetch such a token directly. Without it
            // it would be impossible to go from a federated to a federated_passkey account. You
            // would always need to add a password beforehand only to be able to authenticate
            // locally here.
            //
            // Note: a "perfect" solution would be to send this user an OTP, even when it's not
            // registered yet, or maybe a variant of it with some callback link. But this requires
            // a bigger setup that is not implemented so far for this special case. That could be
            // improved in the future.
            await fetchMfaToken({});
            if (!mfaModToken) {
                console.error('Could not fetch MFA mod token for federated user');
            }
        } else {
            showModal = true;
        }
    }

    async function fetchOtps() {
        let res = await fetchGet<OtpResponse[]>(`/auth/v1/users/${user.id}/otp`);
        loadingOtps = false;
        otpError = res.body ? undefined : res.error?.message || 'Error';
        if (res.body) otps = res.body;
    }

    async function handleCreateOtp() {
        // 'email' is the only implemented type for now
        otpKind = 'email';

        resetMsgErr();
        if (!userId || !otpKind) {
            console.error('missing create OTP data', isInputError, userId, otpKind);
            return;
        }

        let tokenId = mfaModToken?.id;
        if (!tokenId) {
            requestedBeforeTokenFetch = 'otp';
            if (isUserFederatedOnly) {
                await fetchMfaToken({});
                if (!mfaModToken) {
                    console.error('Error fetching MFA mod token for federated-only user');
                }
            } else {
                showModal = true;
            }
            return;
        }

        let res = await postOtp(userId, otpKind, otpName, tokenId);
        if (res.data) {
            const otp = res.data;
            otpId = otp.id;
            otps = [...otps.filter(existing => existing.id !== otp.id), otp];
            showOtpInput = true;
            otpKind = undefined;
            otpName = '';
        } else {
            err = true;
            msg = `${t.mfa.errorReg} - ${res.error}`;
        }
    }

    async function handleActivateOtp(form?: HTMLFormElement, params?: URLSearchParams) {
        resetMsgErr();

        if (isInputError || !userId || !showOtpInput || !otpId) {
            return;
        }

        let tokenId = mfaModToken?.id;
        if (!tokenId) {
            showModal = true;
            showOtpInput = false;
            return;
        }

        let res = await putOtp(userId, otpId, otpValueNew.replaceAll(' ', '') || '', tokenId);
        if (res.error) {
            err = true;
            msg = res.error || 'Error';
        } else {
            showOtpInput = false;
            otpValueNew = '';

            for (let otp of otps) {
                if (otp.id == otpId) {
                    otp.is_active = true;
                    return;
                }
            }
        }
    }

    async function handleDeleteOtp(otpId: string) {
        resetMsgErr();

        if (isInputError || !userId || !otpId) {
            return;
        }

        let tokenId = mfaModToken?.id;
        if (!tokenId) {
            showModal = true;
            return;
        }

        let res = await deleteOtp(userId, otpId, tokenId);
        if (res.error) {
            err = true;
            msg = res.error || 'Error';
        } else {
            await fetchOtps();
        }
    }

    async function onMfaTokenSubmit(_form: HTMLFormElement, params: URLSearchParams) {
        pwdErr = '';
        isLoading = true;

        let payload: UserMfaTokenRequest = {
            password: params.get('password') || '',
        };
        await fetchMfaToken(payload);
        isLoading = false;
    }

    async function fetchMfaToken(payload: UserMfaTokenRequest) {
        let res = await fetchPost<MfaModTokenResponse>(
            `/auth/v1/users/${user.id}/mfa_token`,
            payload,
        );
        if (res.body) {
            mfaModToken = res.body;
            closeModal?.();

            if (interval) {
                clearInterval(interval);
            }

            calcModSecs();
            interval = window.setInterval(() => {
                calcModSecs();
            }, 1000);

            let action = requestedBeforeTokenFetch;
            requestedBeforeTokenFetch = undefined;

            if (action === 'passkey') {
                showPasskeyInput = true;
            } else if (action === 'otp') {
                await handleCreateOtp();
            }
        } else {
            pwdErr = t.mfa.passwordInvalid;
        }
    }

    async function mfaTokenRefresh() {
        requestedBeforeTokenFetch = undefined;
        mfaModToken = undefined;

        if (isUserFederatedOnly) {
            await fetchMfaToken({});
            if (!mfaModToken) {
                console.error('Could not fetch MFA mod token for federated user');
            }
            return;
        }

        showModal = true;
    }

    async function onMfaTokenWebauthnSubmit() {
        closeModal?.();
        mfaPurpose = 'MfaModToken';
        mfaKind = 'webauthn';
    }

    async function onMfaTokenOtpSubmit() {
        closeModal?.();
        mfaPurpose = 'MfaModToken';
        mfaKind = 'otp';
    }

    function onMfaError(error: string) {
        console.log('mfa err', error);
        err = true;
        msg = error;
        mfaPurpose = undefined;
        mfaKind = undefined;
        requestedBeforeTokenFetch = undefined;
        setTimeout(() => {
            err = false;
            msg = '';
        }, 5000);
    }

    function onMfaSuccess(data?: WebauthnAdditionalData | OtpAdditionalData) {
        if (mfaPurpose === 'MfaModToken' && mfaKind === 'webauthn') {
            if (!data) {
                console.error('did not receive WebauthnData after SvcReq');
                return;
            }
            let svc = data as WebauthnServiceReq;
            let payload: UserMfaTokenRequest = {
                mfa_code: svc.code,
            };
            fetchMfaToken(payload);
        } else if (mfaPurpose === 'MfaModToken' && mfaKind === 'otp') {
            if (!data) {
                console.error('did not receive OtpData after SvcReq');
                return;
            }
            let svc = data as OtpServiceReq;
            let payload: UserMfaTokenRequest = {
                mfa_code: svc.code,
            };
            fetchMfaToken(payload);
        } else {
            testSuccess = true;
            msg = t.mfa.testSuccess;
            setTimeout(() => {
                testSuccess = false;
                msg = '';
            }, 3000);
        }

        mfaKind = undefined;
        mfaPurpose = undefined;
    }

    // A closed re-auth modal without a token means the user aborted, and the action it was about
    // to perform must not be resumed by any later fetch. A close that only hands over to the
    // selected method keeps it.
    function onMfaModalClose() {
        if (!mfaModToken && !mfaPurpose) {
            requestedBeforeTokenFetch = undefined;
        }
    }
</script>

<Template id={TPL_IS_OTP_ENABLED} bind:value={isOtpEnabled} />
<Template id={TPL_OTP_LENGTH} bind:value={otpSize} />

<div class="methods">
    {#if mfaModSecs && mfaModSecs > 0}
        <div class="modToken">
            <div>
                {t.account.canModifyFor}
                <strong>{mfaModSecs} {t.common.seconds}</strong>
            </div>
            <Button ariaLabel={t.common.refresh} invisible onclick={mfaTokenRefresh}>
                <IconArrowPathSquare width="1rem" />
            </Button>
        </div>
    {/if}

    <MfaMethod
        title={t.mfa.webauthn.title}
        loading={loadingPasskeys}
        error={passkeyError}
        empty={passkeys.length === 0 && !showPasskeyInput
            ? (t.mfa.webauthn.empty ?? I18nEn.mfa.webauthn.empty)
            : undefined}
    >
        {#snippet actions()}
            {#if isWebauthnSupported && !showPasskeyInput}
                <div class="actions">
                    {#if passkeys.length > 0}
                        <Button
                            level={2}
                            onclick={() => {
                                mfaPurpose = 'Test';
                                mfaKind = 'webauthn';
                            }}
                        >
                            {t.mfa.testPasskey ?? t.mfa.test}
                        </Button>
                    {/if}
                    <Button
                        level={passkeys.length === 0 ? 1 : 2}
                        onclick={() => onRegisterPasskeyClick('passkey')}
                    >
                        {t.mfa.webauthn.registerNew}
                    </Button>
                </div>
            {/if}
        {/snippet}

        {#if !isWebauthnSupported}
            <p>{t.mfa.webauthn.unsupportedText}</p>
        {:else if mfaPurpose && mfaKind === 'webauthn'}
            <WebauthnRequest purpose={mfaPurpose} onSuccess={onMfaSuccess} onError={onMfaError} />
        {/if}

        {#if showPasskeyInput}
            <div class="setup">
                <div class="guidance">
                    <p>{t.mfa.webauthn.p1}</p>
                    <p>{t.mfa.webauthn.p2}</p>
                    <p>
                        {t.mfa.webauthn.p3}
                        <a href="https://sebadob.github.io/rauthy/config/passkeys.html"
                            >{t.mfa.webauthn.docLinkText}</a
                        >.
                    </p>
                </div>
                <Form action="" onSubmit={handleRegisterPasskey}>
                    <PasskeyTypeSelect bind:residentKey />
                    <Input
                        bind:ref={refInput}
                        bind:value={passkeyName}
                        autocomplete="off"
                        label={t.mfa.passkeyName}
                        placeholder={t.mfa.passkeyName}
                        maxLength={32}
                        pattern={PATTERN_USER_NAME}
                        bind:isError={isInputError}
                        required
                    />
                    <div class="actions">
                        <Button type="submit">{t.mfa.register}</Button>
                        <Button level={2} onclick={() => (showPasskeyInput = false)}>
                            {t.common.cancel}
                        </Button>
                    </div>
                </Form>
            </div>
        {/if}

        {#each passkeys as passkey (passkey.name)}
            <UserPasskey {passkey} {showDelete} onDelete={handleDeletePasskey} />
        {/each}
    </MfaMethod>

    {#if isOtpEnabled}
        <MfaMethod
            title={t.mfa.otp.title}
            loading={loadingOtps}
            error={otpError}
            empty={!hasOtp && !(mfaModToken && showOtpInput)
                ? (t.mfa.otp.empty ?? I18nEn.mfa.otp.empty)
                : undefined}
        >
            {#snippet actions()}
                {#if !hasOtp && !(mfaModToken && showOtpInput)}
                    <Button level={2} onclick={handleCreateOtp}>{t.mfa.otp.registerNew}</Button>
                {/if}
            {/snippet}

            {#if mfaPurpose && mfaKind === 'otp'}
                <OtpRequest
                    activeOtps={otps}
                    purpose={mfaPurpose}
                    onSuccess={onMfaSuccess}
                    onError={onMfaError}
                />
            {/if}

            {#if mfaModToken && showOtpInput}
                <div class="setup">
                    <p>{t.mfa.otp.activationCode}</p>
                    <Form action="" onSubmit={handleActivateOtp}>
                        <InputOtp
                            bind:ref={refInput}
                            bind:isError={isInputError}
                            bind:value={otpValueNew}
                        />
                        <div class="actions">
                            <Button type="submit">{t.mfa.register}</Button>
                            <Button level={2} onclick={() => (showOtpInput = false)}>
                                {t.common.cancel}
                            </Button>
                        </div>
                    </Form>
                </div>
            {/if}

            {#if hasOtp}
                <h4>{t.mfa.registeredOtps}</h4>
            {/if}
            {#each otps as otp (otp.id)}
                <UserOtp {otp} showInactive={false} onDelete={handleDeleteOtp} />
            {/each}
        </MfaMethod>
    {/if}

    {#if msg}
        {#if err}
            <p class="err" role="alert">{msg}</p>
        {:else}
            <p class="success" role="status">
                {#if testSuccess}<IconCheck width="1rem" />{/if}
                {msg}
            </p>
        {/if}
    {/if}
</div>

<Modal bind:showModal bind:closeModal onClose={onMfaModalClose}>
    {#if user.webauthn_user_id}
        <p style:max-width="20rem">
            {t.mfa.reAuthenticatePasskey}
        </p>
        <ul>
            {#each passkeys as pk}
                <li>{pk.name}</li>
            {/each}
        </ul>

        <div style:margin-top="1rem">
            <Button bind:ref={refPkAuthBtn} onclick={onMfaTokenWebauthnSubmit}>
                {t.common.authenticate}
            </Button>
        </div>
    {:else if isOtpEnabled && hasOtp}
        <p style:max-width="20rem">
            {t.mfa.reAuthenticateOtp}
        </p>

        <ul>
            {#each otps as otp}
                <li>{otp.kind}</li>
            {/each}
        </ul>

        <div style:margin-top="1rem">
            <Button bind:ref={refPkAuthBtn} onclick={onMfaTokenOtpSubmit}>
                {t.common.authenticate}
            </Button>
        </div>
    {:else}
        <p style:max-width="20rem">
            {t.mfa.reAuthenticatePwd}
        </p>

        <Form action="" onSubmit={onMfaTokenSubmit}>
            <InputPassword
                bind:ref={refInput}
                name="password"
                autocomplete="current-password"
                label={t.account.passwordCurr}
                placeholder={t.account.passwordCurr}
                required
            />
            <Button type="submit" {isLoading}>{t.common.authenticate}</Button>
            {#if pwdErr}
                <div class="pwdInvalid">
                    {pwdErr}
                </div>
            {/if}
        </Form>
    {/if}
</Modal>

<style>
    .methods,
    .setup,
    .guidance {
        display: grid;
        gap: var(--ui-space);
    }

    .setup {
        padding: 0.75rem;
        border: 1px solid var(--ui-outline);
        border-radius: var(--border-radius);
        background: var(--ui-hover);
    }

    .guidance {
        gap: 0.5rem;
        max-width: 65ch;
        color: var(--ui-muted);
    }

    .actions,
    .modToken,
    .success {
        display: flex;
        align-items: center;
        flex-wrap: wrap;
        gap: 0.5rem;
    }

    .modToken {
        padding: 0.375rem 0.75rem;
        border-radius: var(--border-radius);
        background: var(--ui-tonal);
        font-size: var(--ui-control-font-size);
    }

    .success {
        color: var(--ui-primary);
    }

    .pwdInvalid {
        color: hsl(var(--error));
        margin-top: 0.5rem;
    }

    h4 {
        font-size: var(--ui-control-font-size);
    }
</style>
