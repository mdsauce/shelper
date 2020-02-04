use super::auth;
use super::sauce_errors;
use super::users;
extern crate reqwest;
extern crate serde_json;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Build {
    pub status: String,
    pub name: Option<String>,
    pub deletion_time: Option<String>,
    pub jobs: Jobs,
    pub org_id: String,
    pub start_time: u64,
    pub creation_time: u64,
    pub modification_time: u64,
    pub end_time: u64,
    pub number: Option<String>,
    pub public: bool,
    pub prefix: Option<String>,
    pub passed: bool,
    pub owner: String,
    pub run: Option<i64>,
    pub team_id: String,
    pub group_id: Option<String>,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
/// Jobs object inside a Build contains a count of different job statuses
pub struct Jobs {
    pub completed: i32,
    pub finished: i32,
    pub queued: i32,
    pub failed: i32,
    pub running: i32,
    pub passed: i32,
    pub errored: i32,
    pub public: i32,
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
pub fn all_jobs(build_id: String, user: users::User) -> Result<serde_json::Value, Box<dyn Error>> {
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
        match super::all_jobs("91ee45d589ce4177981bf22f911f22c5".to_string(), fake_user) {
            Ok(resp) => assert_eq!(resp["jobs"].as_array().unwrap().len(), 32),
            Err(e) => assert_eq!(e.to_string(), ""),
        }
    }

    #[test]
    fn all_jobs_present() {
        let real_user = super::users::User::new("".to_string(), "".to_string(), None);
        let resp = match super::all_jobs("6fe18c6e08a14d1782a9b9eb322269c1".to_string(), real_user) {
            Ok(resp) => resp,
            Err(e) => panic!("{}", e),
        };
        assert_eq!(resp["jobs"].as_array().unwrap().len(), 30);
        // println!("{:?}", resp);
    }

    #[test]
    fn get_build_data() {
        let real_user = super::users::User::new("".to_string(), "".to_string(), None);
        let resp = match super::build_info("91ee45d589ce4177981bf22f911f22c5", real_user) {
            Ok(resp) => resp,
            Err(e) => panic!("{}", e),
        };
        assert_eq!(resp["jobs"]["finished"], 32);
        println!(
            "Build 91ee45d589ce4177981bf22f911f22c5 has data ---------> {:?}",
            resp
        );
    }

    #[test]
    fn create_new_build_object() {
        let real_user = super::users::User::new("".to_string(), "".to_string(), None);
        let mybuild = match super::Build::new("91ee45d589ce4177981bf22f911f22c5", real_user) {
            Ok(b) => b,
            Err(e) => panic!("{}", e),
        };
        println!("my build ----->{:?}", mybuild);
        assert_eq!(
            mybuild.name,
            Some("generic build: grey Small Fresh Computer 6.0.4".to_string())
        )
    }
}
