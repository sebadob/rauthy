export type DeviceAcceptedRequest = 'accept' | 'decline' | 'pending';

export interface DeviceVerifyRequest {
    /// Validation: PATTERN_ALNUM
    user_code: string,
    /// Validation: PATTERN_URI
    pow: string,
    /// pending - information about the request will be returned.
    /// accept - the device will get a Token Set
    /// decline - the code request will be deleted and rejected
    device_accepted: DeviceAcceptedRequest,
}

export interface DeviceVerifyResponse {
    scopes?: string,
}