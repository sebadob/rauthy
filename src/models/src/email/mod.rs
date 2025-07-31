use chrono::DateTime;

pub mod email_change_confirm;
pub mod email_change_info;
pub mod login_location;
pub mod mailer;
pub mod notification;
pub mod password_reset;
pub mod password_reset_info;
pub mod smtp_oauth_token;

/// Prettifies unix timestamps for E-Mails in a better readable format for end users
#[inline]
fn email_ts_prettify(ts: i64) -> String {
    let dt = DateTime::from_timestamp(ts, 0).unwrap_or_default();
    let fmt = dt.format("%d/%m/%Y %H:%M:%S");
    format!("{fmt} UTC")
}
