use std::{collections::BTreeMap, error::Error};
use prettytable::{row, cell, Cell, Row, Table};
use serde_json::Value;
use crate::utils::de::{deserialize_option_f64, deserialize_option_i64};

pub fn getStockIndividualInfo(symbol: String, info: BTreeMap<String, i8>) -> Result<Table, Box<dyn Error>> {
    let url = "http://push2.eastmoney.com/api/qt/stock/get";
    let mut _result = Table::new();
    let mut response = ureq::get(url)
    .query("ut", "fa5fd1943c7b386f172d6893dbfba10b")
    .query("fltt", "2")
    .query("invt", "2")
    .query("fields", "f120,f121,f122,f174,f175,f59,f163,f43,f57,f58,f169,f170,f46,f44,f51,f168,f47,f164,f116,f60,f45,f52,f50,f48,f167,f117,f71,f161,f49,f530,f135,f136,f137,f138,f139,f141,f142,f144,f145,f147,f148,f140,f143,f146,f149,f55,f62,f162,f92,f173,f104,f105,f84,f85,f183,f184,f185,f186,f187,f188,f189,f190,f191,f192,f107,f111,f86,f177,f78,f110,f262,f263,f264,f267,f268,f255,f256,f257,f258,f127,f199,f128,f198,f259,f260,f261,f171,f277,f278,f279,f288,f152,f250,f251,f252,f253,f254,f269,f270,f271,f272,f273,f274,f275,f276,f265,f266,f289,f290,f286,f285,f292,f293,f294,f295")
    .query("secid", &format!("{}.{}", info[&symbol], symbol.trim_matches('"')).to_string())
    .query("_", "1640157544804")
    .call();
    match response {
        Ok(response) => {
            let body = response.into_string().unwrap();
            let data_json: Value = serde_json::from_str(body.as_str()).unwrap();
            let temp_df: Value = data_json.get("data").unwrap().clone();
            let f57 = temp_df.get("f57").unwrap().to_string();
            let f58 = temp_df.get("f58").unwrap().to_string();
            let f84 = match temp_df.get("f84") {
                Some(Value::Number(num)) => {
                    num.as_f64().unwrap()
                },
                Some(Value::String(s)) => {
                    0 as f64
                },
                Some(_) => {
                    0 as f64
                },
                None => {
                    0 as f64
                }
            };
            let f85 = match temp_df.get("f85") {
                Some(Value::Number(num)) => {
                    num.as_f64().unwrap()
                },
                Some(Value::String(s)) => {
                    0 as f64
                },
                Some(_) => {
                    0 as f64
                },
                None => {
                    0 as f64
                }
            };
            let f127 = temp_df.get("f127").unwrap().to_string();
            let f116 = match temp_df.get("f116") {
                Some(Value::Number(num)) => {
                    num.as_f64().unwrap()
                },
                Some(Value::String(s)) => {
                    0 as f64
                },
                Some(_) => {
                    0 as f64
                },
                None => {
                    0 as f64
                }
            };
            let f117 = match temp_df.get("f116") {
                Some(Value::Number(num)) => {
                    num.as_f64().unwrap()
                },
                Some(Value::String(s)) => {
                    0 as f64
                },
                Some(_) => {
                    0 as f64
                },
                None => {
                    0 as f64
                }
            };
            let f189 = match temp_df.get("f189") {
                Some(Value::Number(num)) => {
                    num.as_f64().unwrap()
                },
                Some(Value::String(s)) => {
                    0 as f64
                },
                Some(_) => {
                    0 as f64
                },
                None => {
                    0 as f64
                }
            };
            _result.set_titles(row!["item", "value"]);
            _result.add_row(row!["股票代码", f57]);
            _result.add_row(row!["股票简称", f58]);
            _result.add_row(row!["总股本", f84]);
            _result.add_row(row!["流通股", f85]);
            _result.add_row(row!["行业", f127]);
            _result.add_row(row!["总市值", f116]);
            _result.add_row(row!["流通市值", f117]);
            _result.add_row(row!["上市时间", f189]);
            _result.printstd();
            return Ok(_result);
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