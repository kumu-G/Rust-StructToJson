use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn main() {
    let article: Article = Article {
        article: String::from("how to work with json in Rust"),
        author: String::from("John Doe"),
        paragraph: vec![
            Paragraph {
                name: String::from("first sentence"),
            },
            Paragraph {
                name: String::from("body of the paragraph"),
            },
            Paragraph {
                name: String::from("end of the paragraph"),
            },
        ],
    };

    let json = serde_json::to_string(&article).unwrap();
    println!("the json is: {}", json)
}

/*
这段代码的功能是将一个包含文章标题、作者和段落的结构体Article转换为JSON格式，并打印出JSON字符串。

代码的步骤如下：

1. 导入serde库的Deserialize和Serialize trait。
2. 定义一个名为Paragraph的结构体，其中包含一个名为name的字符串字段。
3. 使用#[derive(Serialize, Deserialize)]宏为Paragraph结构体实现Serialize和Deserialize trait。
4. 定义一个名为Article的结构体，其中包含一个名为article的字符串字段、一个名为author的字符串字段和一个名为paragraph的Paragraph结构体的向量字段。
5. 使用#[derive(Serialize, Deserialize)]宏为Article结构体实现Serialize和Deserialize trait。
6. 在main函数中，创建一个Article实例，其中包含一个标题、作者和三个段落。
7. 使用serde_json库的to_string函数将Article实例转换为JSON字符串，并将结果绑定到json变量上。
8. 使用println!宏打印出JSON字符串。 */
