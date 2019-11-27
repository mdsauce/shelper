use super::auth;
use super::sauce_errors;
use super::users;
extern crate reqwest;
extern crate serde_json;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Build {
    status: String,
    name: Option<String>,
    deletion_time: Option<String>,
    jobs: Jobs,
    org_id: String,
    start_time: u64,
    creation_time: u64,
    modification_time: u64,
    end_time: u64,
    number: Option<String>,
    public: bool,
    prefix: Option<String>,
    passed: bool,
    owner: String,
    run: Option<i64>,
    team_id: String,
    group_id: Option<String>,
    id: String,
}

#[derive(Serialize, Deserialize, Debug)]
/// Jobs object inside a Build contains a count of different job statuses
pub struct Jobs {
    completed: i64,
    finished: i64,
    queued: i64,
    failed: i64,
    running: i64,
    passed: i64,
    errored: i64,
    public: i64,
}

impl Build {
    pub fn new(build_id: &str, user: users::User) -> Result<Build, Box<dyn Error>> {
        let info = build_info(build_id, user)?;
        let build: Build = serde_json::from_value(info)?;
        return Ok(build);
    }
}

/// Gets a JSON array all jobs in a build.
/// Needs the `Build Id` and `Owner` of a build and returns a json array of all `jobs` and `job` data.
pub fn build_jobs(
    build_id: String,
    user: users::User,
) -> Result<serde_json::Value, Box<dyn Error>> {
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

/// Requires Build Id and User object.
/// Returns an error or a JSON object about a specific build.
///
/// Example return:
/// ```json
/// {"status": "success", "jobs": {"completed": 0, "finished": 32, "queued": 0, "failed": 0, "running": 0, "passed": 32, "errored": 0, "public": 0}, "name": "generic build: grey Small Fresh Computer 6.0.4", "deletion_time": null, "org_id": "695f050ceec84e6e99c5288982eed1b1", "start_time": 1573166295, "creation_time": 1573166306, "number": null, "public": false, "modification_time": 1573166370, "prefix": null, "end_time": 1573166338, "passed": true, "owner": "max.dobeck", "run": 0, "team_id": "28fed5500a474c03a06b928d8efed1e7", "group_id": null, "id": "91ee45d589ce4177981bf22f911f22c5"}
/// ```
pub fn build_info(build_id: &str, user: users::User) -> Result<serde_json::Value, Box<dyn Error>> {
    let build_api = format!("https://app.saucelabs.com/rest/v1/builds/{}", build_id);
    let resp: serde_json::Value = reqwest::Client::new()
        .get(&build_api)
        .basic_auth(&user.creds.username, Some(&user.creds.access_key))
        .send()?
        .json()?;
    return Ok(resp);
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
        match super::build_jobs("91ee45d589ce4177981bf22f911f22c5".to_string(), fake_user) {
            Ok(resp) => assert_eq!(resp["jobs"].as_array().unwrap().len(), 32),
            Err(e) => assert_eq!(e.to_string(), ""),
        }
    }

    #[test]
    fn all_jobs_present() {
        let real_user = super::users::User::new("".to_string(), "".to_string(), None);
        match super::build_jobs("6fe18c6e08a14d1782a9b9eb322269c1".to_string(), real_user) {
            Ok(resp) => assert_eq!(resp["jobs"].as_array().unwrap().len(), 30),
            Err(e) => assert_eq!(e.to_string(), ""),
        }
    }

    #[test]
    fn build_data_retrievable() {
        let real_user = super::users::User::new("".to_string(), "".to_string(), None);
        let resp = match super::build_info("91ee45d589ce4177981bf22f911f22c5", real_user) {
            Ok(resp) => assert_eq!(resp["jobs"]["finished"], 32),
            Err(e) => assert_eq!(e.to_string(), ""),
        };
        println!(
            "Build 91ee45d589ce4177981bf22f911f22c5 has data ---------> {:?}",
            resp
        );
    }

    #[test]
    fn create_new_build_object() {
        let real_user = super::users::User::new("".to_string(), "".to_string(), None);
        let mybuild =  match super::Build::new("91ee45d589ce4177981bf22f911f22c5", real_user) {
            Ok(b) => b,
            Err(e) => panic!("{}", e),
        };
        println!("my build ----->{:?}", mybuild);
        assert_eq!(mybuild.name, Some("generic build: grey Small Fresh Computer 6.0.4".to_string()))
    }
}
