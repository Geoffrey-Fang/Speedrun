use std::{cmp::Reverse, collections::BTreeMap, error::Error, fs::File};
use csv::Writer;
use chrono::{DateTime, Utc};
use serde_json::{self, Value};

/// 处理财联社-电报
///
/// 
/// 
pub fn handleClsTelegraph(map: BTreeMap<Reverse<i64>, Value>, current_time: i64, last_time: i64) {
    let file_path = format!("cls_{}_{}.csv", current_time, last_time);
    let file = File::create(file_path).unwrap();

    let mut writer = Writer::from_writer(file);
    writer.write_record(&["时间", "电报", "消息级别", "个股列表"]).unwrap();
    for (dt, json) in map {
        let stock_list = json.get("stock_list").unwrap().as_array().unwrap();
        let mut names: Vec<String> = Vec::new();
        let mut ids: Vec<String> = Vec::new();
        for s in stock_list {
            names.push(s.get("name").unwrap().to_string());
            ids.push(s.get("StockID").unwrap().to_string());
        }
        let content = json.get("content").unwrap().to_string();
        let level = json.get("level").unwrap().to_string();
        let mut list: String = String::new();
        if names.len() > 0 {
            for i in 0..names.len(){
                list += &format!(" [{} : {}] ", names[i], ids[i]).to_string();
            }
        }
        let mut record = vec![DateTime::<Utc>::from_utc(
            chrono::NaiveDateTime::from_timestamp(dt.0, 0),
            Utc,
        ).with_timezone(&chrono::Local).to_string(), content, level, list];
        writer.write_record(&record).unwrap();
    }
    writer.flush().unwrap();
}


/// 抓取财联社-电报
///
/// 
/// 
pub fn clsTelegraph() -> Result<(BTreeMap<Reverse<i64>, Value>, i64, i64), Box<dyn Error>> {
    let mut url = "https://m.cls.cn/telegraph";
    let mut response = ureq::get(url).call();
    match response {
        Ok(response) => {
            
        }
        Err(ureq::Error::Status(code, response)) => {
            return Err(Box::new(ureq::Error::Status(code, response)));
        }
        Err(e) => {
            eprintln!("Request failed: {}", e);
            return Err(Box::new(e));
        }
    }

    let ts = Utc::now();
    let current_time = ts.timestamp();
    let mut _result: BTreeMap<Reverse<i64>, Value> = BTreeMap::new();
    let mut last_time = current_time;
    while current_time - last_time < 100000 {
        url = "https://m.cls.cn/nodeapi/telegraphs";
        response = ureq::get(url)
            .query("refresh_type", "1")
            .query("rn", "10")
            .query("last_time", last_time.to_string().as_str())
            .query("app", "CailianpressWap")
            .query("sv", "1")
            .call();
        match response {
            Ok(response) => {
                let body = response.into_string().unwrap();
                let data_json: Value = serde_json::from_str(body.as_str()).unwrap();
                let temp_df: Value = data_json.get("data").unwrap().get("roll_data").unwrap().clone();
                for _param_df in temp_df.as_array().unwrap(){
                    let _modified_time = _param_df.get("modified_time").unwrap().clone().as_i64().unwrap();
                    _result.insert(Reverse(_modified_time),_param_df.clone());
                }
                let _update_time = last_time;
                last_time = _result.last_key_value().unwrap().0.0;
                if _update_time == last_time {
                    break;
                }
                println!("last_time is {}", last_time);
            }
            Err(ureq::Error::Status(code, response)) => {
                return Err(Box::new(ureq::Error::Status(code, response)));
            }
            Err(e) => {
                eprintln!("Request failed: {}", e);
                return Err(Box::new(e));
            }
        }
    }
    Ok((_result, current_time, last_time))
}