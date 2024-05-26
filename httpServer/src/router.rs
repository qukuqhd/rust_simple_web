use std::{
    borrow::Borrow,
    cell::RefCell,
    collections::HashMap,
    io::{self, Write},
};

use http::{
    http_request::{HttpRequest, Method, Resource},
    http_response::HttpResponse,
};

use crate::handler::{Handler, PageNotFoundHandler};

//路由树
#[derive(Clone)]
struct RouteTree {
    path: String, //当前的路径部分
    handler_func: Option<fn(&http::http_request::HttpRequest) -> HttpResponse>, //节点对应的请求处理函数
    children: Vec<RefCell<RouteTree>>,                                          //子树
}
impl RouteTree {
    pub fn new(path: String) -> Self {
        Self {
            path: path,
            handler_func: None,
            children: Vec::new(),
        }
    }
    pub fn root() -> Self {
        Self::new("/".to_string())
    }
    fn regis_route(
        &self,
        path: String,
        handler_func: fn(&http::http_request::HttpRequest) -> HttpResponse,
    ) {
        let mut path_vec: Vec<_> = path.split("/").collect();
        dfs_regis(&RefCell::new((*self).clone()), &mut path_vec, handler_func);
    }
    fn find_handler(
        &self,
        path: String,
    ) -> Option<fn(&http::http_request::HttpRequest) -> HttpResponse> {
        let mut path_vev: Vec<_> = path.split("/").collect();
        match dfs_find(&self, &mut path_vev) {
            None => None,
            Some(ref node) => node.handler_func,
        }
    }
}

fn dfs_regis(
    node: &RefCell<RouteTree>,
    path_vec: &mut Vec<&str>,
    handler_func: fn(&http::http_request::HttpRequest) -> HttpResponse,
) {
    if path_vec.len() == 0 {
        //抵达了要添加节点的层次，并且没有路径冲突
        if node.borrow().handler_func == None {
            node.borrow_mut().handler_func = Some(handler_func);
        }
    } else {
        for child in node.borrow().children.iter() {
            if child.borrow().path == path_vec[0] {
                path_vec.remove(0);
                dfs_regis(&child, path_vec, handler_func);
            }
        }
        let child = RefCell::new(RouteTree::new(path_vec[0].into()));
        path_vec.remove(0);
        dfs_regis(&child, path_vec, handler_func);
        node.borrow_mut().children.push(child);
    }
}
fn dfs_find<'a>(node: &'a RouteTree, path_vec: &'a mut Vec<&'a str>) -> Option<&'a RouteTree> {
    if path_vec.len() == 0 {
        Some(node)
    } else {
        for child in node.children.iter() {
            if child.borrow().path == path_vec[0] {
                dfs_find(node, &mut path_vec.clone());
            }
        }
        None
    }
}
pub struct RouterMap {
    tree_map: HashMap<Method, RefCell<RouteTree>>,
}
impl RouterMap {
    pub fn new() -> Self {
        Self {
            tree_map: HashMap::new(),
        }
    }
    fn regis_route(
        &mut self,
        method: Method,
        path: String,
        handler_func: fn(&http::http_request::HttpRequest) -> HttpResponse,
    ) {
        match self.tree_map.get(&method) {
            None => {
                let new_tree = RouteTree::root();
                new_tree.regis_route(path, handler_func);
                self.tree_map.insert(method, RefCell::new(new_tree));
            }
            Some(tree) => {
                tree.borrow_mut().regis_route(path, handler_func);
            }
        }
    }
    pub fn get(
        &mut self,
        path: String,
        handler_func: fn(&http::http_request::HttpRequest) -> HttpResponse,
    ) {
        self.regis_route(Method::GET, path, handler_func);
    }
    pub fn post(
        &mut self,
        path: String,
        handler_func: fn(&http::http_request::HttpRequest) -> HttpResponse,
    ) {
        self.regis_route(Method::POST, path, handler_func);
    }
    fn execute_handler<'a>(&'a self, req: &'a HttpRequest, path: String) -> HttpResponse {
        match self.tree_map.get(&req.method) {
            None => PageNotFoundHandler::handle(req),
            Some(tree) => match tree.borrow().find_handler(path) {
                Some(handler) => handler(&req),
                None => PageNotFoundHandler::handle(&req),
            },
        }
    }
    pub fn handle_req<T: io::Write>(&self, req: &HttpRequest, stream: &mut T) {
        let info: String;
        let Resource::Path(path) = &req.resource;
        info = self.execute_handler(req, path.clone()).into();

        stream.write(info.as_bytes());
    }
}
