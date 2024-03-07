#![allow(non_snake_case)]
use marine_rs_sdk::marine;
// use std::str::Split;

static LENGTH:usize = 4;
static OFFSET:usize = 2;

fn hex_string_to_int(h_str: &str) -> Result<u64, &'static str> {
    let res = u64::from_str_radix(h_str, 16).unwrap();
    Ok(res)
}



fn get_index(key: u64, sorted_vals: Vec<u64>) -> Result<u32, &'static str> {
    let idx = sorted_vals.iter().position(|&k| k == key).unwrap();
    Ok(idx as u32)
}

fn rank_from_hex(key: &str, vals: Vec<&str>) -> Result<u32, &'static str> {

    let mut dupes = true;
    let mut cmp_len = LENGTH;
    let mut _vals: Vec<u64> = Vec::new();
    let mut ref_key:u64 = 0u64;

    while dupes {
        _vals.clear();

        ref_key = hex_string_to_int(&key[2..cmp_len+OFFSET]).unwrap();

        for v in vals.iter() {
            let _key = hex_string_to_int(&v[2..cmp_len+OFFSET]).unwrap();
            _vals.push(_key);
        }

        let n = _vals.len();

        _vals.dedup();

        if n == _vals.len() {
            dupes = false;
        }
        else {
            cmp_len += 1;
        }
    }
    _vals.sort();
    _vals.reverse();
    let idx:u32 = get_index(ref_key, _vals).unwrap();

    Ok(idx)

}

fn rank_from_peerid(key: &str, vals: Vec<&str>) -> Result<u32, &'static str> {

    let mut dupes = true;
    let mut cmp_len = LENGTH;
    let mut _vals: Vec<u64> = Vec::new();
    let mut ref_key:u64 = 0u64;
    let n = key.len();

    while dupes {
        _vals.clear();

        ref_key = hex_string_to_int(&key[n-cmp_len..n]).unwrap();

        for v in vals.iter() {
            let _key = hex_string_to_int(&v[n-cmp_len..n]).unwrap();
            _vals.push(_key);
        }

        let vals_n = _vals.len();
        _vals.dedup();

        if vals_n == _vals.len() {
            dupes = false;
        }
        else {
            // todo: guard that
            cmp_len += 1;
        }
    }
    _vals.sort();
    _vals.reverse();
    let idx:u32 = get_index(ref_key, _vals).unwrap();

    Ok(idx)

}



fn main() {}

#[marine]
pub struct IdxResult {
    pub idx:i32,
    pub err: bool,
    pub stderr: String,
} 


#[marine]
pub fn indexer(key: String, vals: Vec<String>, hex:bool) -> IdxResult {
    if hex {
        let res = match rank_from_hex(key.as_str(), vals.iter().map(|s| s.as_str()).collect()) {
            Ok(res) => IdxResult {idx: res as i32, err: false, stderr: "".to_owned()},
            Err(e) => IdxResult {idx: -1 , err: true, stderr: format!("{}", e)},
        };
        return res;
    }
    let res = match rank_from_peerid(key.as_str(), vals.iter().map(|s| s.as_str()).collect()) {
        Ok(res) => IdxResult {idx: res as i32, err: false, stderr: "".to_owned()},
        Err(e) => IdxResult {idx: -1 , err: true, stderr: format!("{}", e)},
    };
    res

}

#[cfg(test)]
mod tests {
    use marine_rs_sdk_test::marine_test;

    #[marine_test(config_path = "../../../../.fluence/tmp/Config.toml")]
    fn test_rank_from_hex(elector: marine_test_env::leader_election::ModuleInterface) {
        
        let ids = vec!["0x0de95abc1".to_string(), "0x0acf00b09".to_string(), "0x01b148ced".to_string()]; // 3734350785, 3561; 2901412617, 2767; 454331629, 433
        let ref_key = "0x0de95abc1".to_string(); //
        let idx_res = elector.indexer(ref_key, ids.clone(), true);
        assert!(idx_res.idx==0); // leader

        let ref_key = "0x01b148ced".to_string();
        let idx_res = elector.indexer(ref_key, ids.clone(), true);
        assert!(idx_res.idx==2); // not leader

        let ids = vec!["0x0de95abc1".to_string(), "0x0de96abc1".to_string(), "0x0de94abc1".to_string()];
        let ref_key = "0x0de95abc1".to_string();
        let idx_res = elector.indexer(ref_key, ids.clone(), true);
        assert!(idx_res.idx==1); // not leader, with dedupe

        
        let ref_key = "0x0de96abc1".to_string();
        let idx_res = elector.indexer(ref_key, ids.clone(), true);
        assert!(idx_res.idx==0);  //leader with dedupe
    }

    #[marine_test(config_path = "../../../../.fluence/tmp/Config.toml")]
    fn test_rank_from_peerid(elector: marine_test_env::leader_election::ModuleInterface) {
        
        let ids = vec!["0x0de95abc1".to_string(), "0x0de95abc2".to_string(), "0x0de95abc3".to_string()]; // 3734350785, 3561; 2901412617, 2767; 454331629, 433
        let ref_key = "0x0de95abc3".to_string();
        let idx_res = elector.indexer(ref_key, ids.clone(), false);
        //assert!(idx_res.idx==0); // leader

        let ref_key = "0x0de95abc1".to_string();
        let idx_res = elector.indexer(ref_key, ids.clone(), false);
        //assert!(idx_res.idx==2); // not leader

        let ids = vec!["0x0de95abc1".to_string(), "0x0de96abc1".to_string(), "0x0de94abc1".to_string()];
        let ref_key = "0x0de95abc1".to_string();
        let idx_res = elector.indexer(ref_key, ids.clone(), false);
        //assert!(idx_res.idx==1); // not leader, with dedupe

        
        let ref_key = "0x0de96abc1".to_string();
        let idx_res = elector.indexer(ref_key, ids.clone(), false);
        //assert!(idx_res.idx==0);  //leader with dedupe
    }
}
