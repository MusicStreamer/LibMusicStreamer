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

//! Specific deezer implementation of authorization and authentication trait.
//! Object AuthDeezer will implement whole process.

use super::AuthMethods;
use super::AuthorizationStatus;

use std::io::Read;
use hyper::Client;

/// Store information about authorization progress and token
pub struct AuthDeezer {
    status: AuthorizationStatus,
    token: String,
    expires: String,
}

impl AuthDeezer {
    //! Authentication object for deezer.
    //! This object will be used for user and application Authentication
    
    /// Create new Deezer authentication object
    /// token will be set to empty string
    pub fn new() -> AuthDeezer {
        AuthDeezer {
            status: AuthorizationStatus::Nothing,
            token: "".to_string(),
            expires: "".to_string(),
        }
    }

    /// Take server response and parse it to tuple (token, expires)
    /// or error is returned
    fn extract_access_token(response: String) -> Result<(String, String), &'static str> {
        let token_pattern = "access_token=";
        let expires_pattern = "&expires=";
        if let Some(begin) = response.find(&token_pattern) {
            if let Some(end) = response.rfind(&expires_pattern) {
                let token = response[(begin + token_pattern.len())..end].to_string();
                let expires = response[(end + expires_pattern.len())..].to_string();

                return Ok((token, expires))
            };
        }

        Err("Could not find access token part in response")
    }
}

impl AuthMethods for AuthDeezer {
    
    /// Get status of ongoing authentication
    fn status(&self) -> &AuthorizationStatus {
        &self.status
    }
    
    /// Create uri for user authentication in form:
    ///
    /// https://connect.deezer.com/oauth/auth.php?app_id=YOUR_APP_ID&redirect_uri=YOUR_REDIRECT_URI&perms=basic_access,email
    ///
    /// # Examples
    ///
    /// ```
    /// use music_streamer::auth::deezer::AuthDeezer;
    /// use music_streamer::auth::AuthMethods;
    ///
    /// let mut auth = AuthDeezer::new();
    ///
    /// let link = auth.get_authorize_link("111", "http://example.com");
    /// assert_eq!(link, "https://connect.deezer.com/oauth/auth.php?app_id=111\
    ///                   &redirect_uri=http://example.com&perms=basic_access");
    /// ```
    fn get_authorize_link(&mut self, app_id: &str, redirect_uri: &str) -> String {
        let base_uri = "https://connect.deezer.com/oauth/auth.php?app_id=".to_string();
        let complete_uri = base_uri + app_id + "&redirect_uri=" + redirect_uri + "&perms=basic_access";
        self.status = AuthorizationStatus::UserAuthentication;
        complete_uri
    }


    /// Get code from authorization response uri
    ///
    /// # Examples
    ///
    /// ```
    /// use music_streamer::auth::deezer::AuthDeezer;
    /// use music_streamer::auth::AuthMethods;
    ///
    /// let auth = AuthDeezer::new();
    ///
    /// let test = "http://example.com/test_path/?code=fre54bf0a48d1bf566f24c2289ce06d1";
    /// let result = auth.parse_reponse_code(test);
    ///
    /// assert_eq!(result, Some("fre54bf0a48d1bf566f24c2289ce06d1".to_string()));
    /// ```
    fn parse_response_code(&self, response: &str) -> Option<String> {
        let option = response.to_string().rfind("?code=");

        if let Some(x) = option {
            Some(response[x+6..].to_string())
        } else {
            None
        }
    }

    /// Authenticate application with code get from get_authorization_response link.
    /// This will connect to deezer and retrieve token for future communication.
    fn authenticate_application(&mut self, app_id: &str, app_secret: &str,
                               code: &str) -> Result<(), &str> {
        let base_uri = "https://connect.deezer.com/oauth/access_token.php?app_id=".to_string();
        let complete_uri = base_uri + app_id + "&secret=" + app_secret + "&code=" + code;

        // Get the token
        let client = Client::new();
        // Send get to the server
        if let Ok(mut res) = client.get(&complete_uri).send() {
            let mut body = String::new();
            let ret = res.read_to_string(&mut body);

            if ret.is_err() {
                return Err("Can't read the response. Something is really wrong.")
            }

            println!("response: {}", body);
            let (token, expires) = try!(AuthDeezer::extract_access_token(body));
            self.save_token(token);
            self.expires = expires;

            // retrieve the token
            self.status = AuthorizationStatus::AuthorizationCompleted;
        } else {
            return Err("Can't send request to the deezer server")
        }

        Ok(())
    }

    /// Save token to authentication object
    /// Incomming token will be moved so it won't be usable anymore
    /// for security reasons
    ///
    /// # Examples
    ///
    /// ```
    /// use music_streamer::auth::deezer::AuthDeezer;
    /// use music_streamer::auth::AuthMethods;
    /// 
    /// let mut token = "token".to_string();
    /// let mut auth = AuthDeezer::new();
    /// assert_eq!(auth.save_token(token), true);
    /// 
    /// let load_token = auth.get_token();
    /// assert_eq!(load_token, "token");
    /// ```
    ///
    fn save_token(&mut self, token: String) {
        self.token = token;
        self.status = AuthorizationStatus::TokenAquired;
    }
    
    /// Get active user token
    /// 
    /// DO NOT STORE THE TOKEN ELSEWHERE
    fn get_token(&self) -> String {
        self.token.to_string()
    }
}
