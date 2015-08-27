// This file is part of libmusic_streamer.
//
// libmusic_streamer is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// libmusic_streamer is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with libmusic_streamer.  If not, see <http://www.gnu.org/licenses/>.

//! General authorization and authentication trait
//! as first Deezer will be using this trait more will come.

mod deezer;

/// Progress status of the authorization
pub enum AuthorizationStatus {
    /// Authorization doesn't started yet
    Nothing,
    /// User authenticate self on the website
    UserAuthentication,
    /// Get token from user
    TokenAquired,
    /// Authorization is completed - can start using service
    AuthorizationCompleted,
}

/// Type of the service you want to create
pub enum ServiceType {
    DEEZER,
}

/// Create instance of AuthMethods which provides access to
/// ServiceType service.
pub fn new(service: ServiceType) -> Box<AuthMethods> {
    match service {
        ServiceType::DEEZER => {
            Box::new(deezer::AuthDeezer::new())
        }
    }
}

pub trait AuthMethods {
    /// Get status of ongoing authentication
    fn status(&self) -> &AuthorizationStatus;

    /// Return uri for user to authorize the application in his account
    fn get_authorize_link(&mut self, app_id: &str, redirect_uri: &str) -> String;

    /// Get code from response returned by browser after app
    /// authorization is completed by user
    fn parse_response_code(&self, response: &str) -> Option<String>;

    /// Authenticate application with generated code from authorization process
    fn authenticate_application(&mut self, app_id: &str, app_secret: &str, code: &str) -> Result<(), &str>;

    /// Save token to authentication object
    /// Incomming token will be moved so it won't be usable anymore
    /// for security reasons
    fn save_token(&mut self, token: String);

    /// Get active user token
    /// 
    /// DO NOT STORE THE TOKEN ELSEWHERE
    fn get_token(&self) -> String;
}
