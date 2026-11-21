use crate::language::Language;
use crate::rauthy_config::{RauthyConfig, VarsEmailTzFmt};
use chrono::DateTime;
use chrono_tz::Tz;
use std::str::FromStr;

pub mod custom;
pub mod email_change_confirm;
pub mod email_change_info;
pub mod email_registered_already;
pub mod i18n;
pub mod login_location;
pub mod mailer;
pub mod mailer_callback;
mod mailer_microsoft_graph;
pub mod notification;
pub mod otp;
pub mod password_reset;
pub mod password_reset_info;
pub mod smtp_oauth_token;

/// Prettifies unix timestamps for E-Mails in a better readable format for end users
fn email_ts_prettify(ts: i64, lang: &Language, user_tz: Option<&str>) -> String {
    let config = &RauthyConfig::get().vars.email.tz_fmt;
    email_ts_prettify_with(config, ts, lang, user_tz)
}

#[inline(always)]
fn email_ts_prettify_with(
    config: &VarsEmailTzFmt,
    ts: i64,
    lang: &Language,
    user_tz: Option<&str>,
) -> String {
    let dt = DateTime::from_timestamp(ts, 0).unwrap_or_default();
    let tz = user_tz
        .and_then(|tz| Tz::from_str(tz).ok())
        .unwrap_or_else(|| Tz::from_str(&config.tz_fallback).unwrap());

    let fmt_str = match lang {
        Language::Cs => config.cs.as_ref(),
        Language::De => config.de.as_ref(),
        Language::En => config.en.as_ref(),
        Language::Es => config.es.as_ref(),
        Language::Fr => config.fr.as_ref(),
        Language::It => config.it.as_ref(),
        Language::Ja => config.ja.as_ref(),
        Language::Ko => config.ko.as_ref(),
        Language::Nb => config.no.as_ref(),
        Language::Nl => config.nl.as_ref(),
        Language::Pl => config.pl.as_ref(),
        Language::Pt => config.pt.as_ref(),
        Language::Ru => config.ru.as_ref(),
        Language::Uk => config.uk.as_ref(),
        Language::ZhHans => config.zhhans.as_ref(),
    };

    dt.with_timezone(&tz).format(fmt_str).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_email_ts_prettify() {
        let config = crate::rauthy_config::Vars::default().email.tz_fmt;

        let now = Utc
            .with_ymd_and_hms(2000, 10, 20, 13, 23, 33)
            .unwrap()
            .timestamp();

        let p = email_ts_prettify_with(&config, now, &Language::En, None);
        assert_eq!(&p, "10/20/2000 13:23:33 (UTC)");
        let p = email_ts_prettify_with(&config, now, &Language::It, None);
        assert_eq!(&p, "20/10/2000 13:23:33 (UTC)");
        let p = email_ts_prettify_with(&config, now, &Language::It, Some("Europe/Rome"));
        assert_eq!(&p, "20/10/2000 15:23:33 (CEST)");
        let p = email_ts_prettify_with(&config, now, &Language::En, Some("Europe/Berlin"));
        assert_eq!(&p, "10/20/2000 15:23:33 (CEST)");

        let p = email_ts_prettify_with(&config, now, &Language::De, None);
        assert_eq!(&p, "20.10.2000 13:23:33 (UTC)");
        let p = email_ts_prettify_with(&config, now, &Language::De, Some("Europe/Berlin"));
        assert_eq!(&p, "20.10.2000 15:23:33 (CEST)");

        let p = email_ts_prettify_with(&config, now, &Language::Cs, Some("Europe/Prague"));
        assert_eq!(&p, "20. 10. 2000 15:23:33 (CEST)");
    }
}
