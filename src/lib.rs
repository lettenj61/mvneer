extern crate clap;
extern crate curl;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate url;

use std::error::Error;
use std::str;

use curl::easy::Easy;
use serde_json::Value as Json;
use url::Url;
use url::percent_encoding::{percent_encode, SIMPLE_ENCODE_SET};

const API: &'static str = "http://search.maven.org/solrsearch/select";

pub fn search(matches: &clap::ArgMatches) -> Result<SearchResult, String> {

    let query = SearchQuery::from_args(matches);

    let url = try!(query.resolve_url());
    info!("Start searching with URL: {:?}", url);

    let mut buf = Vec::new();
    let mut handle = Easy::new();
    handle.url(url.as_ref()).unwrap();

    // need another scope to isolate the lifetime
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|data| {
                buf.extend_from_slice(data);
                Ok(data.len())
            })
            .unwrap();
        try!(transfer.perform().map_err(|e| e.description().to_owned()));
    }

    let res = str::from_utf8(buf.as_slice()).unwrap();
    debug!("Received data:");
    debug!("{}", res);

    let json: Json = serde_json::from_str(&res).unwrap();
    let mut result = SearchResult::default();
    if let Some(num_found) = json["response"]["numFound"].as_i64() {
        result.num_found = num_found;
    }

    let docs = json["response"]["docs"].clone();
    let artifacts: Vec<Artifact> = try!(serde_json::from_value(docs)
        .map_err(|e| e.description().to_owned()));

    debug!("{:?}", &artifacts);
    result.data = artifacts;

    Ok(result)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Artifact {
    pub id: String,
    #[serde(rename = "g")]
    pub group_id: String,
    #[serde(rename = "a")]
    pub artifact_id: String,
    #[serde(rename = "latestVersion")]
    pub latest_version: String,
    #[serde(rename = "repositoryId")]
    pub repository_id: String,
    #[serde(rename = "versionCount")]
    pub version_count: i32,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SearchResult {
    pub num_found: i64,
    pub data: Vec<Artifact>,
}

/// URL query format used when mimicly invoking Advanced Search Box of the Central.
#[derive(Debug)]
pub struct SearchQuery {
    pub group_id: Option<String>,
    pub artifact_id: Option<String>,
    pub packaging: String,
    // pub version: String, TODO:
    pub rows: i32,
    pub format: String,
}

impl Default for SearchQuery {
    fn default() -> SearchQuery {
        SearchQuery {
            group_id: None,
            artifact_id: None,
            packaging: "jar".to_string(),
            rows: 1,
            format: "json".to_string(),
        }
    }
}

impl SearchQuery {

    /// Create query without specifying groups or atrifacts.
    pub fn new(group_id: &str, artifact_id: &str) -> SearchQuery {
        let mut q = SearchQuery::default();
        q.group_id = Some(group_id.into());
        q.artifact_id = Some(artifact_id.into());
        q
    }

    /// Create query parameter from parsed arguments
    pub fn from_args(args: &clap::ArgMatches) -> SearchQuery {
        let mut q = SearchQuery::default();
        if args.is_present("rows") {
            q.rows = args.value_of("rows").map(|n| n.parse().unwrap_or(1)).unwrap();
        }
        if args.is_present("group") {
            q.group_id = args.value_of("group").map(|g| g.to_owned());
        }
        if args.is_present("artifact") {
            q.artifact_id = args.value_of("artifact").map(|a| a.to_owned());
        }
        if args.is_present("print") {
            q.format = args.value_of("print").unwrap().to_owned();
        }
        q
    }

    fn encode_params(&self) -> Option<String> {
        match (self.group_id.as_ref(), self.artifact_id.as_ref()) {
            (Some(g), Some(a)) => {
                let raw = format!("g:\"{}\" AND a:\"{}\"", g, a);
                Some(percent_encode(raw.as_bytes(), SIMPLE_ENCODE_SET)
                    .collect::<String>())
            },
            (Some(g), None) => {
                let raw = format!("g:\"{}\"", g);
                Some(percent_encode(raw.as_bytes(), SIMPLE_ENCODE_SET)
                    .collect::<String>())
            },
            (None, Some(a)) => {
                let raw = format!("a:\"{}\"", a);
                Some(percent_encode(raw.as_bytes(), SIMPLE_ENCODE_SET)
                    .collect::<String>())
            }
            _ => None,
        }
    }

    /// Resolve search query into absolute http URL.
    pub fn resolve_url(&self) -> Result<Url, String> {
        if let Some(params) = self.encode_params() {
            let mut url = Url::parse(API).unwrap();
            url.query_pairs_mut()
                .append_pair("q", &params)
                .append_pair("rows", &self.rows.to_string())
                .append_pair("wt", &self.format);
            Ok(url)
        } else {
            Err("Group ID and Artifact ID should not be left empty".into())
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    // Recent stats in the Central, at Feb 13, 2017.
    fn scalalib() -> Artifact {
        Artifact {
            group_id: "org.scala-lang".to_string(),
            artifact_id: "scala-library".to_string(),
            latest_version: "2.12.1".to_string(),
            repository_id: "central".to_string(),
            version_count: 153,
        }
    }

    #[test]
    fn search_query() {
        let q = SearchQuery::new("org.typelevel", "cats_2.11");
        assert!(q.resolve_url().is_ok());
    }

    #[test]
    #[should_panic]
    fn invalid_parameter() {
        let q = SearchQuery::default();
        q.resolve_url().unwrap();
    }
}
