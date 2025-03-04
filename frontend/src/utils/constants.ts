export const PKCE_VERIFIER = 'pkce_verifier';
export const PKCE_VERIFIER_UPSTREAM = 'pkce_verifier_upstream';
export const CSRF_TOKEN = 'csrf_token';
export const ACCESS_TOKEN = 'access_token';
export const ID_TOKEN = 'id_token';
export const PROVIDER_TOKEN = 'provider_token';

export const AUTH_ENDPOINT = '/auth/v1/oidc/authorize';
export const CLIENT_ID = 'rauthy';
export const LOGOUT_URL = '/auth/v1/oidc/logout';
export const REDIRECT_URI = '/auth/v1/oidc/callback';
export const REDIRECT_URI_SUCCESS = '/auth/v1/admin/users';
export const REDIRECT_URI_SUCCESS_ACC = '/auth/v1/account';
export const POST_LOGOUT_REDIRECT_URI = '/auth/v1/';

export const EVENT_LEVELS = [
    'info',
    'notice',
    'warning',
    'critical'
]
export const EVENT_TYPES = [
    '-',
    'InvalidLogins',
    'IpBlacklisted',
    'IpBlacklistRemoved',
    'JwksRotated',
    'NewUserRegistered',
    'NewRauthyAdmin',
    'NewRauthyVersion',
    'PossibleBruteForce',
    'RauthyStarted',
    'RauthyHealthy',
    'RauthyUnhealthy',
    'SecretsMigrated',
    'UserEmailChange',
    'UserPasswordReset',
    'Test',
]
export const LANGUAGES = ['de', 'en', 'zh', 'ko'];
export const LANGUAGES_ADMIN = ['de', 'en'];

// All TPL_* values match a possibly existing `<template>` id
// -> src/models/src/html/templates.rs -> HtmlTemplate
export const TPL_AUTH_PROVIDERS = 'tpl_auth_providers';
export const TPL_CLIENT_NAME = 'tpl_client_name';
export const TPL_CLIENT_URL = 'tpl_client_url';
export const TPL_CLIENT_LOGO_UPDATED = 'tpl_client_logo_updated';
export const TPL_CSRF_TOKEN = 'tpl_csrf_token';
export const TPL_EMAIL_OLD = 'tpl_email_old';
export const TPL_EMAIL_NEW = 'tpl_email_new';
// export const TPL_ERROR_DETAILS = 'tpl_error_details';
export const TPL_ERROR_TEXT = 'tpl_error_text';
export const TPL_DEVICE_USER_CODE_LENGTH = 'tpl_device_user_code_length';
export const TPL_IS_REG_OPEN = 'tpl_is_reg_open';
export const TPL_LOGIN_ACTION = 'tpl_login_action';
export const TPL_PASSWORD_RESET = 'tpl_password_reset';
export const TPL_STATUS_CODE = 'tpl_status_code';
export const TPL_RESTRICTED_EMAIL_DOMAIN = 'tpl_restricted_email_domain';
