use std::error::Error;
use itertools::Itertools;
use prettytable::{cell, format, row, Table};
use scraper::{Html, Selector};
use serde_json::Value;

// """
// 淘股吧-热门股票
// https://www.taoguba.com.cn/stock/moreHotStock
// :return: 热门股票
// :rtype: pandas.DataFrame
// """
pub fn getHotFollowTaoGuBa() -> Result<Table, Box<dyn Error>> {
    let url = "https://www.taoguba.com.cn/stock/moreHotStock";
    let response = ureq::get(url).call();
    let mut _result = Table::new();
    match response {
        Ok(response) => {
            let body = response.into_string().unwrap();
            let document = Html::parse_document(&body);
            
             // 选择 HTML 表格元素
            let table_selector = Selector::parse("table").unwrap();
            let row_selector = Selector::parse("tr").unwrap();
            let cell_selector = Selector::parse("td[class='td_bor02 td_bor03']").unwrap();
            _result.set_titles(row!["个股代码", "个股名称"]);
            // 遍历表格并提取数据
            for table in document.select(&table_selector) {
                for row in table.select(&row_selector) {
                    let cells: Vec<_> = row.select(&cell_selector).collect();
                    if !cells.is_empty() {
                        _result.add_row(row![cells[0].text().collect::<Vec<_>>().concat(), cells[1].text().collect::<Vec<_>>().concat()]);
                    }
                }
            }
            _result.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
            Ok(_result)
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