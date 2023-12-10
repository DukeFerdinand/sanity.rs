extern crate reqwest;
extern crate serde_json;
use serde_json::Value;

///
/// ### Convenience function for getting JSON from return string via `serde`

pub fn get_json(
    reqwest_res: reqwest::blocking::Response,
) -> Result<Value, Box<dyn std::error::Error>> {
    let data: Value = serde_json::from_str(&reqwest_res.text()?)?;

    Ok(data)
}
