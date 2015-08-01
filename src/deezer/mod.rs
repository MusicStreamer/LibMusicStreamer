pub mod auth;

pub enum AuthorizationStatus {
    Nothing,
    UserAuthentication,
    TokenAquired,
    AuthorizationCompleted,
}

pub trait AuthMethods {
    fn status(&self) -> &AuthorizationStatus;
    fn get_authorize_link(&self, app_id: &str, redirect_uri: &str) -> String;
    fn save_token(&mut self, mut token: String) -> bool;
    fn get_token(&self) -> String;
}
