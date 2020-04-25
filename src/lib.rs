//! # Shelper
//!
//! Shelper or Sauce_Helper is a library wrapping the Sauce Labs public API.
//! It comes with a binary CLI `shelper.exe` to perform common commands like getting details of job(s),
//! fetching the latest jobs run by a specific user, or fetching the metadata of a build.

extern crate chrono;
extern crate reqwest;
extern crate serde_json;

/// API calls
pub mod api;
/// REST API wrapper for sauce labs.  Gets data about tests.

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
