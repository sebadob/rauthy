export interface EncKeyMigrateRequest {
	/// Validation: PATTERN_ALNUM
	key_id: string;
}

export interface EncKeysResponse {
	active: string;
	keys: string[];
}
