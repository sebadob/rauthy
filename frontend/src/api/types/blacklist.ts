export interface IpBlacklistRequest {
	/// Validation: PATTERN_IPV4
	ip: string;
	/// Unix timestamp in seconds
	exp: number;
}

export interface BlacklistedIp {
	ip: string;
	/// Unix timestamp in seconds
	exp: number;
}

export interface BlacklistResponse {
	ips: BlacklistedIp[];
}
