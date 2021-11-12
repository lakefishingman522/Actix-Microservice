use actix_web::{web, HttpResponse};
use prometheus::{
  HistogramOpts, HistogramVec, IntCounter, IntCounterVec, IntGauge, Opts, Registry,
};

use crate::error::CustomError;
use crate::metrics as metric;

pub async fn metrics() -> Result<HttpResponse, CustomError> {
  use prometheus::Encoder;
  let encoder = prometheus::TextEncoder::new();

  let mut buffer = Vec::new();
  let metric_families = prometheus::gather();

  if let Err(e) = encoder.encode(&metric_families, &mut buffer) {
    //if let Err(e) = encoder.encode(&metric::REGISTRY.gather(), &mut buffer) {
    eprintln!("could not encode custom metrics: {}", e);
  };
  let mut res = match String::from_utf8(buffer.clone()) {
    Ok(v) => v,
    Err(e) => {
      eprintln!("custom metrics could not be from_utf8'd: {}", e);
      String::default()
    }
  };
  buffer.clear();

  let mut buffer = Vec::new();
  if let Err(e) = encoder.encode(&prometheus::gather(), &mut buffer) {
    eprintln!("could not encode prometheus metrics: {}", e);
  };
  let res_custom = match String::from_utf8(buffer.clone()) {
    Ok(v) => v,
    Err(e) => {
      eprintln!("prometheus metrics could not be from_utf8'd: {}", e);
      String::default()
    }
  };
  buffer.clear();
  res.push_str(&res_custom);

  Ok(HttpResponse::Ok().body(res))
}
