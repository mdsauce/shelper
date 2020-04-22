use super::auth;

/// Represents a `User` at saucelabs.com. Contains the Credentials username:access_key
/// Contains the Region they're in within the context of Sauce data centers
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
    pub fn new(
        username: Option<String>,
        access_key: Option<String>,
        region: Option<Region>,
    ) -> User {
        let creds = auth::set_credentials(username, access_key);
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
