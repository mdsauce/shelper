pub mod build {
    use std::error::Error;
    use std::fmt;

    #[derive(Debug)]
    pub struct NoJobs {
        username: String,
        masked_key: String,
        url: String,
        resp: serde_json::Value,
    }

    impl NoJobs {
        pub fn new(
            username: &String,
            masked_key: &String,
            url: &String,
            resp: serde_json::Value,
        ) -> NoJobs {
            NoJobs {
                username: username.to_string(),
                masked_key: masked_key.to_string(),
                url: url.to_string(),
                resp: resp,
            }
        }
    }

    impl fmt::Display for NoJobs {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "Something went wrong with the request using user {}:{}****** {}.  Response: {}",
                self.username, self.masked_key, self.url, self.resp
            )
        }
    }

    impl Error for NoJobs {}
}
