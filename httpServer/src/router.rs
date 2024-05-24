pub mod  router{
    use std::io::Write;

    pub fn route<T:Write>(req: http::http_request::HttpRequest, stream: &mut T) {
        todo!()
    }


}