use gloo_utils::format::JsValueSerdeExt;
use js_sys::Object;
use wasm_bindgen::{
  prelude::*,
  JsStatic,
};
use web_extensions_sys as sys;
use web_extensions_sys::Browser;

use crate::error::Error;

pub(crate) fn js_from_serde<T: serde::Serialize>(v: &T) -> Result<JsValue, Error> {
  JsValue::from_serde(v).map_err(Error::JsonSerialization)
}

pub(crate) fn object_from_js(v: &JsValue) -> Result<&Object, Error> {
  Object::try_from(v).ok_or(Error::ObjectConversion)
}

pub(crate) fn serde_from_js_result<T>(v: Result<JsValue, JsValue>) -> Result<T, Error>
where
  T: for<'a> serde::Deserialize<'a>,
{
  v?.into_serde().map_err(Error::JsonDeserialization)
}

pub(crate) fn serde_from_js<T>(v: JsValue) -> Result<T, Error>
where
  T: for<'a> serde::Deserialize<'a>,
{
  v.into_serde().map_err(Error::JsonDeserialization)
}

#[cfg(not(feature = "firefox"))]
pub(crate) fn chrome() -> &'static JsStatic<Browser> {
  sys::chrome()
}

#[cfg(feature = "firefox")]
pub(crate) fn chrome() -> &'static JsStatic<Browser> {
  sys::browser()
}
