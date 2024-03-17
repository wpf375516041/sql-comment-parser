#[derive(Debug)]
struct SqlCommentParser<'a> {
    sql: &'a str,
    pos: usize,
    start: usize,
}

#[derive(Debug)]
struct Comment {
    start_index: usize,
    end_index: usize,
}

impl Comment {
    fn new(start: usize, end: usize) -> Self {
        Self {
            start_index: start,
            end_index: end,
        }
    }
}

impl<'a> SqlCommentParser<'a> {
    fn new(sql: &'a str) -> Self {
        Self {
            sql,
            pos: 0,
            start: 0,
        }
    }

    fn get_comment_sql(&mut self) -> String {
        let mut comment_sql = String::new();
        loop {
            match self.next_comment() {
                Some(comment_range) => {
                    comment_sql
                        .push_str(&self.sql[comment_range.start_index..comment_range.end_index]);
                }
                None => {
                    break;
                }
            };
        }
        comment_sql
    }

    fn remove_comment_sql(&mut self) -> String {
        let mut new_sql = String::new();
        let mut start_index = 0;
        loop {
            match self.next_comment() {
                Some(comment_range) => {
                    new_sql.push_str(&self.sql[start_index..comment_range.start_index]);
                    start_index = comment_range.end_index;
                }
                None => {
                    if start_index != self.sql.len() {
                        new_sql.push_str(&self.sql[start_index..]);
                    }
                    break;
                }
            };
        }
        new_sql
    }

    fn next_comment(&mut self) -> Option<Comment> {
        while self.pos < self.sql.len() {
            let c = self.sql.as_bytes()[self.pos] as char;
            let start_index;
            match c {
                '\'' => {
                    self.start = self.pos;
                    self.pos += 1;
                    while self.pos < self.sql.len() {
                        let c = self.sql.as_bytes()[self.pos] as char;
                        self.pos += 1;
                        if c == '\'' {
                            break;
                        }
                    }
                }
                '`' => {
                    self.start = self.pos;
                    self.pos += 1;
                    while self.pos < self.sql.len() {
                        let c = self.sql.as_bytes()[self.pos] as char;
                        self.pos += 1;
                        if c == '`' {
                            break;
                        }
                    }
                }
                '\"' => {
                    self.start = self.pos;
                    self.pos += 1;
                    while self.pos < self.sql.len() {
                        let c = self.sql.as_bytes()[self.pos] as char;
                        self.pos += 1;
                        if c == '\"' {
                            break;
                        }
                    }
                }
                '/' => {
                    // possible start of '/*'
                    if self.pos + 1 < self.sql.len() {
                        let c = self.sql.as_bytes()[self.pos + 1] as char;
                        if c == '*' {
                            start_index = self.pos;
                            // 从pos + 2开始查找"*/"
                            let end: usize = match self.sql.find("*/") {
                                Some(end) => end + "*/".len(),
                                None => self.sql.len(),
                            };

                            // 更新pos并计算end_index
                            self.pos = end;
                            let end_index = self.pos;

                            return Some(Comment::new(start_index, end_index));
                        }
                    }
                }
                '-' => {
                    // possible start of '--' comment
                    if c == '-'
                        && self.pos + 1 < self.sql.len()
                        && self.sql.as_bytes()[self.pos + 1] as char == '-'
                    {
                        start_index = self.pos;
                        self.pos = SqlCommentParser::index_of_line_end(self.sql, self.pos + 2);
                        let end_index = self.pos;
                        return Some(Comment::new(start_index, end_index));
                    }
                }
                _ => {
                    if SqlCommentParser::is_open_quote(c) {
                        break;
                    } else {
                        loop {
                            self.pos += 1;
                            if self.pos >= self.sql.len() {
                                break;
                            }
                            let c = self.sql.as_bytes()[self.pos] as char;
                            match c {
                                '\'' | '`' | '\"' | '/' => break,
                                '-' => {
                                    if self.pos + 1 < self.sql.len()
                                        && self.sql.as_bytes()[self.pos + 1] as char == '-'
                                    {
                                        break;
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
        return None;
    }

    fn index_of_line_end(sql: &'a str, mut i: usize) -> usize {
        let length = sql.len();
        while i < length {
            let c = sql.as_bytes()[i] as char;
            match c {
                '\r' | '\n' => {
                    return i;
                }
                _ => {
                    i += 1;
                }
            }
        }
        return i;
    }

    fn is_open_quote(character: char) -> bool {
        match character {
            '\"' | '`' | '\'' => {
                return true;
            }
            _ => {
                return false;
            }
        }
    }
}

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
