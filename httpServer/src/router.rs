use std::{cell::RefCell, collections::HashMap, io::Write};

use http::{
    http_request::{HttpRequest, Method, Resource},
    http_response::HttpResponse,
};

use crate::handler::{Handler, PageNotFoundHandler};

//路由树
#[derive(Clone)]
struct RouteTree {
    handler_func: Option<fn(&http::http_request::HttpRequest) -> HttpResponse>, //节点对应的请求处理函数
    children: HashMap<String, RouteTree>,                                       //子树
}
impl RouteTree {
    pub fn new() -> Self {
        Self {
            handler_func: None,
            children: HashMap::new(),
        }
    }
    pub fn root() -> Self {
        Self::new()
    }
    //路由注册
    fn regis_route(
        &mut self,
        path: String,
        handler_func: fn(&http::http_request::HttpRequest) -> HttpResponse,
    ) {
        let mut current_node = self;
        if path == "" || path == "/" {
            //特殊情况的匹配对根节点进行处理
            match current_node.handler_func {
                None => {
                    current_node.handler_func = Some(handler_func);
                }
                _ => panic!("repeat regis"),
            }
            return;
        } else {
            let path_list: Vec<_> = path.split("/").collect();
            match path_list.len() {
                0 | 1 => {
                    return;
                }
                _ => {
                    //如果是是一个合理的路由
                    let mut last_match = 0;
                    for letter_counter in 1..path_list.len() {
                        //先往下探测以及有的路由
                        if current_node
                            .children
                            .contains_key(path_list[letter_counter])
                        {
                            current_node = current_node
                                .children
                                .get_mut(path_list[letter_counter])
                                .unwrap();
                        } else {
                            last_match = letter_counter;
                            break;
                        }
                        last_match = letter_counter + 1;
                    }
                    if last_match == path_list.len() {
                        //判断是否到达目的的路由
                        match current_node.handler_func {
                            None => current_node.handler_func = Some(handler_func),
                            _ => panic!("repeat regis"),
                        }
                    } else {
                        //否则再往下创建新的节点
                        for new_counter in last_match..path_list.len() {
                            current_node
                                .children
                                .insert((path_list[new_counter]).to_string(), RouteTree::new());
                            current_node = current_node
                                .children
                                .get_mut(path_list[new_counter])
                                .unwrap();
                        }
                        match current_node.handler_func {
                            None => current_node.handler_func = Some(handler_func),
                            _ => panic!("repeat regis"),
                        }
                    }
                }
            }
        }
    }
    //查询路由
    fn find_handler(
        &self,
        path: String,
    ) -> Option<fn(&http::http_request::HttpRequest) -> HttpResponse> {
        if path == "" || path == "/" {
            //特殊字符串获取根的路由
            self.handler_func
        } else {
            let path_list: Vec<_> = path.split("/").collect();
            match path_list.len() {
                0 | 1 => return None, //不是合理的路由匹配字符串
                _ => {
                    //合理的路由匹配字符串
                    let mut current_node = self;
                    for index in 1..path_list.len() {
                        //往下寻找
                        if current_node.children.contains_key(path_list[index]) {
                            current_node = current_node.children.get(path_list[index]).unwrap();
                        } else {
                            //没有了就返回node
                            return None;
                        }
                    }
                    return current_node.handler_func; //找到节点返回
                }
            }
        }
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
                let mut new_tree = RouteTree::root();
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
    pub fn handle_req<T: Write>(&self, pre_path: &str, req: &HttpRequest, stream: &mut T) {
        let info: String;
        let Resource::Path(path) = &req.resource;
        info = self
            .execute_handler(req, format!("{}{}", pre_path, path.clone()))
            .into();

        stream.write(info.as_bytes()).unwrap();
    }
}
