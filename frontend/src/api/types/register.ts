export interface NewUserRegistrationRequest {
	/// Validation: `email`
	email: string;
	/// Validation: `[user_values.preferred_username] -> regex_rust`
	preferred_username?: string;
	/// Validation: PATTERN_USER_NAME
	given_name?: string;
	/// Validation: PATTERN_USER_NAME
	family_name?: string;
	// Validation: PATTERN_DATE_STR
	birthdate?: string | null;
	// Validation: PATTERN_PHONE / 32
	phone?: string | null;
	// Validation: PATTERN_STREET / 48
	street?: string | null;
	// Validation: PATTERN_ALNUM / 24
	zip?: string | null;
	// Validation: PATTERN_CITY / 48
	city?: string | null;
	// Validation: PATTERN_CITY / 48
	country?: string | null;
	/// Validation: Valid Timezone in the format of `Europe/Berlin`
	tz?: string | null;
	/// Validation: PATTERN_URI
	pow: string;
	/// Validation: PATTERN_URI
	redirect_uri?: string;
}
