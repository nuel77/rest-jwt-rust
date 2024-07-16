use actix_jwt_auth_middleware::FromRequest;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, FromRequest)]
pub struct UserClaim {
    id: u32,
}
