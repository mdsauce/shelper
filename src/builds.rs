use super::users;
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
/// All the jobs in a build
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
        let info = super::api::build_info(build_id, user)?;
        let build: Build = serde_json::from_value(info)?;
        return Ok(build);
    }
}
