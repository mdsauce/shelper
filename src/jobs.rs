use super::users;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
/// Gets large batches of recently run jobs for a user.  Data
/// is in FULL mode and should be close to the /jobs API output
pub struct BulkFullJobs {
    pub jobs: Vec<JobDetails>,
}

#[derive(Serialize, Deserialize, Debug)]
/// `JobDetails` contains all the metadata for a single Sauce Labs
/// job.  A job is a Desktop, Android Emulator, or iOS Simulator session
/// at the time of this writing.  May change to include Real Devcices in the future
pub struct JobDetails {
    pub id: String,
    pub browser_version: String,
    pub os: String,
    pub browser: String,
    pub build: Option<String>,
    pub name: Option<String>,
    pub proxied: bool,
    pub manual: Option<bool>,
    pub video_url: String,
    pub log_url: String,
    pub creation_time: u64,
    pub owner: String,
    pub passed: bool,
    pub selenium_version: Option<String>,
    pub public: String,
    pub status: String,
    pub assigned_tunnel_id: Option<String>,
    pub automation_backend: String,
    pub error: Option<String>,
}

impl JobDetails {
    /// Construct new `JobDetails` object which contains all the metadata
    /// for a single job
    pub fn new(
        job_id: &str,
        owner: &users::User,
        super_admin: Option<&users::User>,
    ) -> Result<JobDetails, Box<dyn Error>> {
        let api_resp = match super::api::job_info(&owner, &job_id, super_admin) {
            Ok(body) => body,
            Err(e) => {
                // eprintln!("Problem getting the Job via API: {}", e);
                return Err(format!("Problem getting the Job via API: {}", e))?;
            }
        };
        let job_json: serde_json::Value = serde_json::from_str(&api_resp).unwrap();
        let job: JobDetails = match serde_json::from_value(job_json) {
            Ok(job) => job,
            Err(e) => panic!("{}\n{:?}", e, api_resp),
        };
        return Ok(job);
    }
}

impl BulkFullJobs {
    /// create new `BulkFullJobs` object consisting of a user's
    /// recent jobs. FULL API setting is enabled so more details are returned
    pub fn new(
        owner: &users::User,
        super_admin: Option<&users::User>,
        limit: u64,
    ) -> Result<BulkFullJobs, Box<dyn Error>> {
        let api = match super::api::recent_user_jobs(owner, super_admin, limit) {
            Ok(body) => body,
            Err(e) => panic!(
                "Problem getting jobs from {}/jobs API: {}",
                owner.creds.username, e
            ),
        };
        let jobs: Vec<JobDetails> = serde_json::from_str(&api).unwrap();
        return Ok(BulkFullJobs { jobs: jobs });
    }
}

#[test]
fn json_serializes_job_details_obj() {
    let job_text = r#"{
        "browser_short_version": "8.1",
        "video_url": "https://assets.saucelabs.com/jobs/30b9be879aa84313800c987b7aa325e8/video.mp4",
        "creation_time": 1585759333,
        "custom-data": null,
        "browser_version": "8.1.",
        "owner": "max.dobeck",
        "automation_backend": "appium",
        "id": "30b9be879aa84313800c987b7aa325e8",
        "collects_automator_log": false,
        "record_screenshots": true,
        "record_video": true,
        "build": null,
        "passed": true,
        "public": "team",
        "assigned_tunnel_id": null,
        "status": "complete",
        "log_url": "https://assets.saucelabs.com/jobs/30b9be879aa84313800c987b7aa325e8/selenium-server.log",
        "start_time": 1585759333,
        "proxied": false,
        "modification_time": 1585759387,
        "tags": [],
        "name": "Simple Android EMUSIM Test",
        "commands_not_successful": 1,
        "consolidated_status": "passed",
        "selenium_version": null,
        "manual": false,
        "end_time": 1585759387,
        "error": null,
        "os": "Linux",
        "breakpointed": null,
        "browser": "android"
      }"#;
    let job_json: serde_json::Value = serde_json::from_str(&job_text).unwrap();
    assert_eq!(job_json["id"], "30b9be879aa84313800c987b7aa325e8");
}

#[test]
fn get_job_info_produces_jobdetails() {
    let real_user_env_vars =
        super::users::User::new(Some("".to_string()), Some("".to_string()), None);
    let job_text = match super::api::job_info(
        &real_user_env_vars,
        "30b9be879aa84313800c987b7aa325e8",
        None,
    ) {
        Ok(j) => j,
        Err(e) => panic!("{}", e),
    };

    let job_json: serde_json::Value = serde_json::from_str(&job_text).unwrap();
    let job: JobDetails = match serde_json::from_value(job_json) {
        Ok(job) => job,
        Err(e) => panic!("{}\n{:?}", e, job_text),
    };
    println!("Job object {:?}", job);
    assert_eq!(job.id, "30b9be879aa84313800c987b7aa325e8");
}

#[test]
fn job_object_constructed() {
    let real_user_env_vars =
        super::users::User::new(Some("".to_string()), Some("".to_string()), None);

    let job: JobDetails = super::jobs::JobDetails::new(
        "30b9be879aa84313800c987b7aa325e8",
        &real_user_env_vars,
        Some(&real_user_env_vars),
    )
    .unwrap();
    assert_eq!(job.id, "30b9be879aa84313800c987b7aa325e8");
    assert_eq!(job.name, Some("Simple Android EMUSIM Test".to_string()));
}

#[test]
fn job_object_constructed_wo_admin() {
    let real_user_env_vars =
        super::users::User::new(Some("".to_string()), Some("".to_string()), None);

    let job: JobDetails = super::jobs::JobDetails::new(
        "30b9be879aa84313800c987b7aa325e8",
        &real_user_env_vars,
        None,
    )
    .unwrap();
    assert_eq!(job.id, "30b9be879aa84313800c987b7aa325e8");
    assert_eq!(job.name, Some("Simple Android EMUSIM Test".to_string()));
}

#[test]
fn json_serializes_to_bulk_full_jobs() {
    let raw_jobs = r#"[
        {
          "browser_short_version": "74",
          "video_url": "https://assets.saucelabs.com/jobs/db926070cb7243cbbf4b4561c9fd503f/video.flv",
          "creation_time": 1586477710,
          "custom-data": null,
          "browser_version": "74.0.",
          "owner": "max.dobeck",
          "automation_backend": "webdriver",
          "id": "db926070cb7243cbbf4b4561c9fd503f",
          "container": false,
          "deletion_time": null,
          "log_url": "https://assets.saucelabs.com/jobs/db926070cb7243cbbf4b4561c9fd503f/selenium-server.log",
          "record_screenshots": true,
          "record_video": true,
          "build": "Fantastic Soft Chips 6.1.9",
          "passed": true,
          "public": "team",
          "assigned_tunnel_id": null,
          "status": "complete",
          "performance_enabled": null,
          "tags": [],
          "start_time": 1586477711,
          "proxied": false,
          "modification_time": 1586477735,
          "consolidated_status": "passed",
          "commands_not_successful": 2,
          "command_counts": {
            "All": 14,
            "Error": 2
          },
          "name": "problem users",
          "proxy_host": null,
          "end_time": 1586477734,
          "error": null,
          "os": "Windows 10",
          "breakpointed": null,
          "browser": "firefox"
        },
        {
          "browser_short_version": "74",
          "video_url": "https://assets.saucelabs.com/jobs/3a4df450e7ce442bb1acb347f35d7c03/video.flv",
          "creation_time": 1586477708,
          "custom-data": null,
          "browser_version": "74.0.",
          "owner": "max.dobeck",
          "automation_backend": "webdriver",
          "id": "3a4df450e7ce442bb1acb347f35d7c03",
          "container": false,
          "deletion_time": null,
          "log_url": "https://assets.saucelabs.com/jobs/3a4df450e7ce442bb1acb347f35d7c03/selenium-server.log",
          "record_screenshots": true,
          "record_video": true,
          "build": "Fantastic Soft Chips 6.1.9",
          "passed": true,
          "public": "team",
          "assigned_tunnel_id": null,
          "status": "complete",
          "performance_enabled": null,
          "tags": [],
          "start_time": 1586477708,
          "proxied": false,
          "modification_time": 1586477774,
          "consolidated_status": "passed",
          "commands_not_successful": 2,
          "command_counts": {
            "All": 26,
            "Error": 2
          },
          "name": "login page",
          "proxy_host": null,
          "end_time": 1586477773,
          "error": null,
          "os": "Windows 10",
          "breakpointed": null,
          "browser": "firefox"
        }
      ]"#;
    let job_json: serde_json::Value = serde_json::from_str(&raw_jobs).unwrap();
    let latest_jobs: Vec<super::jobs::JobDetails> = serde_json::from_value(job_json).unwrap();
    println!("UserJobs object: {:?}", latest_jobs);
    assert_eq!(latest_jobs.len(), 2);
}

#[test]
fn create_bulk_full_jobs_obj() {
    let real_user = super::users::User::new(Some("".to_string()), Some("".to_string()), None);
    let latest_jobs: BulkFullJobs =
        super::jobs::BulkFullJobs::new(&real_user, Some(&real_user), 5).unwrap();
    assert_eq!(latest_jobs.jobs.len(), 5);
    assert_eq!(latest_jobs.jobs[0].owner, real_user.creds.username);
}
