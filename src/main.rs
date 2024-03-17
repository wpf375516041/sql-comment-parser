use sql_comment_parser::SqlCommentParser;

fn main() {
    let sql = "SELECT * FROM table--; -- This is a single line comment";
    let mut parser = SqlCommentParser::new(&sql);
    let comment_sql = parser.get_comment_sql();
    let cleaned_sql = parser.remove_comment_sql();
    println!("{}", cleaned_sql);
    println!("{}", comment_sql);

    let sql_with_two_comments = "SELECT * FROM table--; -- Comment 1\n-- Comment 2";
    let mut parser2 = SqlCommentParser::new(&sql_with_two_comments);
    let cleaned_sql2 = parser2.remove_comment_sql();
    println!("{}", cleaned_sql2);

    let sql = "SELECT * FROM table /* This is a multiline
                     comment that spans across multiple lines */ WHERE id = 1";
    let mut parser = SqlCommentParser::new(&sql);
    let cleaned_sql = parser.remove_comment_sql();
    println!("{}", cleaned_sql);

    let sql_with_quotes_and_comments =
        r#"SELECT "col1" FROM `table` WHERE 'string' = 'value' /* Multiline comment */ AND id = 2"#;
    let mut parser3 = SqlCommentParser::new(&sql_with_quotes_and_comments);
    let cleaned_sql3 = parser3.remove_comment_sql();
    println!("{}", cleaned_sql3);
}
