use std::collections::HashMap;
use std::env;

pub fn env_credentials() -> HashMap<String, String> {
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
    let mut creds = HashMap::new();
    creds.insert(USERNAME.to_string(), sauce_username);
    creds.insert(ACCESS_KEY.to_string(), sauce_access_key);
    return creds;
}

#[cfg(test)]
mod tests {
    #[test]
    fn find_env_variables() {
        use std::env;
        env::set_var("SAUCE_USERNAME", "my.name");
        env::set_var("SAUCE_ACCESS_KEY", "my.access.key");

        assert_eq!(crate::env_credentials().len(), 2);
        assert!(crate::env_credentials().contains_key("SAUCE_USERNAME"));
        assert_eq!(crate::env_credentials()["SAUCE_USERNAME"], "my.name");
    }
}
