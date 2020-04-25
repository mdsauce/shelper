use url::{ParseError, Url};

fn get_session_id(user_arg: &str) -> Result<String, String> {
    if Url::parse(user_arg) != Err(ParseError::RelativeUrlWithoutBase) {
        let job_url = match Url::parse(user_arg) {
            Ok(valid_url) => valid_url,
            Err(e) => Err(format!("Invalid Url: {}", e))?,
        };
        let split_url: Vec<_> = job_url.path().split("/").collect();
        return Ok(split_url[2].to_string().clone());
    };
    let path: Vec<_> = user_arg.split("/").collect();
    if path.len() != 3 {
        if user_arg.len() <= 25 {
            return Err(format!("Invalid Url: {}", user_arg))?;
        }
        match path.len() {
            1 => return Ok(user_arg.to_string().clone()),
            _ => return Err(format!("Invalid Url: {}", user_arg))?,
        }
    }
    return Ok(path[2].to_string());
}

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
