export type MfaPurpose =
    | { Login: string }
    | 'MfaModToken'
    | 'PasswordNew'
    | 'PasswordReset'
    | 'Test';
