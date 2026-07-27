use crate::scim::groups::GROUPS;
use crate::scim::users::USERS;
use crate::templates::HTML_SCIM_DATA;
use axum::http::header::CONTENT_TYPE;
use axum::response::Response;
use std::fmt::Write;

pub mod groups;
pub mod users;

pub async fn get_data() -> Response<String> {
    let mut user_data = String::with_capacity(64);
    for user in USERS.read().await.iter() {
        writeln!(user_data, "<p>{user:?}</p>").unwrap();
    }

    let mut group_data = String::with_capacity(64);
    for group in GROUPS.read().await.iter() {
        writeln!(group_data, "<p>{group:?}</p>").unwrap();
    }

    Response::builder()
        .status(200)
        .header(CONTENT_TYPE, "text/html")
        .body(
            HTML_SCIM_DATA
                .replace("{{ USERS }}", &user_data)
                .replace("{{ GROUPS }}", &group_data),
        )
        .unwrap()
}
