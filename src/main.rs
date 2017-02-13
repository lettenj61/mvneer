extern crate clap;
extern crate env_logger;
extern crate mvneer;
#[macro_use]
extern crate serde_derive;

use clap::{App, AppSettings, Arg};

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
            .help("Print result as dependency string for gradle|lein|ivy|pom|sbt")
            .takes_value(true)
            .possible_values(&["gradle", "lein", "pom", "sbt"])
            .hide_possible_values(true))
        .setting(AppSettings::ColorNever)
        .get_matches();

    let group = matches.value_of("group");
    let artifact = matches.value_of("artifact");
    let cond = match (group, artifact) {
        (Some(g), Some(a)) => format!("[{} / {}]", g, a),
        (Some(v), None) | (None, Some(v)) => format!("[{}]", v),
        _ => unreachable!()
    };

    let res = mvneer::search(&matches).unwrap();

    if res.num_found > 0 {
        println!("Found {} result for {} =>", res.num_found, &cond);
        for d in res.data {
            println!("    {}: latest version [{}] (versions behind: {})",
                     d.id,
                     d.latest_version,
                     d.version_count)
        }
    }
}
