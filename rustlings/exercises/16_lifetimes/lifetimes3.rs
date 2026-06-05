struct Book2<'a, 'b> {
    author: &'a str,
    title: &'b str,
}

fn test() {
    let title = String::from("Rust Programming");

    // 只提取 title 的引用，不保留整个 book
    let title_ref;
    {
        let author = String::from("Jane Smith");
        let book = Book2 {
            author: &author,
            title: &title,
        };
        title_ref = book.title; // 只借用 title
        println!("Inside: {} by {}", book.title, book.author);
    } // author 和 book 销毁，但 title_ref 仍然有效
    println!("Outside title: {}", title_ref);
}

struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let book = Book {
        author: "George Orwell",
        title: "1984",
    };

    println!("{} by {}", book.title, book.author);
    test();
}
