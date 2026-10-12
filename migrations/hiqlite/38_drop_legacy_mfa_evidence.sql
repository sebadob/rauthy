-- The pre-tracking `is_mfa` flag was set before the 2nd factor ceremony had finished, and also
-- after a factor enrollment. It is no reliable proof, so it must never become accepted MFA
-- evidence. Affected sessions and refresh tokens need a new MFA ceremony.
UPDATE sessions SET is_mfa = false, mfa_method = 'none'
WHERE mfa_method = 'mfa' OR (is_mfa = true AND mfa_method = 'none');

UPDATE refresh_tokens SET is_mfa = false, mfa_method = 'none'
WHERE mfa_method = 'mfa' OR (is_mfa = true AND mfa_method = 'none');
