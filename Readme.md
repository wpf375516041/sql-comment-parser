## 软件介绍
这是一个解析sql语句中的注释，具备一下两个能力：
- 提取sql中的注释（开发中）
- 移除sql中的注释（已完成）
## 快速使用
```rust
let sql = "SELECT * FROM table--; -- This is a single line comment";
let mut parser = SqlCommentParser::new(&sql);
let cleaned_sql = parser.remove_comment_sql();
println!("{}", cleaned_sql);
```
## 参考链接
https://github.com/wangjie-fourth/SqlParser/blob/main/src/main/java/myself/SqlCommentParser.java
