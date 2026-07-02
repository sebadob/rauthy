import type { MfaPurpose } from '$api/types/mfa';
import type { OtpResponse } from '$api/types/otp';

export type OtpAdditionalData = undefined | OtpLoginFinishResponse | OtpServiceReq;

export interface OtpAuthStartRequest {
    otp_id: string;
    purpose: MfaPurpose;
}

export interface OtpAuthStartResult {
    error?: string;
    data?: OtpAuthStartResponse;
}

export interface OtpAuthStartResponse {
    code: string;
}

export interface OtpAuthFinishRequest {
    code: string;
    otp_code: string;
}

export interface OtpAuthFinishResult {
    error?: string;
    data?: OtpAdditionalData;
}

export interface OtpLoginFinishResponse {
    loc: string;
}

export interface OtpServiceReq {
    code: string;
}

export type OtpKind = 'email' | 'phone' | 'time';

export interface OtpCreateRequest {
    otp_name?: string;
    otp_kind: OtpKind;
    mfa_mod_token_id: string;
}

export interface OtpCreateResponse {
    error?: string;
    data?: OtpResponse;
}

export interface OtpActivateRequest {
    otp_id: string;
    otp_code: string;
    mfa_mod_token_id: string;
}

export interface OtpActivateResponse {
    error?: string;
    data?: null;
}

export interface OtpDeleteRequest {
    otp_id: string;
    mfa_mod_token_id?: string;
}

export interface OtpDeleteResponse {
    error?: string;
    data?: null;
}
