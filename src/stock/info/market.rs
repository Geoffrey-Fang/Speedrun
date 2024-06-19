use std::{collections::BTreeMap, error::Error};

use serde::de::value::IsizeDeserializer;
use serde_json::Value;

/// 股票和市场代码
/// 
/// 
/// 
pub fn getStockIDAndMarketID() -> Result<BTreeMap<String, i8>, Box<dyn Error>> {
    let mut _result: BTreeMap<String, i8> = BTreeMap::new();
    let url = "https://80.push2.eastmoney.com/api/qt/clist/get";
    let mut response = ureq::get(url)
    .query("pn", "1")
    .query("pz", "50000")
    .query("po", "1")
    .query("np", "1")
    .query("ut", "bd1d9ddb04089700cf9c27f6f7426281")
    .query("fltt", "2")
    .query("invt", "2")
    .query("fid", "f3")
    .query("fs", "m:1 t:2,m:1 t:23")
    .query("fields", "f12")
    .query("_", "1623833739532")
    .call();
    match response {
        Ok(response) => {
            let body = response.into_string().unwrap();
            let data_json: Value = serde_json::from_str(body.as_str()).unwrap();
            let temp_df: Value = data_json.get("data").unwrap().get("diff").unwrap().clone();
            for _param_df in temp_df.as_array().unwrap(){
                _result.insert(_param_df.get("f12").unwrap().to_string(), 1);
            }
        }
        Err(ureq::Error::Status(code, response)) => {
            return Err(Box::new(ureq::Error::Status(code, response)));
        }
        Err(e) => {
            eprintln!("Request failed: {}", e);
            return Err(Box::new(e));
        }
    }

    response = ureq::get(url)
    .query("pn", "1")
    .query("pz", "50000")
    .query("po", "1")
    .query("np", "1")
    .query("ut", "bd1d9ddb04089700cf9c27f6f7426281")
    .query("fltt", "2")
    .query("invt", "2")
    .query("fid", "f3")
    .query("fs", "m:0 t:6,m:0 t:80")
    .query("fields", "f12")
    .query("_", "1623833739532")
    .call();
    match response {
        Ok(response) => {
            let body = response.into_string().unwrap();
            let data_json: Value = serde_json::from_str(body.as_str()).unwrap();
            let temp_df: Value = data_json.get("data").unwrap().get("diff").unwrap().clone();
            for _param_df in temp_df.as_array().unwrap(){
                _result.insert(_param_df.get("f12").unwrap().to_string(), 0);
            }
        }
        Err(ureq::Error::Status(code, response)) => {
            return Err(Box::new(ureq::Error::Status(code, response)));
        }
        Err(e) => {
            eprintln!("Request failed: {}", e);
            return Err(Box::new(e));
        }
    }

    response = ureq::get(url)
    .query("pn", "1")
    .query("pz", "50000")
    .query("po", "1")
    .query("np", "1")
    .query("ut", "bd1d9ddb04089700cf9c27f6f7426281")
    .query("fltt", "2")
    .query("invt", "2")
    .query("fid", "f3")
    .query("fs", "m:0 t:81 s:2048")
    .query("fields", "f12")
    .query("_", "1623833739532")
    .call();
    match response {
        Ok(response) => {
            let body = response.into_string().unwrap();
            let data_json: Value = serde_json::from_str(body.as_str()).unwrap();
            let temp_df: Value = data_json.get("data").unwrap().get("diff").unwrap().clone();
            for _param_df in temp_df.as_array().unwrap(){
                _result.insert(_param_df.get("f12").unwrap().to_string(), 0);
            }
        }
        Err(ureq::Error::Status(code, response)) => {
            return Err(Box::new(ureq::Error::Status(code, response)));
        }
        Err(e) => {
            eprintln!("Request failed: {}", e);
            return Err(Box::new(e));
        }
    }

    Ok(_result)
}