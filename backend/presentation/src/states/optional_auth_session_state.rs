use crate::states::AuthSession;

/// Represents an optional authentication session, used when access to a route
/// does not strictly require a logged-in user.
///
/// For example, when retrieving a tier list by ID:
/// - Public tier lists can be accessed by anyone, even without authentication.
/// - Private tier lists require authentication, as only their author can access them.
#[derive(Clone, Debug)]
pub struct OptionalAuthSession {
    pub auth_state: Option<AuthSession>
}

impl OptionalAuthSession {
    pub fn none() -> Self {
        Self {
            auth_state: None
        }
    }

    pub fn some(auth_session: AuthSession) -> Self {
        Self {
            auth_state: Some(auth_session)
        }
    }
}