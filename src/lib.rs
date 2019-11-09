use std::env;
extern crate reqwest;
extern crate serde_json;

pub struct Credentials {
    username: String,
    access_key: String,
}

fn set_credentials(user: Option<String>, key: Option<String>) -> Credentials {
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

pub fn all_jobs(
    build: &str,
    user: String,
    key: String,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let creds = set_credentials(Some(user), Some(key));
    let build_api = format!("https://app.saucelabs.com/rest/v1/builds/{}/jobs", build);
    let resp: serde_json::Value = reqwest::Client::new()
        .get(&build_api)
        .basic_auth(&creds.username, Some(&creds.access_key))
        .send()?
        .json()?;
    if resp["jobs"].is_array() {
        return Ok(resp);
    } else {
        println!("WE ARE HERE");
        let mut masked_key = String::new();
        let mut i = 0;
        for c in creds.access_key.chars() {
            if i == 5 {
                break;
            }
            masked_key.push(c);
            i += 1;
            println!("{}", c);
            println!(">>>>>>>>>>>>>>> {}", masked_key);
        }
        panic!(
            "Something went wrong with the request using user {}:******{} {}.  Response: {}",
            creds.username, masked_key, build_api, resp
        )
    };
}

#[cfg(test)]
fn setup(user: &str, key: &str) {
    env::set_var("SAUCE_USERNAME", user);
    env::set_var("SAUCE_ACCESS_KEY", key);
}

#[cfg(test)]
mod tests {
    #[test]
    fn find_env_variables() {
        crate::setup("my.name", "random123125152");
        assert_eq!(crate::env_credentials().username, "my.name");
    }

    #[test]
    fn all_jobs_present() {
        match crate::all_jobs(
            "91ee45d589ce4177981bf22f911f22c5",
            "wrong-user1234ab12vasf".to_string(),
            "1285b128b519".to_string(),
        ) {
            Ok(resp) => assert_eq!(resp["jobs"].as_array().unwrap().len(), 32),
            Err(e) => assert_eq!(e.to_string(), ""),
        }
    }
}
