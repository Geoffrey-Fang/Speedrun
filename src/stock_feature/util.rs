use std::{collections::HashMap, error::Error};

use serde::Deserialize;

pub fn fetch_data<T: for<'a> Deserialize<'a>>(url: &str, params: &HashMap<&str, String>) -> Result<T, Box<dyn Error>> {
    let response = ureq::get(url)
        .query("sortColumns", params.get("sortColumns").unwrap())
        .query("sortTypes", params.get("sortTypes").unwrap())
        .query("pageSize", params.get("pageSize").unwrap())
        .query("pageNumber", params.get("pageNumber").unwrap())
        .query("reportName", params.get("reportName").unwrap())
        .query("columns", params.get("columns").unwrap())
        .query("source", params.get("source").unwrap())
        .query("client", params.get("client").unwrap())
        .query("filter", params.get("filter").unwrap())
        .call();
    match response {
        Ok(response) => {
            let body = response.into_string().unwrap();
            println!("{}", body);
            let data: T = serde_json::from_str(body.as_str()).unwrap();
            return Ok(data)
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