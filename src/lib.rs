extern crate curl;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate url;

// use std::io;
use std::error::Error;
use std::str;

use curl::easy::Easy;
use serde_json::Value as Json;
use url::Url;
use url::percent_encoding::*;

const API: &'static str = "http://search.maven.org/solrsearch/select";

pub struct Client {}

impl Client {
    pub fn run(group: &str, artifact: &str) -> Result<(), String> {

        let query = SearchQuery::new(group, artifact);
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
            }).unwrap();
            try!(transfer.perform().map_err(|e| e.description().to_owned()));
        }

        let res = str::from_utf8(buf.as_slice()).unwrap();
        debug!("Received data:");
        debug!("{}", res);

        let json: Json = serde_json::from_str(res).unwrap();

        let docs_head = json["response"]["docs"][0].clone();
        let artifact: Artifact = try!(serde_json::from_value(docs_head)
            .map_err(|e| e.description().to_owned()));

        let printer = Print::ScalaSBT;
        debug!("{:?}", &artifact);
        debug!("{}", printer.render(&artifact));

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Artifact {
    #[serde(rename = "g")]
    group_id: String,
    #[serde(rename = "a")]
    artifact_id: String,
    #[serde(rename = "latestVersion")]
    latest_version: String,
    #[serde(rename = "repositoryId")]
    repository_id: String,
    #[serde(rename = "versionCount")]
    version_count: i32,
}

#[derive(Debug)]
pub struct SearchQuery {
    pub group_id: Option<String>,
    pub artifact_id: Option<String>,
    pub packaging: String,
    // pub version: String, TODO:
    pub rows: i32,
    pub format: String
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

    pub fn new(group_id: &str, artifact_id: &str) -> SearchQuery {
        let mut q = SearchQuery::default();
        q.group_id = Some(group_id.into());
        q.artifact_id = Some(artifact_id.into());
        q
    }

    fn encode_params(&self) -> Option<String> {
        let both_ids = self.group_id.as_ref()
            .and_then(|g| self.artifact_id.as_ref().map(|a| (g, a)));
        if let Some((gid, aid)) = both_ids {
            let raw = format!(r#"g:"{}" AND a:"{}""#, &gid, &aid);
            Some(percent_encode(&raw.as_bytes(), SIMPLE_ENCODE_SET).collect::<String>())
        } else {
            None
        }
    }

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

pub enum Print {
    Buildr,
    Gradle,
    Grape,
    Ivy,
    Leiningen,
    MavenPom,
    ScalaSBT,
    ScalaAmmonite,
}

impl Print {
    pub fn render(&self, artifact: &Artifact) -> String {
        match *self {
            Print::ScalaSBT => {
                format!(r#"libraryDependencies += "{}" % "{}" % "{}""#,
                        &artifact.group_id,
                        &artifact.artifact_id,
                        &artifact.latest_version)
            },
            _ => unimplemented!()
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