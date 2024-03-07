// use std::path::Path;
use marine_rs_sdk::{marine, MountedBinaryResult};
use marine_rs_sdk::module_manifest;

module_manifest!();

pub fn main() {}


#[marine]
pub fn strip_esc(s: String) -> String {
    let s = s.replace("\"", "");
    s
}

#[marine]
pub fn url_test(url:String) -> MountedBinaryResult {
    curl_request(vec![url])
}

#[marine]
#[module_import ("curl_effector")]
extern "C" {
    pub fn curl_request(cmd: Vec<String>) -> MountedBinaryResult;
}