use std::collections::HashMap;
use std::env;
extern crate reqwest;
extern crate serde_json;


fn env_credentials() -> HashMap<String, String> {
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

fn all_jobs(build: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let creds = env_credentials();
    let build_api = format!("https://app.saucelabs.com/rest/v1/builds/{}/jobs", build);
    let resp: serde_json::Value  = reqwest::Client::new()
    .get(&build_api)
    .basic_auth(&creds["SAUCE_USERNAME"], Some(&creds["SAUCE_ACCESS_KEY"]))
    .send()?
    .json()?;

    println!("This is the all_jobs(buildId) response headers: {:#?}", resp);
    Ok(resp)
}

fn setup(user: &str, key: &str) {
    env::set_var("SAUCE_USERNAME", user);
    env::set_var("SAUCE_ACCESS_KEY", key);
}

#[cfg(test)]
mod tests {
    #[test]
    fn find_env_variables() {
        crate::setup("my.name", "random123125152");
        assert_eq!(crate::env_credentials().len(), 2);
        assert!(crate::env_credentials().contains_key("SAUCE_USERNAME"));
        assert_eq!(crate::env_credentials()["SAUCE_USERNAME"], "my.name");
    }

    #[test]
    fn all_jobs_present() {
        match crate::all_jobs("91ee45d589ce4177981bf22f911f22c5") {
            Ok(resp) => assert_eq!(resp["jobs"].as_array().unwrap().len(), 32),
            Err(e) => assert_eq!(e.to_string(), ""),
        }
    }
}
