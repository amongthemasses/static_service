// use super::impls::http_resp::HttpResp;

#[get("/test-1")]
pub fn index_1() -> &'static str { "Hello, world! _----1" }

#[get("/test-2")]
pub fn index_2() -> &'static str {
    "Hello, world! _----1"
}
