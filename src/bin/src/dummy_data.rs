use rauthy_common::utils::get_rand;
use rauthy_error::ErrorResponse;
use rauthy_models::{entity::users::User, language::Language};
use std::time::Duration;
use tracing::info;

// TODO keep around until internal benchmarking is finished
#[allow(dead_code)]
pub async fn insert_dummy_data(amount: u32) -> Result<(), ErrorResponse> {
    tokio::time::sleep(Duration::from_secs(1)).await;
    info!(
        r#"

Preparing to insert dummy data into the Database.
CAUTION: Do not do this on a production database!
Will go on in 10 seconds...
        "#
    );
    tokio::time::sleep(Duration::from_secs(10)).await;
    info!("Generating and inserting dummy data now");

    // for now, this will simply generate users equal to the given amount with some random values
    // and insert them into the database

    // TODO in the future, this can be extended with a handful of groups and roles the users can
    // be randomly assigned to, as well as maybe scope clients and scopes to have more data to
    // play with

    let rnd = get_rand(4);

    for i in 0..amount {
        let user = User {
            email: format!("dummy_{rnd}_{i}@rauthy.local"),
            email_verified: false,
            given_name: format!("given {i}"),
            family_name: Some(format!("family {i}")),
            language: Language::En,
            groups: None,
            roles: String::default(),
            user_expires: None,
            ..Default::default()
        };
        User::insert(user).await?;
    }

    Ok(())
}
