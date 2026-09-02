use crate::entity::pwd_exp_mails::PasswordExpMail;
use rauthy_error::ErrorResponse;
use tracing::error;

/// Used for callbacks after an E-Mail was sent successfully.
#[derive(Debug)]
pub enum EMailCallback {
    None,
    PasswordExp { user_id: String },
}

impl EMailCallback {
    pub async fn call(self) {
        if let Err(err) = self.call_inner().await {
            error!("Error executing E-Mail callback: {err:?}");
        }
    }

    #[inline]
    async fn call_inner(self) -> Result<(), ErrorResponse> {
        match self {
            EMailCallback::None => {}
            EMailCallback::PasswordExp { user_id } => {
                PasswordExpMail::upsert_now(user_id).await?;
            }
        }

        Ok(())
    }
}
