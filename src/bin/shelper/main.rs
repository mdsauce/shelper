#[macro_use]
extern crate clap;
extern crate shelper;
use clap::{App, Arg};
use shelper::jobs;
use shelper::users;

fn main() {
    let cmds = App::new("shelper")
        .version(env!("CARGO_PKG_VERSION"))
        .author(crate_authors!())
        .about("Get details about jobs and tunnels")
        .version(crate_version!())
        .arg(
            Arg::with_name("version")
                .long("version")
                .help("Print the current version of Gimme")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("job")
                .long("job")
                .short("j")
                .help("Get job details.  Takes a URL link to a session or Job ID")
                .multiple(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("owner")
                .help("The sauce user who ran a job")
                .long("owner")
                .short("o")
                .takes_value(true)
                .multiple(false),
        )
        .arg(
            Arg::with_name("access_key")
                .help("Sauce Access Key")
                .short("k")
                .long("key")
                .takes_value(true)
                .multiple(false),
        )
        .arg(
            Arg::with_name("region")
                .help("Region/datacenter to search.")
                .short("r")
                .long("region")
                .takes_value(true)
                .possible_value("EU")
                .possible_value("US"),
        )
        .get_matches();

    if cmds.is_present("version") {
        println!("shelper version {}", env!("CARGO_PKG_VERSION"))
    }

    let region = match cmds.is_present("region") {
        true => value_t!(cmds, "region", users::Region).unwrap_or_else(|e| e.exit()),
        false => users::Region::US,
    };

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
        let job_count = jobs.len();
        for (i, job) in jobs.enumerate() {
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
