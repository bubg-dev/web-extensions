//! Wrapper for the [`chrome.downloads` API](https://developer.chrome.com/docs/extensions/reference/downloads/).

use std::path::PathBuf;

use serde::{
  Deserialize,
  Serialize,
};
use web_extensions_sys::chrome_downloads;

use crate::{
  util::*,
  Error,
};

/// <https://developer.chrome.com/docs/extensions/reference/downloads/#method-search>
pub async fn search(query: &Query<'_>) -> Result<Vec<DownloadItem>, Error> {
  let js_query = js_from_serde(query)?;
  let js_value = chrome_downloads().search(&js_query).await?;
  serde_from_js(js_value)
}

/// <https://developer.chrome.com/docs/extensions/reference/downloads/#type-DownloadQuery>
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Query<'a> {
  pub query: Option<Vec<&'a str>>,
  pub start_time: Option<&'a str>,
}

impl<'a> From<&'a str> for Query<'a> {
  fn from(q: &'a str) -> Self {
    Self {
      query: Some(vec![q]),
      start_time: None,
    }
  }
}

/// <https://developer.chrome.com/docs/extensions/reference/downloads/#type-DownloadItem>
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadItem {
  pub filename: PathBuf,
  pub mime: String,
  pub start_time: String,
  pub url: String,
}
