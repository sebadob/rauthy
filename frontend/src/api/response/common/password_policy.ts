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