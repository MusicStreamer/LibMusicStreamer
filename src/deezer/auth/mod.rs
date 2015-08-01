use super::AuthMethods;
use super::AuthorizationStatus;

/// Store information about authorization progress and token
pub struct AuthObjectDeezer {
    status: AuthorizationStatus,
    token: String,
}

impl AuthObjectDeezer {
    fn new() -> AuthObjectDeezer {
        AuthObjectDeezer {
            status: AuthorizationStatus::Nothing,
            token: "".to_string(),
        }
    }
}

impl AuthMethods for AuthObjectDeezer {
    
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
    /// use music_streamer::deezer::auth::AuthObjectDeezer;
    /// use music_streamer::deezer::auth::AuthMethods;
    ///
    /// let auth = AuthObjectDeezer::new();
    ///
    /// let link = auth.get_authorize_link("111", "http://example.com");
    /// assert_eq!(link, "https://connect.deezer.com/oauth/auth.php?app_id=111\
    ///                   &redirect_uri=http://example.com&perms=basic_access");
    /// ```
    fn get_authorize_link(&self, app_id: &str, redirect_uri: &str) -> String {
        let base_uri = "https://connect.deezer.com/oauth/auth.php?app_id=".to_string();
        let complete_uri = base_uri + app_id + "&redirect_uri=" + redirect_uri + "&perms=basic_access";
        complete_uri
    }

    /// Save token to authentication object
    /// Incomming token will be erased for security reasons
    ///
    /// # Examples
    ///
    /// ```
    /// use music_streamer::deezer::auth::AuthObjectDeezer;
    /// 
    /// let token = "token";
    /// let auth = AuthObjectDeezer();
    /// assert_eq!(auth.save_token(token), true);
    /// 
    /// let load_token = auth.load_token();
    /// assert_eq!(load_token, token);
    /// ```
    ///
    fn save_token(&mut self, mut token: String) -> bool {
        self.token = token.to_string();
        // remove token 
        // this will be the only place where token will be stored
        // for security reasons
        token.clear();
        true
    }
    
    /// Get active user token
    /// 
    /// DO NOT STORE THE TOKEN ELSEWHERE
    fn get_token(&self) -> String {
        self.token.to_string()
    }
}
