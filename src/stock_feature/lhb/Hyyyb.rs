use std::{collections::HashMap, error::Error};

use csv::Writer;
use serde::Deserialize;

use crate::stock_feature::util::fetch_data;

#[derive(Debug, Deserialize)]
struct LhbHyyybCrawlResponse {
    result: Option<LhbHyyybCrawlReturn>,
}

#[derive(Debug, Deserialize)]
struct LhbHyyybData {
    #[serde(rename = "OPERATEDEPT_NAME")]
    operate_dept_name: Option<String>,
    #[serde(rename = "ONLIST_DATE")]
    onlist_date: Option<String>,
    #[serde(rename = "BUYER_APPEAR_NUM")]
    buyer_appear_num: Option<u32>,
    #[serde(rename = "SELLER_APPEAR_NUM")]
    seller_appear_num: Option<u32>,
    #[serde(rename = "TOTAL_BUYAMT")]
    total_buy_amt: Option<f64>,
    #[serde(rename = "TOTAL_SELLAMT")]
    total_sell_amt: Option<f64>,
    #[serde(rename = "TOTAL_NETAMT")]
    total_net_amt: Option<f64>,
    #[serde(rename = "BUY_STOCK")]
    buy_stock: Option<String>,
    #[serde(rename = "OPERATEDEPT_CODE")]
    operate_dept_code: Option<String>,
    #[serde(rename = "SECURITY_NAME_ABBR")]
    security_name_abbr: Option<String>,
    #[serde(rename = "OPERATEDEPT_CODE_OLD")]
    operate_dept_code_old: Option<String>,
    #[serde(rename = "ORG_NAME_ABBR")]
    org_name_abbr: Option<String>,
}

#[derive(Debug, Deserialize)]
struct LhbHyyybCrawlReturn {
    pages: usize,
    data: Vec<LhbHyyybData>,
}

pub fn getStockLhbHyyybEm() -> Result<String, Box<dyn Error>>  {
    let url = "https://datacenter-web.eastmoney.com/api/data/v1/get";
    let start_date = "2023-01-01".to_string(); // 示例开始日期
    let end_date = "2023-12-31".to_string(); // 示例结束日期

    let mut params = HashMap::new();
    params.insert("sortColumns", "TOTAL_NETAMT,ONLIST_DATE,OPERATEDEPT_CODE".to_string());
    params.insert("sortTypes", "-1,-1,1".to_string());
    params.insert("pageSize", "5000".to_string());
    params.insert("pageNumber", "1".to_string());
    params.insert("reportName", "RPT_OPERATEDEPT_ACTIVE".to_string());
    params.insert("columns", "ALL".to_string());
    params.insert("source", "WEB".to_string());
    params.insert("client", "WEB".to_string());
    params.insert("filter", format!("(ONLIST_DATE>='{}')(ONLIST_DATE<='{}')", start_date, end_date));

    let initial_response: LhbHyyybCrawlResponse = fetch_data(url, &params).unwrap();
    let total_pages = initial_response.result.as_ref().map_or(1, |r| r.pages);

    let mut all_data = vec![];

    for page in 1..=total_pages {
        params.insert("pageNumber", page.to_string());
        let response: LhbHyyybCrawlResponse = fetch_data(url, &params).unwrap();
        if let Some(result) = response.result {
            all_data.extend(result.data);
        }
    }

    let mut wtr = Writer::from_writer(vec![]);
    wtr.write_record(&["序号", "营业部名称", "上榜日", "买入个股数", "卖出个股数", "买入总金额", "卖出总金额", "总买卖净额", "买入股票"]).unwrap();

    for (index, row) in all_data.iter().enumerate() {
        let mut record = vec![index.to_string()];
        record.push(<std::option::Option<std::string::String> as Clone>::clone(&row.operate_dept_name).unwrap().clone().as_str().to_string());
        record.push(<std::option::Option<std::string::String> as Clone>::clone(&row.onlist_date).unwrap().as_str().to_string());
        record.push(match row.buyer_appear_num {
            Some(e) => e.to_string(),
            None => String::new(),
        });
        record.push(match row.seller_appear_num {
            Some(e) => e.to_string(),
            None => String::new(),
        });
        record.push(match row.total_buy_amt {
            Some(e) => e.to_string(),
            None => String::new(),
        });
        record.push(match row.total_sell_amt {
            Some(e) => e.to_string(),
            None => String::new(),
        });
        record.push(match row.total_net_amt {
            Some(e) => e.to_string(),
            None => String::new(),
        });
        record.push(<std::option::Option<std::string::String> as Clone>::clone(&row.buy_stock).unwrap().as_str().to_string());
        wtr.write_record(&record).unwrap();
    }

    wtr.flush().unwrap();
    let csv_data = String::from_utf8(wtr.into_inner().unwrap()).unwrap();
    return Ok(csv_data)
}