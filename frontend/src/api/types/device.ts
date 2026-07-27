export type DeviceAcceptedRequest = 'accept' | 'decline' | 'pending';

export interface DeviceRequest {
    /// Validation: PATTERN_URI
    device_id: string;
    /// Validation: PATTERN_CLIENT_NAME
    name?: String;
}

export interface DeviceVerifyRequest {
    /// Validation: PATTERN_ALNUM
    user_code: string;
    /// Validation: PATTERN_URI
    pow: string;
    /// pending - information about the request will be returned.
    /// accept - the device will get a Token Set
    /// decline - the code request will be deleted and rejected
    device_accepted: DeviceAcceptedRequest;
}

export interface DeviceVerifyResponse {
    scopes?: string;
}

export interface DeviceResponse {
    id: string;
    client_id: string;
    user_id?: string;
    created: number;
    access_exp: number;
    refresh_exp?: number;
    peer_ip: string;
    name: string;
}
