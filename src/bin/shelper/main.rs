#[macro_use]
extern crate clap;
extern crate shelper;
use clap::{App, Arg};
use shelper::jobs;
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
                .value_names(&["job id", "job URL"])
                .multiple(true)
                .takes_value(true),
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
                .value_name("sauce_access_key")
                .takes_value(true)
                .multiple(false),
        )
        .arg(
            Arg::with_name("region")
                .help("Region/datacenter to search.")
                .short("r")
                .long("region")
                .takes_value(true)
                .value_names(&["EU", "US"])
                .possible_value("EU")
                .possible_value("US"),
        )
        .arg(
            Arg::with_name("tunnel")
                .help("Find information about a tunnel. REQUIRES username that created the tunnel in the Owner flag(-o/--owner)")
                .short("t")
                .long("tunnel")
                .value_name("tunnel_id")
                .takes_value(true),
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
    if cmds.is_present("access_key") && cmds.is_present("owner") {
        let key_arg = cmds.value_of("access_key").unwrap().to_string();
        let owner_arg = cmds.value_of("owner").unwrap().to_string();
        match region {
            users::Region::US => owner = users::User::new(Some(owner_arg), Some(key_arg), None),
            users::Region::EU => {
                owner = users::User::new(Some(owner_arg), Some(key_arg), Some(users::Region::EU))
            }
        }
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
}
