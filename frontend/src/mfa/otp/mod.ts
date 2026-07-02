import { fetchDelete, fetchPost, fetchPut } from '$api/fetch';
import type { OtpResponse } from '$api/types/otp';
import type {
    OtpActivateRequest,
    OtpActivateResponse,
    OtpCreateRequest,
    OtpCreateResponse,
    OtpDeleteRequest,
    OtpDeleteResponse,
    OtpKind,
} from '$mfa/otp/types.ts';

export async function otpRequest(
    userId: string,
    kind: OtpKind,
    name: undefined | string,
    mfaModTokenId: string,
): Promise<OtpCreateResponse> {
    let payload: OtpCreateRequest = {
        otp_name: name,
        otp_kind: kind,
        mfa_mod_token_id: mfaModTokenId,
    };
    let res = await fetchPost<OtpResponse>(`/auth/v1/users/${userId}/otp`, payload);
    if (res.error) {
        console.error(res.error);
        return {
            error: res.error.message || 'Error requesting an otp',
        };
    }
    if (!res.body) {
        let error = 'Did not receive a valid otp body';
        console.error(error);
        return { error };
    }

    return { data: res.body };
}

export async function otpActivate(
    userId: string,
    id: string,
    code: string,
    mfaModTokenId: string,
): Promise<OtpActivateResponse> {
    let payload: OtpActivateRequest = {
        otp_id: id,
        otp_code: code,
        mfa_mod_token_id: mfaModTokenId,
    };
    let res = await fetchPut<null>(`/auth/v1/users/${userId}/otp`, payload);
    if (res.error) {
        console.error(res.error);
        return {
            error: res.error.message || 'Error activating the Otp',
        };
    }
    return {};
}

export async function otpDelete(
    userId: string,
    id: string,
    mfaModTokenId?: string,
): Promise<OtpDeleteResponse> {
    let payload: OtpDeleteRequest = {
        otp_id: id,
        mfa_mod_token_id: mfaModTokenId,
    };
    let res = await fetchDelete<null>(`/auth/v1/users/${userId}/otp`, payload);
    if (res.error) {
        console.error(res.error);
        return {
            error: res.error.message || 'Error deleting otp',
        };
    }
    return {};
}
