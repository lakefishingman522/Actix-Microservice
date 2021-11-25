use prometheus::{labels, opts, register_counter, register_gauge, register_histogram_vec};
use prometheus::{Counter, Gauge, HistogramOpts, HistogramVec};

const ENVS: &'static [&'static str] = &["dev", "production"];

lazy_static! {
  pub static ref HTTP_COUNTER: Counter = register_counter!(opts!(
    "rs_auth_http_requests_total",
    "Number of HTTP requests made.",
    labels! {"handler" => "all",}
  ))
  .unwrap();
  pub static ref HTTP_BODY_GAUGE: Gauge = register_gauge!(opts!(
    "rs_auth_http_response_size_bytes",
    "The HTTP response sizes in bytes.",
    labels! {"handler" => "all",}
  ))
  .unwrap();
  pub static ref HTTP_REQ_HISTOGRAM: HistogramVec = register_histogram_vec!(
    "rs_auth_http_request_duration_seconds",
    "The HTTP request latencies in seconds.",
    &["env"]
  )
  .unwrap();
  pub static ref HTTP_REQ_HISTOGRAM2: HistogramVec = HistogramVec::new(
    HistogramOpts::new("response_time", "Response Times"),
    &["env"]
  )
  .unwrap();
  //pub static ref REGISTRY: Registry = Registry::new();
}

// pub fn register_metrics() {
//   REGISTRY
//     .register(Box::new(HTTP_COUNTER.clone()))
//     .expect("collector can be registered");

//   REGISTRY
//     .register(Box::new(HTTP_BODY_GAUGE.clone()))
//     .expect("collector can be registered");

//   REGISTRY
//     .register(Box::new(HTTP_REQ_HISTOGRAM.clone()))
//     .expect("collector can be registered");
// }
