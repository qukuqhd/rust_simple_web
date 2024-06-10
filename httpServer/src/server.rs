use std::{cell::RefCell,  io::Read, net::TcpListener, rc::Rc};

use http::http_request::HttpRequest;

use crate::router::RouterMap;
pub struct Server<'a> {
    pre_path :String,//前置路由
    socket_addr: &'a str,
    router: Rc<RefCell<RouterMap>>,
}
type ServerGroup<'a> = Server<'a>;
impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Self {
        Self {
            pre_path: "".into(),
            socket_addr,
            router: Rc::new(RefCell::new(RouterMap::new())),
        }
    }
    //服务运行
    pub fn run(&mut self) {
        let connection_listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("Http Server running on {}", self.socket_addr);
        for stream in connection_listener.incoming() {
            let mut stream = stream.unwrap();
            println!("Connection established");
            let mut read_buf = [0; 2000];
            stream.read(&mut read_buf).unwrap();
            let req: HttpRequest = String::from_utf8(read_buf.to_vec()).unwrap().into();
            self.router.borrow().handle_req(&self.pre_path,&req, &mut stream);
        }
    }
    pub fn get(
        &mut self,
        path: String,
        handler_func: fn(&HttpRequest) -> http::http_response::HttpResponse,
    ) {
        self.router.borrow_mut().get(format!("{}{}",self.pre_path,path), handler_func)
    }
    pub fn post(
        &mut self,
        path: String,
        handler_func: fn(&HttpRequest) -> http::http_response::HttpResponse,
    ) {
        self.router.borrow_mut().post(format!("{}{}",self.pre_path,path), handler_func)
    }
    //创建路由分组
    pub fn create_group(&self,child_path:String)->ServerGroup<'a>{
        ServerGroup{
            socket_addr:&self.socket_addr,
            router:self.router.clone(),
            pre_path:format!("{}/{}",self.pre_path,child_path)//获取新的路由服务组前缀
        }
    }
}
