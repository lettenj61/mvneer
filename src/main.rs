extern crate clap;
extern crate env_logger;
extern crate mvneer;

use clap::{App, AppSettings, Arg};

fn main() {

    env_logger::init().unwrap();

    let matches = App::new("Mvneer")
        .version(concat!(env!("CARGO_PKG_VERSION")))
        .about("Command line client for Maven Central REST Search API")
        .arg(Arg::with_name("artifact")
            .value_name("artifact_id")
            .help("An artifact to search")
            .takes_value(true)
            .required_unless("group"))
        .arg(Arg::with_name("group")
            .long("group")
            .short("g")
            .value_name("group_id")
            .help("A group id of the artifact to search")
            .takes_value(true)
            .required_unless("artifact"))
        .arg(Arg::with_name("rows")
            .long("rows")
            .short("n")
            .help("Set max number of records")
            .takes_value(true))
        .setting(AppSettings::ColorNever)
        .get_matches();

    let group = matches.value_of("group");
    let artifact = matches.value_of("artifact");
    let cond = match (group, artifact) {
        (Some(g), Some(a)) => format!("[group: {} / artifact: {}]", g, a),
        (Some(g), None) => format!("[group: {}]", g),
        (None, Some(a)) => format!("[artifact: {}]", a),
        _ => unreachable!()
    };

    let res = mvneer::search(&matches).unwrap();

    if res.num_found > 0 {
        println!("Found {} result for {} :", res.num_found, &cond);
        let l = res.data.len() as i64;
        for d in res.data {
            println!("    {}: latest version [{}] (versions behind: {})",
                     d.id,
                     d.latest_version,
                     d.version_count)
        }

        if l < res.num_found {
            println!("( ... Omitting another {} records)", res.num_found - l);
        }
    } else {
        println!("There are no artifact in the Central with parameter: {}", cond);
    }
}
