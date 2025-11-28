use crate::email::mailer::EMail;
use crate::entity::theme::{ThemeCss, ThemeCssFull};
use crate::entity::users::User;
use crate::rauthy_config::RauthyConfig;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use std::time::Duration;
use tracing::error;

pub static HTML_EMAIL_HEAD_LEFT: &str = r#"
<head>
    <meta charset="utf-8"/>
    <meta name="viewport" content="width=device-width, initial-scale=1"/>
    <style>
"#;

pub static HTML_EMAIL_HEAD_RIGHT: &str = r#"
        *, *::before, *::after { box-sizing: border-box; }
        * { margin: 0; padding: 0; }
        input, button, textarea, select { font: inherit; }
        ::-webkit-scrollbar { width: 7px; height: 7px; }
        ::-webkit-scrollbar-thumb { background-color: hsl(var(--bg-high)); border-radius: 30px; }
        ::-webkit-scrollbar-corner { display: none; }
        body {
            padding: 1rem;
            font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Roboto', 'Segoe UI', 'Oxygen',
            'Ubuntu', 'Cantarell', 'Fira Sans', 'Droid Sans', 'Helvetica Neue', sans-serif;
            line-height: 1.5rem;
            -webkit-font-smoothing: antialiased;
            color: hsl(var(--text));
            background-color: hsl(var(--bg));
            scroll-behavior: smooth;
            scrollbar-gutter: stable;
        }
        a, a:link, a:visited { color: hsl(var(--action)) !important; }
        a:hover, a:active { color: hsl(var(--action)) !important; text-decoration: underline; }
        blockquote {
            padding-left: .33rem;
            border-left: 3px solid hsl(var(--accent));
            border-radius: var(--border-radius);
            background-color: hsl(var(--bg-high));
        }
        h1, h2, h3, h4, h5, h6 { color: hsl(var(--text-high)); overflow-wrap: break-word; }
        h1 { font-size: 1.5rem; margin: 1rem 0; letter-spacing: -0.04em; }
        h2 { font-size: 1.35rem; margin: .75rem 0; letter-spacing: -0.03em; }
        h3 { font-size: 1.25rem; margin: .6rem 0; letter-spacing: -0.025em; }
        h4 { font-size: 1.2rem; margin: .65rem 0; letter-spacing: -0.015em; }
        h5 { font-size: 1.05rem; margin: .55rem 0; letter-spacing: -0.01em; }
        p { margin: .5rem 0;  max-width: 467pt; }
        hr { border: none; border-top: 1px solid hsla(var(--bg-high), .66); margin: .5rem 0; }
        code {
            border-radius: var(--border-radius);
            font-family: ui-monospace, 'Cascadia Code', 'Source Code Pro', Menlo, Consolas,
            'DejaVu Sans Mono', monospace;
            color: hsl(var(--text));
        }
        code, pre { background: hsl(var(--bg-high)) !important; }
        pre { padding: .5rem; border-radius: var(--border-radius); overflow: auto; }
        ol, ul { padding-left: 1rem; }
        ::selection { color: hsl(var(--bg)); background: hsl(var(--action)); }
    </style>
</head>
"#;

#[inline]
pub async fn build_custom_email_head() -> Result<String, ErrorResponse> {
    let theme = ThemeCssFull::find_theme_variables_email().await?;
    Ok(format!(
        "{HTML_EMAIL_HEAD_LEFT}\n{theme}\n{HTML_EMAIL_HEAD_RIGHT}"
    ))
}

pub async fn build_custom_html_email(body: &str) -> Result<String, ErrorResponse> {
    let head = build_custom_email_head().await?;
    Ok(format!(
        r#"<!DOCTYPE html>
<html>
{head}
<body>{body}</body>
</html>
"#
    ))
}

pub async fn send_custom(
    user: &User,
    subject: &str,
    text: Option<String>,
    html: Option<String>,
) -> Result<(), ErrorResponse> {
    let email_sub_prefix = &RauthyConfig::get().vars.email.sub_prefix;
    let req = EMail {
        recipient_name: user.email_recipient_name(),
        address: user.email.to_string(),
        subject: format!("{email_sub_prefix} - {subject}"),
        text,
        html,
    };

    RauthyConfig::get()
        .tx_email
        .send_timeout(req, Duration::from_secs(10))
        .await
        .map_err(|err| ErrorResponse::new(ErrorResponseType::Timeout, "email send timeout"))
}
