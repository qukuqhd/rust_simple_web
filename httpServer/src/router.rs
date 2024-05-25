use std::{ cell::RefCell, collections::HashMap, io::Write};

use http::{http_request::{HttpRequest, Method, Resource}, http_response::HttpResponse};

use crate::handler::{Handler, PageNotFoundHandler};
struct HandlerMap(HashMap<String, fn(&HttpRequest) -> HttpResponse>);
impl HandlerMap{
    pub fn new()->Self{
        Self(HashMap::new())
    }
    pub fn add(&mut self,path:String,handler_func:fn(&HttpRequest) -> HttpResponse){
        self.0.insert(path, handler_func);
    }
    pub fn execute_handler<'a>(&'a self,req:&'a HttpRequest,path:&String)->Option<HttpResponse>{
        match self.0.get(path) {
            Some(func)=>{
                Some(func(req))
            },
            None=>{
                None
            }
        }
    }
}
pub struct Router{
    route_map :  HashMap<Method,RefCell<HandlerMap>>
}
impl Router {
    pub fn route<T: Write>(&self,req: http::http_request::HttpRequest, stream: &mut T) {
        match &req.resource {
            Resource::Path( full_path)=>{
                match self.route_map.get(&req.method) {
                    None=>{
                    },
                    Some(map)=>{
                        if let Some(res) = map.borrow().execute_handler(&req, full_path) {
                            let info:String = res.into();
                            stream.write(info.as_bytes()).unwrap();
                            return;
                        }
                    }
                }
            }
        }
        let res = PageNotFoundHandler::handle(&req);
        let info :String = res.into();
        stream.write(info.as_bytes()).unwrap();
    }
    pub fn new()->Self{
        Router{
            route_map:HashMap::new()
        }
    }
    fn add_method(&mut self,method:Method)->Option<RefCell<HandlerMap>>{
        self.route_map.insert(method,RefCell::new(HandlerMap::new()))
    }
    pub fn register_route(&mut self,method:Method,path:String,handler_func:fn(&HttpRequest) -> HttpResponse){
        match self.route_map.get(&method) {
            Some(route)=>{
                route.borrow_mut().add(path, handler_func);
            },
            None=>{
                self.add_method(method);
                self.route_map.get(&method).unwrap().borrow_mut().add(path, handler_func);
            }
        }
    }
}
