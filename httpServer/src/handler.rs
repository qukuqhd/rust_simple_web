use http::{
    http_request::{HttpRequest, Resource},
    http_response::HttpResponse,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, fs};

pub trait Handler {
    fn handle(req: &HttpRequest) -> HttpResponse;
    fn load_file(file_name: &str) -> Option<String> {
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", public_path, file_name);
        let contents = fs::read_to_string(full_path);
        contents.ok()
    }
}
pub struct WebServiceHandler;
pub struct StaticPageHandler;
pub struct PageNotFoundHandler;
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderStatus {
    order_id: i32,
    order_status: String,
    order_date: String,
}
impl Handler for PageNotFoundHandler {
    fn handle(_req: &HttpRequest) -> HttpResponse {
        HttpResponse::new("404", None, Self::load_file("404.html"))
    }
}
impl Handler for StaticPageHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        let http::http_request::Resource::Path(s) = &req.resource;
        let route: Vec<_> = s.split("/").collect();
        match route[1] {
            "" => HttpResponse::new("200", None, Self::load_file("index.html")),
            "health" => HttpResponse::new("200", None, Self::load_file("health.html")),
            path => match Self::load_file(path) {
                Some(content) => {
                    let mut map = HashMap::new();
                    if path.ends_with(".css") {
                        map.insert("Content-Type", "text/css");
                    } else if path.ends_with(".js") {
                        map.insert("Content-Type", "text/javascript");
                    } else {
                        map.insert("Content-Type", "text/html");
                    }
                    HttpResponse::new("200", Some(map), Some(content))
                }
                None => PageNotFoundHandler::handle(req),
            },
        }
    }
}
impl WebServiceHandler {
    fn load_json() -> Vec<OrderStatus> {
        let default_path = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
        let data_path = env::var("DATA_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", data_path, "order.json");
        let json_contents = fs::read_to_string(full_path).unwrap();
        serde_json::from_str(json_contents.as_str()).unwrap()
    }
}
impl Handler for WebServiceHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        let Resource::Path(s) = &req.resource;
        let route: Vec<_> = s.split("/").collect();
        match route[2] {
            "shipping" if route.len() > 2 && route[3] == "orders" => {
                let body = Some(serde_json::to_string(&Self::load_json()).unwrap());
                let mut header = HashMap::new();
                header.insert("Content-Type", "application/json");
                HttpResponse::new("200", Some(header), body)
            }
            _ => PageNotFoundHandler::handle(req),
        }
    }
}
