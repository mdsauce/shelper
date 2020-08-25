use std::env;

/// SAUCE_USERNAME:SAUCE_ACCESS_KEY are used for authentication.
/// Checks for Environment variables first then accepts user input.
#[derive(Debug)]
pub struct Credentials {
    pub username: String,
    pub access_key: String,
}

/// defaults to using SAUCE_USERNAME and SAUCE_ACCESS_KEY environment variables.
/// Can be overwritten with custom credentials.
pub fn set_credentials(username: Option<String>, access_key: Option<String>) -> Credentials {
    match (username, access_key) {
        (None, None) => return env_credentials(),
        (Some(username), Some(access_key)) => {
            return Credentials {
                username,
                access_key,
            }
        }
        (Some(user), None) => {
            return Credentials {
                username: user,
                access_key: "".to_string(),
            };
        }
        (None, Some(_)) => return env_credentials(),
    }
}

/// gets the env variables  SAUCE_USERNAME and SAUCE_ACCESS_KEY.
fn env_credentials() -> Credentials {
    const USERNAME: &str = "SAUCE_USERNAME";
    const ACCESS_KEY: &str = "SAUCE_ACCESS_KEY";
    let sauce_username = match env::var(USERNAME) {
        Ok(name) => name,
        Err(e) => panic!(
            "Problem getting your username from the environment variables {}: {}",
            USERNAME, e
        ),
    };
    let sauce_access_key = match env::var(ACCESS_KEY) {
        Ok(name) => name,
        Err(e) => panic!(
            "Problem getting the access key from your environment variables {}: {}",
            ACCESS_KEY, e
        ),
    };
    let creds: Credentials = Credentials {
        username: sauce_username,
        access_key: sauce_access_key,
    };
    return creds;
}

/// mask the API Key used by users prior to outputting it.
pub fn mask_key(api_key: String) -> String {
    let mut mask = String::new();
    let mut i = 0;
    for c in api_key.chars() {
        if i == 5 {
            break;
        }
        mask.push(c);
        i += 1;
    }
    return mask;
}

#[cfg(test)]
fn setup(user: &str, key: &str) {
    env::set_var("SAUCE_USERNAME", user);
    env::set_var("SAUCE_ACCESS_KEY", key);
}
#[cfg(test)]
mod tests {
    #[test]
    #[ignore]
    fn find_env_variables() {
        super::setup("my.name", "random123125152");
        assert_eq!(super::env_credentials().username, "my.name");
    }
}
