use poem_openapi::{ApiResponse, OpenApi};
use poem_openapi::payload::Json;
use crate::api::ErrorResponse;
use crate::auth::{create_user, User};
use crate::State;

#[derive(ApiResponse)]
enum SignUpRespose{
    /// User created successfully
    #[oai(status = 201)]
    Ok(Json<User>),
    /// An unknown error occurred
    #[oai(status = 500)]
    Unknown(Json<ErrorResponse>),
}
pub struct AuthApi{
    state: State
}

impl AuthApi {
    pub fn new(state: State) -> Self {
        Self{state}
    }
}

#[OpenApi(prefix_path="/auth")]
impl AuthApi {
    /// Sign up a new user
    #[oai(path="/signup", method="post")]
    async fn sign_up(&self) -> SignUpRespose{
        let user = create_user(self.state.pool()).await;
        SignUpRespose::Ok(Json(user))
    }
}