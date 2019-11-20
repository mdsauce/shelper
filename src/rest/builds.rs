use super::auth;
use super::sauce_errors;
use super::users;
extern crate reqwest;
extern crate serde_json;
use std::error::Error;

pub fn jobs(build_id: String, user: users::User) -> Result<serde_json::Value, Box<dyn Error>> {
    let build_api = format!("https://app.saucelabs.com/rest/v1/builds/{}/jobs", build_id);
    let resp: serde_json::Value = reqwest::Client::new()
        .get(&build_api)
        .basic_auth(&user.creds.username, Some(&user.creds.access_key))
        .send()?
        .json()?;
    if resp["jobs"].is_array() {
        return Ok(resp);
    } else {
        let masked_key = auth::mask_key(user.creds.access_key);
        return Err(Box::new(sauce_errors::build::NoJobs::new(
            &user.creds.username,
            &masked_key,
            &build_api,
            resp,
        )));
    };
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic]
    fn all_jobs_bad_input() {
        let fake_user = super::users::User::new(
            "bad.user12b1581b".to_string(),
            "1285-fake-b128b519".to_string(),
            None,
        );
        match super::jobs("91ee45d589ce4177981bf22f911f22c5".to_string(), fake_user) {
            Ok(resp) => assert_eq!(resp["jobs"].as_array().unwrap().len(), 32),
            Err(e) => assert_eq!(e.to_string(), ""),
        }
    }

    #[test]
    fn all_jobs_present() {
        let real_user = super::users::User::new("".to_string(), "".to_string(), None);
        match super::jobs("6fe18c6e08a14d1782a9b9eb322269c1".to_string(), real_user) {
            Ok(resp) => assert_eq!(resp["jobs"].as_array().unwrap().len(), 30),
            Err(e) => assert_eq!(e.to_string(), ""),
        }
    }
}
