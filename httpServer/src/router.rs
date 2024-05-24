use std::io::Write;

use http::http_response::HttpResponse;

use crate::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};

pub struct Router;
impl Router {
    pub fn route<T: Write>(req: http::http_request::HttpRequest, stream: &mut T) {
        match req.method {
            http::http_request::Method::GET => match req.resource {
                http::http_request::Resource::Path(ref s) => {
                    let route: Vec<&str> = s.split("/").collect();
                    match route[1] {
                        "api" => {
                            let resp: HttpResponse = WebServiceHandler::handle(&req);
                            resp.send_response(stream).unwrap();
                        }
                        _ => {
                            let resp: HttpResponse = StaticPageHandler::handle(&req);
                            resp.send_response(stream).unwrap();
                        }
                    }
                }
            },
            _ => {
                let resp: HttpResponse = PageNotFoundHandler::handle(&req);
                resp.send_response(stream).unwrap();
            }
        }
    }
}
