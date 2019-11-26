use super::auth;

/// Represents a User within Sauce's Virtual Desktop and Device Cloud at saucelabs.com.
/// Someone with a Sauce Labs Username and API Access Key.
pub struct User {
    pub creds: auth::Credentials,
    pub region: Region,
}

/// Which saucelabs.com datacenter that the user belongs to.
/// Defaults to US.
pub enum Region {
    EU,
    US,
}

impl User {
    pub fn new(username: String, access_key: String, region: Option<Region>) -> User {
        let creds = auth::set_credentials(Some(username), Some(access_key));
        match region {
            Some(region) => User {
                creds: creds,
                region: region,
            },
            None => User {
                creds: creds,
                region: Region::US,
            },
        }
    }
}
