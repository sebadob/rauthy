export interface PasswordPolicyRequest {
    /// Validation: `8 <= length_min <= 128`
    length_min: number,
    /// Validation: `8 <= length_max <= 128`
    length_max: number,
    /// Validation: `1 <= include_lower_case <= 32`
    include_lower_case?: number,
    /// Validation: `1 <= include_upper_case <= 32`
    include_upper_case?: number,
    /// Validation: `1 <= include_digits <= 32`
    include_digits?: number,
    /// Validation: `1 <= include_special <= 32`
    include_special?: number,
    /// Validation: `1 <= valid_days <= 3650`
    valid_days?: number,
    /// Validation: `1 <= not_recently_used <= 10`
    not_recently_used?: number,
}

export interface PasswordPolicyResponse {
    length_min: number,
    length_max: number,
    include_lower_case?: number,
    include_upper_case?: number,
    include_digits?: number,
    include_special?: number,
    valid_days?: number,
    not_recently_used?: number,
}