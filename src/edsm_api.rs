use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

use anyhow::{Context, Error};
use reqwest::blocking::get;
use serde::Deserialize;
use serde_json::from_str;

#[derive(Debug)]
pub struct API {
    first: bool,
    memo: HashMap<String, bool>,
}

impl API {
    pub fn new() -> API {
        API {
            first: true,
            memo: HashMap::new(),
        }
    }

    pub fn check_moved(&mut self, system_name: &str) -> Result<bool, Error> {
        if let Some(v) = self.memo.get(system_name) {
            return Ok(*v);
        }

        println!("星系 {} の情報を取得中...", system_name);
        if self.first {
            self.first = false;
        } else {
            sleep(Duration::from_secs(8));
        }

        let escaped_name = system_name.replace(' ', "+");
        let url = format!(
            "https://www.edsm.net/api-v1/system?showId=1&includeHidden=1&systemName={}",
            escaped_name
        );
        let resp = get(&url)
            .with_context(|| format!("EDSMへの照会に失敗しました。 {}", system_name))?
            .text()?;

        if resp == "{}" || resp == "[]" {
            self.memo.insert(system_name.to_owned(), false);
            return Ok(false);
        }

        let info: Info = from_str(&resp).context("EDSMからのレスポンスのパースに失敗しました。")?;
        let moved = info.merged_to.is_some();
        self.memo.insert(system_name.to_owned(), moved);
        Ok(moved)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Info {
    merged_to: Option<u64>,
}
