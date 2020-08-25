#[macro_use]
extern crate clap;
extern crate shelper;
use clap::{App, Arg};
use shelper::api;
use shelper::jobs;
use shelper::tunnels;
use shelper::users;
mod input_stripper;

fn main() {
    let cmds = App::new("shelper")
        .version(env!("CARGO_PKG_VERSION"))
        .author(crate_authors!())
        .about("Get details about jobs and tunnels")
        .version(crate_version!())
        .arg(
            Arg::with_name("version")
                .long("version")
                .help("Print the current version of Shelper")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("job")
                .long("job")
                .short("j")
                .help("Get job details.  Takes a URL link to a session or a Job ID string")
                .value_name("one or more job")
                .multiple(true)
                .takes_value(true)
                .possible_value("job URL link")
                .possible_value("session id")
        )
        .arg(
            Arg::with_name("owner")
                .help("Sauce account that owns a sauce resource (tunnel, job, asset)")
                .long("owner")
                .short("o")
                .value_name("sauce_username")
                .takes_value(true)
                .multiple(false),
        )
        .arg(
            Arg::with_name("access_key")
                .help("Sauce Access Key")
                .short("k")
                .long("key")
                .value_name("key")
                .takes_value(true)
                .multiple(false)
                .possible_value("sauce-access-key"),
        )
        .arg(
            Arg::with_name("region")
                .help("Region/datacenter to search.")
                .short("r")
                .long("region")
                .takes_value(true)
                .value_name("region")
                .possible_value("EU")
                .possible_value("US"),
        )
        .arg(
            Arg::with_name("tunnel")
                .help(r#"Get information about a tunnel. REQUIRES:
- the sauce username that created the tunnel, either in the Owner flag(-o/--owner) OR as an env. variable
- the access key used to authenticate (env variable or as a flag)
- the tunnel id, provided at tunnel runtime"#)
                .short("t")
                .long("tunnelinfo")
                .value_name("tunnel_id")
                .multiple(true)
                .takes_value(true)
        )
        .get_matches();

    if cmds.is_present("version") {
        println!("shelper version {}", env!("CARGO_PKG_VERSION"))
    }

    // if the user doesn't specify a region default to US
    let region = match cmds.is_present("region") {
        true => value_t!(cmds, "region", users::Region).unwrap_or_else(|e| e.exit()),
        false => users::Region::US,
    };

    // Build out a user w/ key + username + region
    let owner: users::User;
    if cmds.is_present("owner") && cmds.is_present("access_key") {
        let key_arg = cmds.value_of("access_key").unwrap().to_string();
        let owner_arg = cmds.value_of("owner").unwrap().to_string();
        match region {
            users::Region::US => owner = users::User::new(Some(owner_arg), Some(key_arg), None),
            users::Region::EU => {
                owner = users::User::new(Some(owner_arg), Some(key_arg), Some(users::Region::EU))
            }
        }
    } else if cmds.is_present("owner") {
        owner = users::User::new(
            Some(cmds.value_of("owner").unwrap().to_string()),
            None,
            Some(region),
        );
    // println!("new owner {:?}", owner);
    } else {
        match region {
            users::Region::US => owner = users::User::new(None, None, None),
            users::Region::EU => owner = users::User::new(None, None, Some(users::Region::EU)),
        }
    }

    if let Some(jobs) = cmds.values_of("job") {
        let sanitized_jobs = input_stripper::get_job_id(jobs.collect());
        let job_count = sanitized_jobs.len();
        for (i, job) in sanitized_jobs.iter().enumerate() {
            let deets: shelper::jobs::JobDetails;
            if !cmds.is_present("access_key") {
                let admin = users::User::new(None, None, None);
                deets = match jobs::JobDetails::new(job, &owner, Some(&admin)) {
                    Ok(deets) => deets,
                    Err(e) => {
                        eprintln!("{}", e);
                        continue;
                    }
                }
            } else {
                deets = match jobs::JobDetails::new(job, &owner, Some(&owner)) {
                    Ok(deets) => deets,
                    Err(e) => {
                        eprintln!("{}", e);
                        continue;
                    }
                }
            }
            println!("{}/{}", i + 1, job_count);
            deets.pretty_print();
            println!("");
        }
    }

    if let Some(t) = cmds.values_of("tunnel") {
        // println!("Splitting: {:?}", t);
        let tunnel_list: Vec<&str> = t.collect();
        let tunnel_count = tunnel_list.len();
        // println!("{:?}", tunnels)
        for (i, tunnel) in tunnel_list.iter().enumerate() {
            if !cmds.is_present("access_key") {
                let admin = users::User::new(None, None, None);
                let info: tunnels::TunnelMetadata =
                    match api::tunnel_raw(&owner, &tunnel, Some(&admin)) {
                        Ok(resp) => serde_json::from_str(&resp).unwrap(),
                        Err(e) => {
                            eprintln!("{}", e);
                            continue;
                        }
                    };
                println!("{}/{}", i + 1, tunnel_count);
                info.pretty_print();
            }
        }
    }
}
