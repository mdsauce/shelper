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
    pub fn new(creds: auth::Credentials, region: Option<Region>) -> User {
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
