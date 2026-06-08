// ============================================================
// Builder Pattern — 分步构造复杂对象，避免构造函数参数爆炸
// 对比 TS: 01_builder.ts
// 运行: cargo run --bin builder
// ============================================================

#[derive(Debug)]
struct HttpRequest {
    url: String,
    method: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

struct HttpRequestBuilder {
    url: String,
    method: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

impl HttpRequestBuilder {
    fn new() -> Self {
        Self {
            url: String::new(),
            method: "GET".into(),
            headers: Vec::new(),
            body: None,
        }
    }

    fn url(mut self, url: &str) -> Self {
        self.url = url.into();
        self
    }

    fn method(mut self, method: &str) -> Self {
        self.method = method.into();
        self
    }

    fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.push((key.into(), value.into()));
        self
    }

    fn body(mut self, body: &str) -> Self {
        self.body = Some(body.into());
        self
    }

    fn build(self) -> HttpRequest {
        HttpRequest {
            url: self.url,
            method: self.method,
            headers: self.headers,
            body: self.body,
        }
    }
}

fn main() {
    let req = HttpRequestBuilder::new()
        .url("https://api.example.com/users")
        .method("POST")
        .header("Content-Type", "application/json")
        .header("Authorization", "Bearer token-123")
        .body(r#"{"name":"Alice","age":30}"#)
        .build();

    println!("=== Builder Pattern ===");
    println!("URL:    {}", req.url);
    println!("Method: {}", req.method);
    println!("Headers:");
    for (k, v) in &req.headers {
        println!("  {}: {}", k, v);
    }
    println!("Body:   {:?}", req.body);

    let get_req = HttpRequestBuilder::new()
        .url("https://api.example.com/users/1")
        .build();

    println!("\nGET request:");
    println!("URL:    {}", get_req.url);
    println!("Method: {}", get_req.method);
    println!("Body:   {:?}", get_req.body);
}
