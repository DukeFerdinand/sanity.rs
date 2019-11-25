#![crate_name = "sanity"]

extern crate reqwest;
extern crate serde;
extern crate serde_json;

pub mod helpers;

use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct SanityConfig {
    project_id: String,
    access_token: String,
    data_set: String,
    url: String,
    pub query: Query,
}

pub fn construct_headers(auth_token: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    let auth_header_val = HeaderValue::from_str(auth_token);
    if auth_header_val.is_ok() {
        headers.insert("Authorization", auth_header_val.unwrap());
    } else {
        panic!("Invalid header. {}", auth_header_val.unwrap_err());
    }
    headers
}

pub fn get_url(project_id: &str, data_set: &str) -> String {
    format!(
        "https://{}.api.sanity.io/v1/data/query/{}",
        project_id, data_set
    )
}

/// # Initialize the Sanity config
/// ---
///
/// Here we take in the api key as well as data set you'll use, and return the convenience config
///
/// ### Example
/// ```
/// extern crate sanity;
/// use sanity::create;
///
/// fn main() {
///      // Returns SanityConfig
///     let config = create("project_string", "development");
/// }
/// ```
///
///
pub fn create(project_id: &str, data_set: &str, token: &str, use_prod: bool) -> SanityConfig {
    SanityConfig {
        project_id: project_id.to_string(),
        access_token: token.to_string(),
        data_set: data_set.to_string(),
        url: get_url(project_id, data_set),
        query: Query {
            base_url: if use_prod {
                format!(
                    "https://{}.apicdn.sanity.io/v1/data/query/{}/",
                    project_id, data_set
                )
            } else {
                format!(
                    "https://{}.api.sanity.io/v1/data/query/{}/",
                    project_id, data_set
                )
            },
            query: None,
        },
    }
}

#[derive(Debug, Clone)]
pub struct Query {
    base_url: String,
    pub query: Option<String>,
}

impl Query {
    pub fn execute(&self) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{}?query={}", self.base_url, self.query.as_ref().unwrap());
        let mut res: _ = reqwest::get(&url)?;
        let data: Value = serde_json::from_str(&res.text()?)?;

        Ok(data)
    }
}

impl SanityConfig {
    pub fn build_url(&mut self, query: Option<&str>) -> String {
        match query {
            Some(query) => format!("{}?query={}", self.query.base_url, query),
            None => format!(
                "{}?query={}",
                self.query.base_url,
                self.query.query.as_ref().unwrap()
            ),
        }
    }
    /// ## Convenience wrapper for get request
    /// ---
    /// Takes in arg `query: &str`, which expects GROQ query language (Something like this: `*[_type == 'recipe']`)
    ///
    /// ### Please note: There is NO syntax checking for GROQ query language at this time, but a macro is in the works
    /// See [the sanity docs](https://www.sanity.io/docs/overview-groq) for more
    ///
    /// ### Example usage:
    /// ```
    /// extern crate sanity;
    /// use sanity::helpers::get_json;
    ///
    /// fn main() {
    ///   let mut sn = sanity::create(
    ///     "proj_id",
    ///     "data_set",  // i.e. "development"
    ///     "Long_string_for_token",          // Bearer token
    ///   );
    ///   let res = sn.get(&String::from("*[_type == 'recipe']"));
    ///   if res.is_ok() {
    ///     println!("{:?}", get_json(res.unwrap()));
    ///   }
    /// }
    /// ```
    pub fn get(&mut self, query: &str) -> Result<reqwest::Response, reqwest::Error> {
        let client = Client::new();
        let url = self.build_url(Some(query));
        let res = client.get(&url).bearer_auth(&self.access_token).send();
        res
    }
}
