use url::Url;

pub fn get_session_id(user_arg: &std::string::String) -> Result<String, String> {
    let job_url = match Url::parse(user_arg) {
        Ok(valid_url) => valid_url,
        Err(e) => Err(format!("Invalid Url: {}", e))?,
    };
    let split_url: Vec<_> = job_url.path().split("/").collect();
    Ok(split_url[2].to_string().clone())
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
    assert_eq!(get_session_id(&long_url).unwrap(), "d73e717d7fcd46cd9927f369ba64fc28".to_string())
}