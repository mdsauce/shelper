use super::auth;

pub struct User {
    pub creds: auth::Credentials,
    pub region: Region,
}

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
