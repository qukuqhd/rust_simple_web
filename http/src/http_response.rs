use std::{collections::HashMap, io::Write};
#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}
impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1".into(),
            status_code: "200".into(),
            status_text: "OK".into(),
            headers: None,
            body: None,
        }
    }
}
impl<'a> From<HttpResponse<'a>> for String {
    fn from(value: HttpResponse<'a>) -> Self {
        let res = value.clone();
        format!(
            "{} {} {}\r\n{} Content-Length: {}\r\n\r\n{}",
            res.version(),
            res.status_code(),
            res.status_text(),
            res.header(),
            res.body().len(),
            res.body()
        )
    }
}
impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> Self {
        let mut response = Self::default();
        if status_code != "200" {
            response.status_code = status_code.into();
        }
        response.headers = match &headers {
            Some(_h) => headers,
            _ => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            }
        };
        response.status_text = match response.status_code {
            "200" => "OK",
            "404" => "Not Found".into(),
            "505" => "Internal Server Error".into(),
            "400" | _ => "Bad Request".into(),
        };
        response.body = body;
        response
    }
    pub fn send_response<T: Write>(&self, write_stream: &mut T) -> Result<(), std::io::Error> {
        let res = self.clone();
        let response_str: String = String::from(res);
        write!(write_stream, "{}", response_str)
    }
    pub fn version(&self) -> &str {
        &self.version
    }
    pub fn status_code(&self) -> &str {
        &self.status_code
    }
    pub fn status_text(&self) -> &str {
        &self.status_text
    }
    pub fn header(&self) -> String {
        let map = self.headers.clone().unwrap();
        let mut header_string: String = "".into();
        for (k, v) in map.iter() {
            header_string = format!("{}{}:{}\r\n", header_string, k, v);
        }
        header_string
    }
    pub fn body(&self) -> &str {
        match &self.body {
            None => "",
            Some(val) => val.as_str(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_response_struct_creation_200() {
        let res = HttpResponse::new("200", None, Some("xxxx".into()));
        let res_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxxx".into()),
        };
        assert_eq!(res, res_expected);
    }
    #[test]
    fn test_response_struct_creation_404() {
        let res = HttpResponse::new("404", None, Some("xxxx".into()));
        let res_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxxx".into()),
        };
        assert_eq!(res, res_expected);
    }
    #[test]
    fn test_http_response_creation() {
        let res_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxxx".into()),
        };
        let res_str: String = res_expected.into();
        println!("{}", res_str);
    }
}
