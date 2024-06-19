use std::error::Error;
use serde::{Deserialize, Serialize};
use serde::de::{self, Deserializer};

#[derive(Serialize, Deserialize, Debug)]
struct RealtimeQuoteData {
    #[serde(rename = "f1")]
    f1: Option<i32>,
    #[serde(rename = "f2")]
    #[serde(deserialize_with = "deserialize_option_f64")]
    f2: Option<f64>,
    #[serde(rename = "f3")]
    #[serde(deserialize_with = "deserialize_option_f64")]
    f3: Option<f64>,
    #[serde(rename = "f4")]
    #[serde(deserialize_with = "deserialize_option_f64")]
    f4: Option<f64>,
    #[serde(rename = "f5")]
    #[serde(deserialize_with = "deserialize_option_i64")]
    f5: Option<i64>,
    #[serde(rename = "f6")]
    #[serde(deserialize_with = "deserialize_option_f64")]
    f6: Option<f64>,
    #[serde(rename = "f7")]
    #[serde(deserialize_with = "deserialize_option_f64")]
    f7: Option<f64>,
    #[serde(rename = "f8")]
    #[serde(deserialize_with = "deserialize_option_f64")]
    f8: Option<f64>,
    #[serde(rename = "f9")]
    #[serde(deserialize_with = "deserialize_option_f64")]
    f9: Option<f64>,
    #[serde(rename = "f10")]
    #[serde(deserialize_with = "deserialize_option_f64")]
    f10: Option<f64>,
    #[serde(rename = "f11")]
    #[serde(deserialize_with = "deserialize_option_f64")]
    f11: Option<f64>,
    #[serde(rename = "f12")]
    f12: Option<String>,
    #[serde(rename = "f13")]
    f13: Option<i32>,
    #[serde(rename = "f14")]
    f14: Option<String>,
    #[serde(rename = "f15")]
    #[serde(deserialize_with = "deserialize_option_f64")]
    f15: Option<f64>,
    #[serde(rename = "f16")]
    #[serde(deserialize_with = "deserialize_option_f64")]
    f16: Option<f64>,
    #[serde(rename = "f17")]
    #[serde(deserialize_with = "deserialize_option_f64")]
    f17: Option<f64>,
    #[serde(rename = "f18")]
    #[serde(deserialize_with = "deserialize_option_f64")]
    f18: Option<f64>,
    #[serde(rename = "f20")]
    #[serde(deserialize_with = "deserialize_option_i64")]
    f20: Option<i64>,
    #[serde(rename = "f21")]
    #[serde(deserialize_with = "deserialize_option_i64")]
    f21: Option<i64>,
    #[serde(rename = "f22")]
    #[serde(deserialize_with = "deserialize_option_f64")]
    f22: Option<f64>,
    #[serde(rename = "f23")]
    #[serde(deserialize_with = "deserialize_option_f64")]
    f23: Option<f64>,
    #[serde(rename = "f24")]
    #[serde(deserialize_with = "deserialize_option_f64")]
    f24: Option<f64>,
    #[serde(rename = "f25")]
    #[serde(deserialize_with = "deserialize_option_f64")]
    f25: Option<f64>,
    #[serde(rename = "f62")]
    #[serde(deserialize_with = "deserialize_option_f64")]
    f62: Option<f64>,
    #[serde(rename = "f115")]
    #[serde(deserialize_with = "deserialize_option_f64")]
    f115: Option<f64>,
    #[serde(rename = "f128")]
    f128: Option<String>,
    #[serde(rename = "f140")]
    f140: Option<String>,
    #[serde(rename = "f141")]
    f141: Option<String>,
    #[serde(rename = "f136")]
    f136: Option<String>,
    #[serde(rename = "f152")]
    f152: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct RealtimeQuoteReturn {
    #[serde(rename = "total")]
    total: i32,
    #[serde(rename = "diff")]
    diff: Vec<RealtimeQuoteData>,
}

#[derive(Serialize, Deserialize, Debug)]
struct RealtimeQuoteResponse {
    #[serde(rename = "rc")]
    rc: i32,
    #[serde(rename = "rt")]
    rt: i32,
    #[serde(rename = "svr")]
    svr: i64,
    #[serde(rename = "lt")]
    lt: i32,
    #[serde(rename = "full")]
    full: i32,
    #[serde(rename = "dlmkts")]
    dlmkts: String,
    #[serde(rename = "data")]
    data: RealtimeQuoteReturn,
}

fn deserialize_option_f64<'de, D>(deserializer: D) -> std::result::Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<serde_json::Value>::deserialize(deserializer)?;
    match opt {
        Some(serde_json::Value::String(s)) if s == "-" => Ok(None),
        Some(serde_json::Value::String(s)) => s.parse::<f64>().map(Some).map_err(de::Error::custom),
        Some(serde_json::Value::Number(num)) => num.as_f64().ok_or_else(|| de::Error::custom("Invalid number")).map(Some),
        Some(_) => Err(de::Error::custom("Invalid type")),
        None => Ok(None),
    }
}

fn deserialize_option_i64<'de, D>(deserializer: D) -> std::result::Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<serde_json::Value>::deserialize(deserializer)?;
    match opt {
        Some(serde_json::Value::String(s)) if s == "-" => Ok(None),
        Some(serde_json::Value::String(s)) => s.parse::<i64>().map(Some).map_err(de::Error::custom),
        Some(serde_json::Value::Number(num)) => num.as_i64().ok_or_else(|| de::Error::custom("Invalid number")).map(Some),
        Some(_) => Err(de::Error::custom("Invalid type")),
        None => Ok(None),
    }
}

pub fn zhSpot() -> Result<(), Box<dyn Error>> {
    let url = "https://82.push2.eastmoney.com/api/qt/clist/get";
    let response = ureq::get(url)
        .query("pn", "1")
        .query("pz", "50000")
        .query("po", "1")
        .query("np", "1")
        .query("ut", "bd1d9ddb04089700cf9c27f6f7426281")
        .query("fltt", "2")
        .query("invt", "2")
        .query("fid", "f3")
        .query("fs", "m:0 t:6,m:0 t:80,m:1 t:2,m:1 t:23,m:0 t:81 s:2048")
        .query("fields", "f1,f2,f3,f4,f5,f6,f7,f8,f9,f10,f12,f13,f14,f15,f16,f17,f18,f20,f21,f23,f24,f25,f22,f11,f62,f128,f136,f115,f152")
        .query("_", "1623833739532")
        .call();
    match response {
        Ok(response) => {
            let body = response.into_string().unwrap();
            Ok(())
        }
        Err(ureq::Error::Status(code, response)) => {
            Err(Box::new(ureq::Error::Status(code, response)))
        }
        Err(e) => {
            eprintln!("Request failed: {}", e);
            Err(Box::new(e))
        }
    }
}

pub fn shSpot() -> Result<(), Box<dyn Error>> {
    let url = "https://82.push2.eastmoney.com/api/qt/clist/get";
    let response = ureq::get(url)
        .query("pn", "1")
        .query("pz", "50000")
        .query("po", "1")
        .query("np", "1")
        .query("ut", "bd1d9ddb04089700cf9c27f6f7426281")
        .query("fltt", "2")
        .query("invt", "2")
        .query("fid", "f3")
        .query("fs", "m:1 t:2,m:1 t:23")
        .query("fields", "f1,f2,f3,f4,f5,f6,f7,f8,f9,f10,f12,f13,f14,f15,f16,f17,f18,f20,f21,f23,f24,f25,f22,f11,f62,f128,f136,f115,f152")
        .query("_", "1623833739532")
        .call();
    match response {
        Ok(response) => {
            let body = response.into_string().unwrap();
            let data : RealtimeQuoteResponse = serde_json::from_str(body.as_str()).unwrap();
            Ok(())
        }
        Err(ureq::Error::Status(code, response)) => {
            Err(Box::new(ureq::Error::Status(code, response)))
        }
        Err(e) => {
            eprintln!("Request failed: {}", e);
            Err(Box::new(e))
        }
    }
}

pub fn szSpot() -> Result<(), Box<dyn Error>> {
    let url = "https://82.push2.eastmoney.com/api/qt/clist/get";
    let response = ureq::get(url)
        .query("pn", "1")
        .query("pz", "50000")
        .query("po", "1")
        .query("np", "1")
        .query("ut", "bd1d9ddb04089700cf9c27f6f7426281")
        .query("fltt", "2")
        .query("invt", "2")
        .query("fid", "f3")
        .query("fs", "m:0 t:6,m:0 t:80")
        .query("fields", "f1,f2,f3,f4,f5,f6,f7,f8,f9,f10,f12,f13,f14,f15,f16,f17,f18,f20,f21,f23,f24,f25,f22,f11,f62,f128,f136,f115,f152")
        .query("_", "1623833739532")
        .call();
    match response {
        Ok(response) => {
            let body = response.into_string().unwrap();
            println!("{}", body);
            let data : RealtimeQuoteResponse = serde_json::from_str(body.as_str()).unwrap();
            Ok(())
        }
        Err(ureq::Error::Status(code, response)) => {
            Err(Box::new(ureq::Error::Status(code, response)))
        }
        Err(e) => {
            eprintln!("Request failed: {}", e);
            Err(Box::new(e))
        }
    }
}

pub fn bjSpot() -> Result<(), Box<dyn Error>> {
    let url = "https://82.push2.eastmoney.com/api/qt/clist/get";
    let response = ureq::get(url)
        .query("pn", "1")
        .query("pz", "50000")
        .query("po", "1")
        .query("np", "1")
        .query("ut", "bd1d9ddb04089700cf9c27f6f7426281")
        .query("fltt", "2")
        .query("invt", "2")
        .query("fid", "f3")
        .query("fs", "m:0 t:81 s:2048")
        .query("fields", "f1,f2,f3,f4,f5,f6,f7,f8,f9,f10,f12,f13,f14,f15,f16,f17,f18,f20,f21,f23,f24,f25,f22,f11,f62,f128,f136,f115,f152")
        .query("_", "1623833739532")
        .call();
    match response {
        Ok(response) => {
            let body = response.into_string().unwrap();
            println!("{}", body);
            let data : RealtimeQuoteResponse = serde_json::from_str(body.as_str()).unwrap();
            Ok(())
        }
        Err(ureq::Error::Status(code, response)) => {
            Err(Box::new(ureq::Error::Status(code, response)))
        }
        Err(e) => {
            eprintln!("Request failed: {}", e);
            Err(Box::new(e))
        }
    }
}

pub fn newSpot() -> Result<(), Box<dyn Error>> {
    let url = "https://82.push2.eastmoney.com/api/qt/clist/get";
    let response = ureq::get(url)
        .query("pn", "1")
        .query("pz", "50000")
        .query("po", "1")
        .query("np", "1")
        .query("ut", "bd1d9ddb04089700cf9c27f6f7426281")
        .query("fltt", "2")
        .query("invt", "2")
        .query("wbp2u", "|0|0|0|web")
        .query("fid", "f26")
        .query("fs", "m:0 f:8,m:1 f:8")
        .query("fields", "f1,f2,f3,f4,f5,f6,f7,f8,f9,f10,f12,f13,f14,f15,f16,f17,f18,f20,f21,f23,f24,f25,f22,f11,f62,f128,f136,f115,f152")
        .query("_", "1623833739532")
        .call();
    match response {
        Ok(response) => {
            let body = response.into_string().unwrap();
            println!("{}", body);
            let data : RealtimeQuoteResponse = serde_json::from_str(body.as_str()).unwrap();
            Ok(())
        }
        Err(ureq::Error::Status(code, response)) => {
            Err(Box::new(ureq::Error::Status(code, response)))
        }
        Err(e) => {
            eprintln!("Request failed: {}", e);
            Err(Box::new(e))
        }
    }
}

pub fn cySpot() -> Result<(), Box<dyn Error>> {
    let url = "https://82.push2.eastmoney.com/api/qt/clist/get";
    let response = ureq::get(url)
        .query("pn", "1")
        .query("pz", "50000")
        .query("po", "1")
        .query("np", "1")
        .query("ut", "bd1d9ddb04089700cf9c27f6f7426281")
        .query("fltt", "2")
        .query("invt", "2")
        .query("wbp2u", "|0|0|0|web")
        .query("fid", "f3")
        .query("fs", "m:0 t:80")
        .query("fields", "f1,f2,f3,f4,f5,f6,f7,f8,f9,f10,f12,f13,f14,f15,f16,f17,f18,f20,f21,f23,f24,f25,f22,f11,f62,f128,f136,f115,f152")
        .query("_", "1623833739532")
        .call();
    match response {
        Ok(response) => {
            let body = response.into_string().unwrap();
            println!("{}", body);
            let data : RealtimeQuoteResponse = serde_json::from_str(body.as_str()).unwrap();
            Ok(())
        }
        Err(ureq::Error::Status(code, response)) => {
            Err(Box::new(ureq::Error::Status(code, response)))
        }
        Err(e) => {
            eprintln!("Request failed: {}", e);
            Err(Box::new(e))
        }
    }
}

pub fn kcSpot() -> Result<(), Box<dyn Error>> {
    let url = "https://82.push2.eastmoney.com/api/qt/clist/get";
    let response = ureq::get(url)
        .query("pn", "1")
        .query("pz", "50000")
        .query("po", "1")
        .query("np", "1")
        .query("ut", "bd1d9ddb04089700cf9c27f6f7426281")
        .query("fltt", "2")
        .query("invt", "2")
        .query("wbp2u", "|0|0|0|web")
        .query("fid", "f3")
        .query("fs", "m:1 t:23")
        .query("fields", "f1,f2,f3,f4,f5,f6,f7,f8,f9,f10,f12,f13,f14,f15,f16,f17,f18,f20,f21,f23,f24,f25,f22,f11,f62,f128,f136,f115,f152")
        .query("_", "1623833739532")
        .call();
    match response {
        Ok(response) => {
            let body = response.into_string().unwrap();
            println!("{}", body);
            let data : RealtimeQuoteResponse = serde_json::from_str(body.as_str()).unwrap();
            Ok(())
        }
        Err(ureq::Error::Status(code, response)) => {
            Err(Box::new(ureq::Error::Status(code, response)))
        }
        Err(e) => {
            eprintln!("Request failed: {}", e);
            Err(Box::new(e))
        }
    }
}