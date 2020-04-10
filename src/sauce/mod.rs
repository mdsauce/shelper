extern crate reqwest;
extern crate serde_json;

/// Authenticating a user or dealing with credentials for saucelabs.com REST API
pub mod auth;
/// API related to builds or build metadata
pub mod builds;
/// API related to individual Jobs (test sessions)
pub mod jobs;
/// Custom error messages thrown when encountering problems accessing the Sauce REST API
pub mod sauce_errors;
/// User data and Sauce REST API routes
pub mod users;
