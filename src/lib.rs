extern crate curl;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate url;

// use std::io;
use std::str;

use curl::easy::Easy;
use serde_json::Value as Json;
use url::Url;
use url::percent_encoding::*;

const CENTRAL: &'static str = r#"
http://search.maven.org/solrsearch/select?q=g:"org.typelevel" AND a:"cats_2.11"&rows=1&wt=json
"#;

const API: &'static str = "http://search.maven.org/solrsearch/select";

pub struct Client {}

impl Client {
    pub fn run() {
        let url = percent_encode(CENTRAL.trim().as_bytes(), QUERY_ENCODE_SET)
            .collect::<String>();
        let url = Url::parse(&url).unwrap();
        info!("{:?}", url);

        let mut v = Vec::new();
        let mut handle = Easy::new();
        handle.url(url.as_ref()).unwrap();

        // need another scope to isolate the lifetime
        {
            let mut transfer = handle.transfer();
            transfer.write_function(|data| {
                v.extend_from_slice(data);
                Ok(data.len())
            }).unwrap();
            transfer.perform().unwrap();
        }

        let res = str::from_utf8(v.as_slice()).unwrap();

        let json: Json = serde_json::from_str(res).unwrap();
        //println!("{:?}", json["response"]);

        println!("{}", res);
        //println!("{:?}", json["response"]["numfound"]);
        //println!("{:?}", json["response"]["docs"][0]);

        let docs_head = json["response"]["docs"][0].clone();
        let jar: Artifact = serde_json::from_value(docs_head).unwrap();
        println!("{:?}", jar);
        let printer = Print::ScalaSBT;
        println!("{}", printer.render(&jar));
    }
}

#[derive(Debug, Deserialize, Serialize)]
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

    fn encode_params(&self) -> Option<String> {
        let both_ids = self.group_id.as_ref()
            .and_then(|g| self.artifact_id.as_ref().map(|a| (g, a)));
        if let Some((gid, aid)) = both_ids {
            let raw = format!(r#"g:"{}" AND a:"{}""#, &gid, &aid);
            Some(percent_encode(&raw.as_bytes(), QUERY_ENCODE_SET).collect::<String>())
        } else {
            None
        }
    }

    pub fn to_url(&self) -> Result<Url, String> {
        if let Some(params) = self.encode_params() {
            let mut url = Url::parse(API).unwrap();
            url.query_pairs_mut()
                .append_pair("wt", &self.format)
                .append_pair("rows", &self.rows.to_string())
                .append_pair("q", &params);
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

    #[test]
    fn search_query() {
        let mut q = SearchQuery::default();
        q.group_id = Some("org.typelevel".into());
        q.artifact_id = Some("cats_2.11".into());
        assert!(q.to_url().is_ok());
    }
}