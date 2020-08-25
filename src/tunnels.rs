use serde::{Deserialize, Serialize};

/// The literal `meatadata`, a nested json object
#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub command: String,
    pub release: String,
    pub hostname: String,
}

/// `TunnelMetadata` is the overall tunnel config, the maki used,
/// the owner, start time, overall duration etc etc.
#[derive(Serialize, Deserialize, Debug)]
pub struct TunnelMetadata {
    pub id: String,
    pub owner: String,
    pub use_kgp: bool,
    pub no_ssl_bump_list: Option<String>,
    pub direct_domains_list: Option<String>,
    #[serde(alias = "shared_tunnel")]
    pub shared: bool,
    #[serde(alias = "host")]
    pub maki: String,
    pub status: String,
    #[serde(alias = "user_shutdown")]
    pub shutdown_by_user: Option<bool>,
    #[serde(alias = "tunnel_identifier")]
    pub name: Option<String>,
    pub creation_time: Option<i64>,
    pub shutdown_time: Option<i64>,
    pub last_connected_time: Option<i64>,
    #[serde(skip)]
    pub duration: Option<i64>,
    pub metadata: Metadata,
}

impl TunnelMetadata {
    pub fn pretty_print(&self) {
        println!("Id: {}", &self.id);

        match &self.name {
            None => (),
            Some(name) => println!("Name(identifier): {}", name),
        }

        println!("Release Version: {}", &self.metadata.release);
        println!("Host: {}", &self.metadata.hostname);
        println!("Owner: {}", &self.owner);
        println!("Shared: {}", &self.shared);
        println!("Maki: {}", &self.maki);
        println!("Status: {}", &self.status);

        match &self.no_ssl_bump_list {
            None => (),
            Some(no_bump_domains) => println!("No Bump Domains: {}", no_bump_domains),
        }

        match &self.direct_domains_list {
            None => (),
            Some(direct_domains) => println!("Direct Domains: {}", direct_domains),
        }
        println!("Command Line Flags: {}", &self.metadata.command);
        // Leave for styling
        println!("")
    }
}

#[test]
fn basic_tunnel_json() {
    let raw_tunnel = r#"{
        "team_ids": [
          "*"
        ],
        "ssh_port": 443,
        "creation_time": 1597790974,
        "domain_names": [
          "sauce-connect.proxy"
        ],
        "owner": "max.dobeck",
        "use_kgp": true,
        "id": "8144b3635a6f414188ac5eb683ac5b73",
        "extra_info": "{\"tunnel_cert\": \"public\", \"inject_job_id\": true, \"backend\": \"kgp\", \"metrics_host\": \"localhost\", \"metrics_port\": 8888}",
        "direct_domains": null,
        "vm_version": "",
        "no_ssl_bump_domains": null,
        "shared_tunnel": false,
        "metadata": {
          "hostname": "SL-0465",
          "git_version": "ad61662 ",
          "platform": "Darwin 19.6.0 Darwin Kernel Version 19.6.0: Thu Jun 18 20:49:00 PDT 2020; root:xnu-6153.141.1~1/RELEASE_X86_64 x86_64",
          "command": "bin/sc -u max.dobeck -k **** ",
          "build": "ᐿ",
          "release": "4.6.2",
          "nofile_limit": 12000
        },
        "status": "terminated",
        "shutdown_time": 1597791014,
        "host": "maki584.miso.saucelabs.com",
        "ip_address": null,
        "last_connected": 1597790988,
        "user_shutdown": true,
        "use_caching_proxy": null,
        "launch_time": 1597790983,
        "no_proxy_caching": false,
        "tunnel_identifier": ""
      }"#;
    let tunnel_test: TunnelMetadata = match serde_json::from_str(raw_tunnel) {
        Ok(tunnel_metadata) => tunnel_metadata,
        Err(e) => panic!("{}\n", e),
    };
    println!("tunnel object: \n{:?}", tunnel_test);
    assert_eq!(tunnel_test.owner, "max.dobeck")
}
