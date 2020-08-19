use super::auth;
use std::str::FromStr;

/// Represents a `User` at saucelabs.com. A user object is made up 
/// of the username, accesskey, and Region(US vs EU). This can be a customer user
/// or a Super Admin sauce employee.
#[derive(Debug)]
pub struct User {
    pub creds: auth::Credentials,
    pub region: Region,
}

/// Which saucelabs.com datacenter that the user belongs to.
/// Defaults to US.
#[derive(Debug, PartialEq)]
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
                region: Region::default(),
            },
        }
    }
}

impl Default for Region {
    fn default() -> Self {
        Region::US
    }
}

impl FromStr for Region {
    type Err = &'static str;

    fn from_str(r: &str) -> Result<Self, Self::Err> {
        match r {
            "US" | "us" => Ok(Region::US),
            "EU" | "eu" => Ok(Region::EU),
            _ => Err("Region does not exist"),
        }
    }
}
