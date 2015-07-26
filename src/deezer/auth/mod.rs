pub fn auth_application() {
    println!("auth")
}

/// Create uri for user authentication in form:
///
/// https://connect.deezer.com/oauth/auth.php?app_id=YOUR_APP_ID&redirect_uri=YOUR_REDIRECT_URI&perms=basic_access,email
///
pub fn get_authorize_link(app_id: &str, redirect_uri: &str) -> String {
    let base_uri = "https://connect.deezer.com/oauth/auth.php?app_id=".to_string();
    let complete_uri = base_uri + app_id + "&redirect_uri=" + redirect_uri + "&perms=basic_access";
    complete_uri
}
