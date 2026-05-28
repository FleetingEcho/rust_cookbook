#[cfg(test)]
mod tests {
    // 导入必要的模块
    use std::fs::OpenOptions;
    use std::io::Write;

    // 这个测试向文件写入内容
    #[test]
    fn test_file() {
        // 打开 ferris.txt 文件，如果不存在则创建一个
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("ferris.txt")
            .expect("无法打开 ferris.txt");

        // 循环打印 "Ferris" 5 次
        for _ in 0..5 {
            file.write_all("Ferris\n".as_bytes())
                .expect("无法写入 ferris.txt");
        }
    }

    // 这个测试尝试写入同一个文件
    #[test]
    fn test_file_also() {
        // 打开 ferris.txt 文件，如果不存在则创建一个
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("ferris.txt")
            .expect("无法打开 ferris.txt");

        // 循环打印 "Corro" 5 次
        for _ in 0..5 {
            file.write_all("Corro\n".as_bytes())
                .expect("无法写入 ferris.txt");
        }
    }
}
