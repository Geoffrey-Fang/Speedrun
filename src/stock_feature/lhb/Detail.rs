use std::{collections::HashMap, error::Error};

use csv::Writer;
use serde::Deserialize;

use crate::stock_feature::util::fetch_data;

#[derive(Debug, Deserialize)]
struct LhbDetailCrawlResponse {
    result: Option<LhbDetailCrawlReturn>,
}

#[derive(Debug, Deserialize)]
struct LhbDetailCrawlReturn {
    pages: usize,
    data: Vec<LhbDetailData>,
}

#[derive(Debug, Deserialize)]
struct LhbDetailData {
    #[serde(rename = "SECURITY_CODE")]
    security_code: Option<String>,
    #[serde(rename = "SECUCODE")]
    secucode: Option<String>,
    #[serde(rename = "SECURITY_NAME_ABBR")]
    security_name_abbr: Option<String>,
    #[serde(rename = "TRADE_DATE")]
    trade_date: Option<String>,
    #[serde(rename = "EXPLAIN")]
    explain: Option<String>,
    #[serde(rename = "CLOSE_PRICE")]
    close_price: Option<f64>,
    #[serde(rename = "CHANGE_RATE")]
    change_rate: Option<f64>,
    #[serde(rename = "BILLBOARD_NET_AMT")]
    billboard_net_amt: Option<f64>,
    #[serde(rename = "BILLBOARD_BUY_AMT")]
    billboard_buy_amt: Option<f64>,
    #[serde(rename = "BILLBOARD_SELL_AMT")]
    billboard_sell_amt: Option<f64>,
    #[serde(rename = "BILLBOARD_DEAL_AMT")]
    billboard_deal_amt: Option<f64>,
    #[serde(rename = "ACCUM_AMOUNT")]
    accum_amount: Option<f64>,
    #[serde(rename = "DEAL_NET_RATIO")]
    deal_net_ratio: Option<f64>,
    #[serde(rename = "DEAL_AMOUNT_RATIO")]
    deal_amount_ratio: Option<f64>,
    #[serde(rename = "TURNOVERRATE")]
    turnoverrate: Option<f64>,
    #[serde(rename = "FREE_MARKET_CAP")]
    free_market_cap: Option<f64>,
    #[serde(rename = "EXPLANATION")]
    explanation: Option<String>,
    #[serde(rename = "D1_CLOSE_ADJCHRATE")]
    d1_close_adjchrate: Option<f64>,
    #[serde(rename = "D2_CLOSE_ADJCHRATE")]
    d2_close_adjchrate: Option<f64>,
    #[serde(rename = "D5_CLOSE_ADJCHRATE")]
    d5_close_adjchrate: Option<f64>,
    #[serde(rename = "D10_CLOSE_ADJCHRATE")]
    d10_close_adjchrate: Option<f64>,
    #[serde(rename = "SECURITY_TYPE_CODE")]
    security_type_code: Option<String>,
}

pub fn getStockLhbDetailEm() -> Result<String, Box<dyn Error>> {
    let url = "https://datacenter-web.eastmoney.com/api/data/v1/get";
    let start_date = "2024-06-05".to_string(); // 示例开始日期
    let end_date = "2024-06-05".to_string(); // 示例结束日期

    let mut params = HashMap::new();
    params.insert("sortColumns", "SECURITY_CODE,TRADE_DATE".to_string());
    params.insert("sortTypes", "1,-1".to_string());
    params.insert("pageSize", "5000".to_string());
    params.insert("pageNumber", "1".to_string());
    params.insert("reportName", "RPT_DAILYBILLBOARD_DETAILSNEW".to_string());
    params.insert("columns", "SECURITY_CODE,SECUCODE,SECURITY_NAME_ABBR,TRADE_DATE,EXPLAIN,CLOSE_PRICE,CHANGE_RATE,BILLBOARD_NET_AMT,BILLBOARD_BUY_AMT,BILLBOARD_SELL_AMT,BILLBOARD_DEAL_AMT,ACCUM_AMOUNT,DEAL_NET_RATIO,DEAL_AMOUNT_RATIO,TURNOVERRATE,FREE_MARKET_CAP,EXPLANATION,D1_CLOSE_ADJCHRATE,D2_CLOSE_ADJCHRATE,D5_CLOSE_ADJCHRATE,D10_CLOSE_ADJCHRATE,SECURITY_TYPE_CODE".to_string());
    params.insert("source", "WEB".to_string());
    params.insert("client", "WEB".to_string());
    params.insert("filter", format!("(TRADE_DATE<='{}')(TRADE_DATE>='{}')", end_date, start_date));

    let initial_response: LhbDetailCrawlResponse = fetch_data(url, &params).unwrap();
    let total_pages = initial_response.result.as_ref().map_or(1, |r| r.pages);

    let mut all_data = vec![];

    for page in 1..=total_pages {
        params.insert("pageNumber", page.to_string());
        let response: LhbDetailCrawlResponse = fetch_data(url, &params).unwrap();
        if let Some(result) = response.result {
            all_data.extend(result.data);
        }
    }

    let mut wtr = Writer::from_writer(vec![]);
    wtr.write_record(&["序号", "代码", "-", "名称", "上榜日", "解读", "收盘价", "涨跌幅", "龙虎榜净买额", "龙虎榜买入额", "龙虎榜卖出额",
     "龙虎榜成交额", "市场总成交额", "市场总成交额", "净买额占总成交比", "成交额占总成交比", "换手率", "流通市值", "上榜原因", "上榜后1日", 
     "上榜后2日", "上榜后5日", "上榜后10日",]).unwrap();

    for (index, row) in all_data.iter().enumerate() {
        let mut record = vec![index.to_string()];
        
        record.push(match <std::option::Option<std::string::String> as Clone>::clone(&row.security_code) {
            Some(e) => e.as_str().to_string(),
            None => String::new(),
        });
        record.push(match <std::option::Option<std::string::String> as Clone>::clone(&row.secucode) {
            Some(e) => e.as_str().to_string(),
            None => String::new(),
        });
        record.push(match <std::option::Option<std::string::String> as Clone>::clone(&row.security_name_abbr) {
            Some(e) => e.as_str().to_string(),
            None => String::new(),
        });
        record.push(match <std::option::Option<std::string::String> as Clone>::clone(&row.trade_date) {
            Some(e) => e.as_str().to_string(),
            None => String::new(),
        });
        record.push(match <std::option::Option<std::string::String> as Clone>::clone(&row.explain) {
            Some(e) => e.as_str().to_string(),
            None => String::new(),
        });
        record.push(match row.close_price {
            Some(e) => e.to_string(),
            None => String::new(),
        });
        record.push(match row.change_rate {
            Some(e) => e.to_string(),
            None => String::new(),
        });
        record.push(match row.billboard_net_amt {
            Some(e) => e.to_string(),
            None => String::new(),
        });
        record.push(match row.billboard_buy_amt {
            Some(e) => e.to_string(),
            None => String::new(),
        });
        record.push(match row.billboard_sell_amt {
            Some(e) => e.to_string(),
            None => String::new(),
        });
        record.push(match row.billboard_deal_amt {
            Some(e) => e.to_string(),
            None => String::new(),
        });
        record.push(match row.accum_amount {
            Some(e) => e.to_string(),
            None => String::new(),
        });
        record.push(match row.deal_net_ratio {
            Some(e) => e.to_string(),
            None => String::new(),
        });
        record.push(match row.deal_amount_ratio {
            Some(e) => e.to_string(),
            None => String::new(),
        });
        record.push(match row.turnoverrate {
            Some(e) => e.to_string(),
            None => String::new(),
        });
        record.push(match row.free_market_cap {
            Some(e) => e.to_string(),
            None => String::new(),
        });
        record.push(match <std::option::Option<std::string::String> as Clone>::clone(&row.explanation)  {
            Some(e) => e.as_str().to_string(),
            None => String::new(),
        });
        record.push(match row.d1_close_adjchrate {
            Some(e) => e.to_string(),
            None => String::new(),
        });
        record.push(match row.d2_close_adjchrate {
            Some(e) => e.to_string(),
            None => String::new(),
        });
        record.push(match row.d5_close_adjchrate {
            Some(e) => e.to_string(),
            None => String::new(),
        });
        record.push(match row.d10_close_adjchrate {
            Some(e) => e.to_string(),
            None => String::new(),
        });
        record.push(match <std::option::Option<std::string::String> as Clone>::clone(&row.security_type_code)  {
            Some(e) => e.as_str().to_string(),
            None => String::new(),
        });
        wtr.write_record(&record).unwrap();
    }

    wtr.flush().unwrap();
    let csv_data = String::from_utf8(wtr.into_inner().unwrap()).unwrap();
    return Ok(csv_data)
}