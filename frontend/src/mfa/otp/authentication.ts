import { fetchPost } from '$api/fetch';
import type { MfaPurpose } from '$api/types/mfa';
import type {
    OtpAdditionalData,
    OtpAuthFinishRequest,
    OtpAuthFinishResult,
    OtpAuthStartRequest,
    OtpAuthStartResponse,
    OtpAuthStartResult,
} from './types';

export async function otpAuthStart(
    userId: string,
    otpId: string,
    purpose: MfaPurpose,
): Promise<OtpAuthStartResult> {
    let payloadStart: OtpAuthStartRequest = {
        otp_id: otpId,
        purpose,
    };
    let res = await fetchPost<OtpAuthStartResponse>(
        `/auth/v1/users/${userId}/otp/auth/start`,
        payloadStart,
    );
    if (res.error) {
        console.error(res.error);
        return {
            error: res.error.message || 'Error starting the Authentication',
        };
    }
    if (!res.body) {
        let error = 'Did not receive a valid otp body';
        console.error(error);
        return { error };
    }

    return { data: res.body };
}

export async function otpAuthFinish(
    userId: string,
    code: string,
    otpCode: string,
): Promise<OtpAuthFinishResult> {
    let payloadFinish: OtpAuthFinishRequest = {
        code,
        otp_code: otpCode,
    };
    console.log('payload start finish: ', payloadFinish);
    let res = await fetchPost<OtpAdditionalData>(
        `/auth/v1/users/${userId}/otp/auth/finish`,
        payloadFinish,
    );
    if (res.status === 202 || res.status === 206) {
        return {
            data: res.body,
        };
    } else if (res.status === 205) {
        return {};
    } else {
        console.error(res.error);
        return {
            error: res.error?.message || 'Authentication Error',
        };
    }
}
