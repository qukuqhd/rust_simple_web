use std::collections::HashMap;

#[derive(Debug, Clone, Copy,PartialEq,Eq,Hash)]
pub enum Method {
    GET,
    POST,
    Uninitialized,
}
impl From<&str> for Method {
    fn from(value: &str) -> Self {
        match value {
            "GET" => Self::GET,
            "POST" => Self::POST,
            _ => Self::Uninitialized,
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_2,
    V3_3,
    Uninitialized,
}
impl From<&str> for Version {
    fn from(value: &str) -> Self {
        match value {
            "HTTP/1.1" => Self::V1_1,
            _ => Self::Uninitialized,
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}
#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub header: HashMap<String, String>,
    pub body: String,
}
fn process_req_line(s: &str) -> (Method, Resource, Version) {
    let mut words = s.split_whitespace();
    let method = words.next().unwrap();
    let resource = words.next().unwrap();
    let version: &str = words.next().unwrap();
    (
        Method::from(method),
        Resource::Path(resource.to_string()),
        Version::from(version),
    )
}
fn process_header_line(s: &str) -> (String, String) {
    let mut header_items = s.split(":");
    (
        header_items.next().unwrap().to_string(),
        header_items.next().unwrap().to_string(),
    )
}
impl From<String> for HttpRequest {
    fn from(value: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_header = HashMap::new();
        let mut parsed_msg_body = "";
        for line in value.lines() {
            if line.contains("HTTP") {
                (parsed_method, parsed_resource, parsed_version) = process_req_line(line);
            } else if line.contains(":") {
                let (key, val) = process_header_line(line);
                parsed_header.insert(key, val);
            } else if line.len() != 0 {
                parsed_msg_body = line;
            }
        }
        Self {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            header: parsed_header,
            body: parsed_msg_body.to_string(),
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::GET);
    }
    #[test]
    fn test_version_into() {
        let v: Version = "HTTP/1.1".into();
        assert_eq!(v, Version::V1_1);
    }
    #[test]
    fn test_http_request_into() {
        let s: String = String::from("GET /greeting HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent:curl/7.71\r\nAccept:*/*\r\n\r\n");
        let mut header_expected: HashMap<String, String> = HashMap::new();
        header_expected.insert("Host".into(), " localhost".into());
        header_expected.insert("Accept".into(), "*/*".into());
        header_expected.insert("User-Agent".into(), "curl/7.71".into());
        let req: HttpRequest = s.into();
        assert_eq!(Method::GET, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(Resource::Path("/greeting".to_string()), req.resource);
        assert_eq!(header_expected, req.header);
    }
}
