extern crate clap;
extern crate env_logger;
extern crate mvneer;
extern crate semver;
#[macro_use]
extern crate serde_derive;

use std::error::Error;
use clap::{App, AppSettings, Arg};

fn validate_semver(v: String) -> Result<(), String> {
    semver::Version::parse(&v).map(|_| ()).map_err(|e| e.description().to_owned())
}

fn main() {

    env_logger::init().unwrap();

    let matches = App::new("Mvneer")
        .version(concat!(env!("CARGO_PKG_VERSION")))
        .about("Command line client for Maven Central REST Search API")
        .arg(Arg::with_name("group")
            .value_name("group_id")
            .help("A group id of the artifact to search")
            .takes_value(true)
            .required_unless("artifact"))
        .arg(Arg::with_name("artifact")
            .value_name("artifact_id")
            .help("An artifact to search")
            .takes_value(true)
            .required_unless("group"))
        .arg(Arg::with_name("rows")
            .long("rows")
            .short("n")
            .help("Limit output record to given number")
            .takes_value(true))
        .arg(Arg::with_name("print")
            .long("print")
            .help("Print result as dependency string for sbt|sbt-scalajs")
            .takes_value(true)
            .possible_values(&["gradle", "lein", "pom", "sbt"])
            .hide_possible_values(true))
        .arg(Arg::with_name("scala")
            .long("scala")
            .help("Specify Scala compiler version in SemVer format")
            .takes_value(true)
            .validator(validate_semver))
        .arg(Arg::with_name("scalajs")
            .long("scalajs")
            .help("Specify Scala.js compiler version in SemVer format")
            .takes_value(true)
            .requires("scala")
            .validator(validate_semver))
        .setting(AppSettings::ColorNever)
        .get_matches();

    let group = matches.value_of("group");
    let artifact = matches.value_of("artifact");
    let cond = match (group, artifact) {
        (Some(g), Some(a)) => format!("[{} / {}]", g, a),
        (Some(v), None) | (None, Some(v)) => format!("[{}]", v),
        _ => unreachable!(),
    };

    let res = mvneer::search(&matches).unwrap();

    if res.num_found > 0 {
        println!("Found {} result for {} :", res.num_found, &cond);
        for d in res.data {
            println!("    {}: latest version [{}] (versions behind: {})",
                     d.id,
                     d.latest_version,
                     d.version_count)
        }
    }
}
