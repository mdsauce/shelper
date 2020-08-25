use url::{ParseError, Url};

/// `get_session_id` strips the session id from a URL. If the user
/// passed in the literal session id then it ensures the session id is valid.
fn get_session_id(user_arg: &str) -> Result<String, String> {
    // did user pass in a job url?
    if Url::parse(user_arg) != Err(ParseError::RelativeUrlWithoutBase) {
        let job_url = match Url::parse(user_arg) {
            Ok(valid_url) => valid_url,
            Err(e) => Err(format!("Invalid Url: {}", e))?,
        };
        // split the url on the / symbol and return the value
        let split_url: Vec<_> = job_url.path().split("/").collect();
        return Ok(split_url[2].to_string());
    };
    let path: Vec<_> = user_arg.split("/").collect();
    // is this a literal session id? no url?
    if path.len() != 3 {
        // is it too short to be a session id?
        if user_arg.len() <= 20 {
            return Err(format!("Invalid Url or session id: {}", user_arg))?;
        }
        // return the literal session id (if it is valid)
        match path.len() {
            1 => return Ok(user_arg.to_string()),
            _ => return Err(format!("Invalid Url or session id: {}", user_arg))?,
        }
    }
    return Ok(path[2].to_string());
}

/// get_job_id returns a vector of sanitized job ids. It accepts the raw vector of
/// &strs, potential job ids. The potential job ids can be the URL or the literal session id.
pub fn get_job_id(jobs: Vec<&str>) -> Vec<String> {
    let mut job_ids: Vec<String> = Vec::new();
    for job in jobs {
        match get_session_id(job) {
            Ok(id) => job_ids.push(id.clone()),
            Err(_) => {
                eprintln!("Not a valid URL, skipping: {:?}", job);
                continue;
            }
        }
    }
    job_ids
}

#[test]
fn get_session_id_short_url() {
    let short_url = "https://app.saucelabs.com/tests/d73e717d7fcd46cd9927f369ba64fc28".to_string();
    let id = get_session_id(&short_url).unwrap();
    assert_eq!(id, "d73e717d7fcd46cd9927f369ba64fc28".to_string())
}

#[test]
fn get_id_from_partial_url() {
    let short_url = "app.saucelabs.com/tests/d73e717d7fcd46cd9927f369ba64fc28".to_string();
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
fn get_bulk_job_id() {
    let j = vec![
        "https://app.saucelabs.com/tests/d73e717d7fcd46cd9927f369ba64fc28#7",
        "https://app.saucelabs.com/tests/68d86d4795fa4efbbc628def8452866e",
        "app.saucelabs.com/tests/13a1e7e67bd841b0baf20c55317adb12",
        "junk data",
        "saucelabs.com/tests/13a1e7e67bd841b0baf2",
    ];
    assert_eq!(
        get_job_id(j),
        [
            "d73e717d7fcd46cd9927f369ba64fc28",
            "68d86d4795fa4efbbc628def8452866e",
            "13a1e7e67bd841b0baf20c55317adb12",
            "13a1e7e67bd841b0baf2"
        ]
    )
}
