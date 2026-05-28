
# Rust `serde_json` 方法表

| **类别**        | **方法**                                   | **返回类型**                 | **是否消耗 JSON** | **说明** |
|---------------|----------------------------------------|---------------------------|-----------------|---------|
| **JSON 序列化（Rust → JSON）** | `serde_json::to_string(&value)`       | `Result<String>`           | ❌ 否 | 转换为 JSON 字符串 |
|               | `serde_json::to_string_pretty(&value)` | `Result<String>`           | ❌ 否 | 生成格式化 JSON |
|               | `serde_json::to_vec(&value)`          | `Result<Vec<u8>>`         | ❌ 否 | 转换为 JSON 字节数组 |
|               | `serde_json::to_writer(writer, &value)` | `Result<()>`               | ❌ 否 | 直接写入 `std::io::Write` |
| **JSON 反序列化（JSON → Rust）** | `serde_json::from_str::<T>(json_str)`  | `Result<T>`                 | ✅ 是 | 从字符串解析 JSON |
|               | `serde_json::from_slice::<T>(json_bytes)` | `Result<T>`                 | ✅ 是 | 从字节数组解析 JSON |
|               | `serde_json::from_reader::<T, R>(reader)` | `Result<T>`                 | ✅ 是 | 从 `std::io::Read` 解析 JSON |
| **动态 JSON 解析 (`serde_json::Value`)** | `serde_json::json!({ "key": "value" })` | `Value`                    | ❌ 否 | 创建 JSON 值 |
|               | `Value::get("key")`                  | `Option<&Value>`          | ❌ 否 | 获取 JSON 字段 |
|               | `Value::as_str()`                    | `Option<&str>`            | ❌ 否 | 解析为字符串 |
|               | `Value::as_i64()`                    | `Option<i64>`             | ❌ 否 | 解析为整数 |
|               | `Value::as_f64()`                    | `Option<f64>`             | ❌ 否 | 解析为浮点数 |
|               | `Value::as_bool()`                   | `Option<bool>`            | ❌ 否 | 解析为布尔值 |
|               | `Value::is_null()`                   | `bool`                    | ❌ 否 | 判断是否为 `null` |
|               | `Value::to_string()`                 | `String`                  | ❌ 否 | 转换为 JSON 字符串 |
| **JSON 数组操作** | `Value::as_array()`                 | `Option<&Vec<Value>>`      | ❌ 否 | 解析为 JSON 数组 |
|               | `Value::as_array_mut()`              | `Option<&mut Vec<Value>>`  | ✅ 是 | 获取可变引用，允许修改 |
|               | `json!([1, 2, 3])`                   | `Value`                    | ❌ 否 | 创建 JSON 数组 |
|               | `.push(value)`                       | `()`                       | ✅ 是 | 向 JSON 数组追加元素 |
| **JSON 对象操作** | `Value::as_object()`               | `Option<&Map<String, Value>>` | ❌ 否 | 解析为 JSON 对象 |
|               | `Value::as_object_mut()`            | `Option<&mut Map<String, Value>>` | ✅ 是 | 获取可变对象 |
|               | `serde_json::Map::new()`            | `Map<String, Value>`       | ❌ 否 | 创建空 JSON 对象 |
|               | `.insert("key".to_string(), value)` | `()`                       | ✅ 是 | 插入键值对 |
|               | `.remove("key")`                    | `Option<Value>`            | ✅ 是 | 删除键值对 |
| **处理 `Option<T>`** | `serde_json::from_str::<Option<T>>(json_str)` | `Result<Option<T>>` | ✅ 是 | 解析可能为 `null` 的值 |
|               | `serde_json::to_string(&Option<T>)` | `Result<String>`           | ❌ 否 | 序列化 `Option<T>` |
| **读取 & 写入 JSON 文件** | `serde_json::from_reader(reader)` | `Result<T>`                | ✅ 是 | 读取 JSON 文件 |
|               | `serde_json::to_writer(writer, &value)` | `Result<()>`           | ❌ 否 | 写入 JSON 文件 |
| **处理 `Result<T, E>`** | `serde_json::Error` | `serde_json::Error` | ❌ 否 | 解析或序列化错误 |


解释
不会消耗 JSON 的方法（❌ 否）

读取 JSON 字段 或 解析数据（如 get()、as_str()）。
查询 JSON（如 is_null()、to_string()）。
创建 JSON（如 json!()）。
序列化 JSON（如 to_string()、to_vec()）。
会消耗 JSON 的方法（✅ 是）

反序列化（如 from_str() 会 获取所有权）。
修改 JSON（如 .push()、.insert()）。
删除 JSON 字段（如 .remove()）。
获取可变引用（如 as_array_mut()、as_object_mut()）。


```rust
use serde_json::{json, Value, Map};
use std::fs::File;
use std::io::{Read, Write};

fn main() {
    {
        // **1. JSON 序列化（Rust → JSON）**
        let data = json!({
            "name": "Alice",
            "age": 25,
            "active": true,
            "languages": ["Rust", "Python"]
        });

        let json_str = serde_json::to_string(&data).unwrap();
        println!("Serialized JSON: {}", json_str);

        let pretty_json = serde_json::to_string_pretty(&data).unwrap();
        println!("Pretty JSON:\n{}", pretty_json);
    }

    {
        // **2. JSON 反序列化（JSON → Rust）**
        let json_str = r#"{ "name": "Bob", "age": 30 }"#;
        let parsed: Value = serde_json::from_str(json_str).unwrap();
        println!("Parsed JSON: {:?}", parsed);
    }

    {
        // **3. 动态 JSON 解析 (`serde_json::Value`)**
        let data = json!({ "name": "Charlie", "age": 28, "height": 1.75 });
        println!("Name: {}", data["name"].as_str().unwrap());
        println!("Age: {}", data["age"].as_i64().unwrap());
        println!("Height: {}", data["height"].as_f64().unwrap());
    }

    {
        // **4. JSON 数组操作**
        let mut array = json!([1, 2, 3]);
        array.as_array_mut().unwrap().push(json!(4));
        println!("JSON Array: {}", array);
    }

    {
        // **5. JSON 对象操作**
        let mut obj = json!({ "city": "New York", "country": "USA" });

        obj.as_object_mut().unwrap().insert("population".to_string(), json!(8_500_000));
        obj.as_object_mut().unwrap().remove("city");

        println!("Modified JSON Object: {}", obj);
    }

    {
        // **6. 处理 `Option<T>`**
        let data = json!({ "email": null });

        if data["email"].is_null() {
            println!("Email is missing");
        }
    }

    {
        // **7. 读取 JSON 文件**
        let file = File::open("data.json");
        if let Ok(mut file) = file {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            let parsed: Value = serde_json::from_str(&contents).unwrap();
            println!("Loaded JSON: {}", parsed);
        }
    }

    {
        // **8. 写入 JSON 文件**
        let data = json!({ "status": "ok", "code": 200 });

        let mut file = File::create("output.json").unwrap();
        file.write_all(serde_json::to_string_pretty(&data).unwrap().as_bytes())
            .unwrap();
    }

    {
        // **9. 处理 `Result<T, E>`**
        let invalid_json = "{ invalid json }";
        let result: Result<Value, _> = serde_json::from_str(invalid_json);

        match result {
            Ok(value) => println!("Parsed JSON: {}", value),
            Err(e) => println!("Error parsing JSON: {}", e),
        }
    }
}

```