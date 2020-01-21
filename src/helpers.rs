extern crate reqwest;
extern crate serde_json;

use serde_json::Value;

///
/// ### Convenience function for getting JSON from return string via `serde`

pub fn get_json(reqwest_res: reqwest::blocking::Response) -> Result<Value, Box<dyn std::error::Error>> {
    reqwest_res.json().map_err(|e| e.into())
}
