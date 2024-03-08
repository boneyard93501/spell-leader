// use std::path::Path;
use marine_rs_sdk::{marine, MountedBinaryResult};
use marine_rs_sdk::module_manifest;

module_manifest!();

pub fn main() {}


#[marine]
pub fn strip_esc(s: String) -> String {
    s.replace("\"", "")
}

#[marine]
pub fn url_test(url:String) -> MountedBinaryResult {
    curl_request(vec![url])
}


#[marine]
pub struct IdxResult {
    pub idx:i32,
    pub err: bool,
    pub stderr: String,
}

#[marine]
pub fn leader_idx(key: String, vals: Vec<String>, hex:bool) -> IdxResult {
    indexer(key, vals, hex)
}

#[marine]
#[module_import ("leader_election")]
extern "C" {
    pub fn indexer(key: String, vals: Vec<String>, hex:bool) -> IdxResult;
}

#[marine]
#[module_import ("curl_effector")]
extern "C" {
    pub fn curl_request(cmd: Vec<String>) -> MountedBinaryResult;
}