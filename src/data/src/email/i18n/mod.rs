//! Email templates are selected separately from the UI locale. Italian uses the English
//! templates until an Italian email catalog and its template configuration are available.

pub mod change_info_new;
pub mod change_info_old;
pub mod confirm_change;
pub mod confirm_change_html;
pub mod email_registered_already;
pub mod login_location;
pub mod otp;
pub mod password_new;
pub mod reset;
pub mod reset_info;
