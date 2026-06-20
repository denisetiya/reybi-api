//! Firebase ID-token verification helper.
//!
//! Used by auth-protected routes when the client sends a Firebase ID token
//! (e.g. after Firebase Auth sign-in on mobile).  Returns the verified user
//! or `None` if the token is missing / invalid / verifier is unconfigured.

use firebase_auth::{FirebaseAuth, FirebaseUser};

/// Holds the Firebase project id needed to verify ID tokens.
#[derive(Clone)]
pub struct FirebaseVerifier {
    inner: Option<FirebaseAuth>,
}

impl FirebaseVerifier {
    pub async fn new(project_id: &str) -> Self {
        if project_id.is_empty() {
            tracing::warn!("FIREBASE_PROJECT_ID not set — Firebase ID-token verification disabled");
            Self { inner: None }
        } else {
            let fa = FirebaseAuth::new(project_id).await;
            tracing::info!(project_id, "✓ Firebase auth verifier ready");
            Self { inner: Some(fa) }
        }
    }

    pub fn verify(&self, id_token: &str) -> Option<FirebaseUser> {
        let fa = self.inner.as_ref()?;
        match fa.verify(id_token) {
            Ok(user) => Some(user),
            Err(e) => {
                tracing::debug!(error = ?e, "firebase id-token invalid");
                None
            }
        }
    }
}
