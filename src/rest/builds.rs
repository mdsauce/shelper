use super::sauce_errors;
use super::auth;
extern crate reqwest;
extern crate serde_json;
use std::error::Error;

pub fn all_jobs(
    build: String,
    user: String,
    key: String,
) -> Result<serde_json::Value, Box<dyn Error>> {
    let creds = auth::set_credentials(Some(user), Some(key));
    let build_api = format!("https://app.saucelabs.com/rest/v1/builds/{}/jobs", build);
    let resp: serde_json::Value = reqwest::Client::new()
        .get(&build_api)
        .basic_auth(&creds.username, Some(&creds.access_key))
        .send()?
        .json()?;
    if resp["jobs"].is_array() {
        return Ok(resp);
    } else {
        // should be separate util func for masking output
        let mut masked_key = String::new();
        let mut i = 0;
        for c in creds.access_key.chars() {
            if i == 5 {
                break;
            }
            masked_key.push(c);
            i += 1;
        }
        return Err(Box::new(sauce_errors::build::NoJobs::new(
            &creds.username,
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
        match super::all_jobs(
            "91ee45d589ce4177981bf22f911f22c5".to_string(),
            "bad.user12b1581b".to_string(),
            "1285-fake-b128b519".to_string(),
        ) {
            Ok(resp) => assert_eq!(resp["jobs"].as_array().unwrap().len(), 32),
            Err(e) => assert_eq!(e.to_string(), ""),
        }
    }

    #[test]
    fn all_jobs_present() {
        match super::all_jobs(
            "6fe18c6e08a14d1782a9b9eb322269c1".to_string(),
            "".to_string(),
            "".to_string(),
        ) {
            Ok(resp) => assert_eq!(resp["jobs"].as_array().unwrap().len(), 30),
            Err(e) => assert_eq!(e.to_string(), ""),
        }
    }
}
