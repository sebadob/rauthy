export type UserValueConfigValue = 'required' | 'optional' | 'hidden';

export interface UserValuesConfig {
    given_name: UserValueConfigValue;
    family_name: UserValueConfigValue;
    birthdate: UserValueConfigValue;
    street: UserValueConfigValue;
    zip: UserValueConfigValue;
    city: UserValueConfigValue;
    country: UserValueConfigValue;
    phone: UserValueConfigValue;
    tz: UserValueConfigValue;
    preferred_username: UserValuesPreferredUsername;
}

export interface UserValuesPreferredUsername {
    preferred_username: UserValueConfigValue;
    immutable: boolean;
    blacklist: string[];
    pattern_html: string;
    email_fallback: boolean;
}
