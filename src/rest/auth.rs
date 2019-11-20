use::std::env;

pub struct Credentials {
    pub username: String,
    pub access_key: String,
}

pub fn set_credentials(user: Option<String>, key: Option<String>) -> Credentials {
    let mut creds: Credentials = Credentials {
        username: "default".to_string(),
        access_key: "default".to_string(),
    };
    creds.username = match user {
        Some(user) => user,
        None => return env_credentials(),
    };
    creds.access_key = match key {
        Some(key) => key,
        None => return env_credentials(),
    };
    if creds.access_key == "" || creds.username == "" {
        return env_credentials();
    }
    return creds;
}

fn env_credentials() -> Credentials {
    const USERNAME: &str = "SAUCE_USERNAME";
    const ACCESS_KEY: &str = "SAUCE_ACCESS_KEY";
    let sauce_username = match env::var(USERNAME) {
        Ok(name) => name,
        Err(e) => panic!("Problem getting environment variable {}: {}", USERNAME, e),
    };
    let sauce_access_key = match env::var(ACCESS_KEY) {
        Ok(name) => name,
        Err(e) => panic!("Problem getting environment variable {}: {}", ACCESS_KEY, e),
    };
    let creds: Credentials = Credentials {
        username: sauce_username,
        access_key: sauce_access_key,
    };
    return creds;
}

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
