use super::auth;
use super::sauce_errors;
use super::users;
use std::error::Error;

/// Returns the JSON info for a Job. `job_info` makes a REST call
/// with given credentials to fetch the details of a single job.
pub fn job_info(
    owner: &users::User,
    job_id: &str,
    super_admin: Option<&users::User>,
) -> Result<String, Box<dyn Error>> {
    let auth: &users::User = match super_admin {
        Some(admin) => admin,
        None => owner,
    };
    let job_info_api: std::string::String;
    match owner.region {
        users::Region::US => {
            job_info_api = format!("https://saucelabs.com/rest/v1.1/jobs/{}", job_id)
        }
        users::Region::EU => {
            job_info_api = format!(
                "https://eu-central-1.saucelabs.com/rest/v1.1/jobs/{}",
                job_id
            )
        }
    }

    let client = reqwest::blocking::Client::new();
    let resp = client
        .get(&job_info_api)
        .basic_auth(&auth.creds.username, Some(&auth.creds.access_key))
        .send()?;
    if !resp.status().is_success() {
        return Err(format!(
            "{} response during GET req to {}",
            resp.status(),
            job_info_api
        ))?;
    }
    return Ok(resp.text()?);
}

/// Get latest jobs for a user, limit of 500 at a time.  Returns
/// a JSON object with details of each job
pub fn recent_user_jobs(
    owner: &users::User,
    super_admin: Option<&users::User>,
    limit: u64,
) -> Result<String, Box<dyn Error>> {
    if limit > 500 {
        Err(format!(
            "{} is too many jobs.  Limit is 500 for /user/jobs API. See Sauce Labs API docs",
            limit
        ))?;
    }
    let auth: &users::User = match super_admin {
        Some(admin) => admin,
        None => owner,
    };
    let job_info_api = format!(
        "https://saucelabs.com/rest/v1/{}/jobs?limit={}&full=true",
        owner.creds.username, limit
    );

    let text_resp = reqwest::blocking::Client::new()
        .get(&job_info_api)
        .basic_auth(&auth.creds.username, Some(&auth.creds.access_key))
        .send()?
        .text()?;
    return Ok(text_resp);
}

/// `all_jobs` makes an API call to the builds/:id/jobs endpoint
/// to fetch the job details for each job in the build.
pub fn all_jobs(build_id: String, user: users::User) -> Result<serde_json::Value, Box<dyn Error>> {
    let build_api = format!("https://app.saucelabs.com/rest/v1/builds/{}/jobs", build_id);
    let resp: serde_json::Value = reqwest::blocking::Client::new()
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

/// `build_info` uses the v1/builds/:id endpoint to fetch
/// all the build meta data as a json object
pub fn build_info(build_id: &str, user: users::User) -> Result<serde_json::Value, Box<dyn Error>> {
    let build_api = format!("https://app.saucelabs.com/rest/v1/builds/{}", build_id);
    let resp: serde_json::Value = reqwest::blocking::Client::new()
        .get(&build_api)
        .basic_auth(&user.creds.username, Some(&user.creds.access_key))
        .send()?
        .json()?;
    return Ok(resp);
}

#[test]
/// use the recent_user_jobs api call and confirm
/// we only get the requested number of jobs as raw json
fn json_user_last_3_jobs() {
    let real_user_env_vars =
        super::users::User::new(None, None, None);

    let jobs_json = super::api::recent_user_jobs(&real_user_env_vars, None, 3).unwrap();

    let last_3_jobs: serde_json::Value = serde_json::from_str(&jobs_json).unwrap();
    println!(
        "{}\nLength of jobs_json: {}",
        last_3_jobs,
        last_3_jobs.as_array().unwrap().len()
    );
    assert_eq!(last_3_jobs.as_array().unwrap().len(), 3);
}

#[test]
fn over_500_limit() {
    let real_user_env_vars =
        super::users::User::new(None, None, None);

    // let _jobs_json = super::jobs::recent_user_jobs(&real_user_env_vars, None, 505).unwrap();
    match super::api::recent_user_jobs(&real_user_env_vars, None, 505) {
        Ok(_) => println!("Shouldn't be here"),
        Err(e) => {
            println!("{:?}", e);
            assert_eq!(
                "505 is too many jobs.  Limit is 500 for /user/jobs API. See Sauce Labs API docs",
                e.to_string()
            )
        }
    }
}

#[test]
#[should_panic]
fn all_jobs_bad_input() {
    let fake_user = super::users::User::new(
        Some("bad.user12b1581b".to_string()),
        Some("1285-fake-b128b519".to_string()),
        None,
    );
    match super::api::all_jobs("91ee45d589ce4177981bf22f911f22c5".to_string(), fake_user) {
        Ok(resp) => assert_eq!(resp["jobs"].as_array().unwrap().len(), 32),
        Err(e) => assert_eq!(e.to_string(), ""),
    }
}

#[test]
fn get_build_data() {
    let real_user = super::users::User::new(None, None, None);
    let resp = match super::api::build_info("91ee45d589ce4177981bf22f911f22c5", real_user) {
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
    let real_user = super::users::User::new(None, None, None);
    let mybuild = match super::builds::Build::new("91ee45d589ce4177981bf22f911f22c5", real_user) {
        Ok(b) => b,
        Err(e) => panic!("{}", e),
    };
    println!("my build ----->{:?}", mybuild);
    assert_eq!(
        mybuild.name,
        Some("generic build: grey Small Fresh Computer 6.0.4".to_string())
    )
}
