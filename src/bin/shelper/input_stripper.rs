use url::Url;

pub fn get_session_id(user_arg: &str) -> Result<String, String> {
    let job_url = match Url::parse(user_arg) {
        Ok(valid_url) => valid_url,
        Err(e) => Err(format!("Invalid Url: {}", e))?,
    };
    let split_url: Vec<_> = job_url.path().split("/").collect();
    Ok(split_url[2].to_string().clone())
}

pub fn sanitize_jobs(jobs: Vec<&str>) -> Vec<String> {
    let mut job_ids: Vec<String> = Vec::new();
    for job in jobs {
        match get_session_id(job) {
            Ok(id) => job_ids.push(id.clone()),
            Err(_) => job_ids.push(job.to_string().clone()),
        }
    }
    job_ids
}

#[test]
fn get_sesssion_id_short_url() {
    let short_url = "https://app.saucelabs.com/tests/d73e717d7fcd46cd9927f369ba64fc28".to_string();
    let id = get_session_id(&short_url).unwrap();
    assert_eq!(id, "d73e717d7fcd46cd9927f369ba64fc28".to_string())
}

#[test]
fn get_session_id_hash_url() {
    let long_url = "https://app.saucelabs.com/tests/d73e717d7fcd46cd9927f369ba64fc28#7".to_string();
    assert_eq!(
        get_session_id(&long_url).unwrap(),
        "d73e717d7fcd46cd9927f369ba64fc28".to_string()
    )
}

#[test]
fn jobs_get_sanitized() {
    let j = vec![
        "https://app.saucelabs.com/tests/d73e717d7fcd46cd9927f369ba64fc28#7",
        "https://app.saucelabs.com/tests/68d86d4795fa4efbbc628def8452866e",
        "https://app.saucelabs.com/tests/13a1e7e67bd841b0baf20c55317adb12",
    ];
    println!("{:?}", sanitize_jobs(j.clone()));
    assert_eq!(
        sanitize_jobs(j),
        [
            "d73e717d7fcd46cd9927f369ba64fc28",
            "68d86d4795fa4efbbc628def8452866e",
            "13a1e7e67bd841b0baf20c55317adb12"
        ]
    )
}
