## 软件介绍
> 移植于：
> https://github.com/wangjie-fourth/SqlParser/blob/main/src/main/java/myself/SqlCommentParser.java

解析sql语句中的注释，具备以下两个能力：
- 提取sql中的注释（✅）
- 移除sql中的注释（✅）
## 快速使用
```rust
use sql_comment_parser::SqlCommentParser;

fn main() {
    let sql = "SELECT * FROM table--; -- This is a single line comment";
    let mut parser = SqlCommentParser::new(&sql);
    let comment_sql = parser.get_comment_sql();
    let cleaned_sql = parser.remove_comment_sql();
    println!("{}", cleaned_sql); //打印SELECT * FROM table
    println!("{}", comment_sql); //打印--; -- This is a single line comment
}
```
## 参考链接
https://wangjie-fourth.github.io/2021/01/30/experience/remove-comment-in-sql/
