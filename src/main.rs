#[derive(Debug)]
struct Source {
    form: Box<str>,
    pos: usize,
}

impl Source {
    fn new(form: &str) -> Self {
        Self {
            form: Box::from(form),
            pos: 0,
        }
    }

    fn next(&mut self) {
        self.pos += 1;
    }

    fn peek(&mut self) -> Option<char> {
        if self.pos >= self.form.len() {
            panic!("over length!!")
        }
        self.form.chars().nth(self.pos)
    }
}

fn main() {
    let mut form = Parser::new("12+45");
    println!("{:?}", form.expr());

    println!("{:?}", form);
}

#[derive(Debug)]
struct Parser {
    source: Source,
}

impl Parser {
    pub fn new(form: &str) -> Self {
        Self {
            source: Source::new(form),
        }
    }

    fn next(&mut self) {
        self.source.next();
    }

    fn peek(&mut self) -> Option<char> {
        self.source.peek()
    }

    fn number(&mut self) -> i32 {
        self.number_str().parse::<i32>().unwrap()
    }

    fn expr(&mut self) -> i32 {
        let mut x = self.number();
        match self.peek() {
            None => {-1}
            Some('+') => {
                self.next();
                x += self.number();
                x
            }
            Some(_) => {-1}
        }
    }

    // number := 1|2|3|4|5|6|7|8|9|0 +
    fn number_str(&mut self) -> String {
        // 読み込み開始位置
        let read_start = self.source.pos;
        let form_enu = self.source.form.chars().enumerate();

        for (iter, fig) in self.source.form.chars().enumerate() {
            // 読み込み開始位置まで読み飛ばし
            if iter < read_start { continue; }
            self.source.pos = iter;
            // 数字でなければ、そこまでの数字を返す
            if !fig.is_numeric() {
                return form_enu
                    .filter(|&(i, _)| i >= read_start && i < iter)
                    .fold("".to_string(), |s, (_, c)| format!("{}{}", s, c));
            }
        }

        form_enu
            .filter(|&(i, _)| i >= read_start)
            .fold("".to_string(), |s, (_, c)| format!("{}{}", s, c))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_test() {
        let mut form_0 = Parser::new("");
        let mut form_1 = Parser::new("1");
        let mut form_2 = Parser::new("25");
        let mut form_3 = Parser::new("1+1");
        let mut form_4 = Parser::new("12+21+43");

        // 先頭の連続する数字を返す
        assert_eq!(form_0.number_str(), "");
        assert_eq!(form_1.number_str(), "1");
        assert_eq!(form_2.number_str(), "25");
        assert_eq!(form_3.number_str(), "1");
        assert_eq!(form_4.number_str(), "12");

        let mut form_5 = Parser::new("123+456");

        // i32型に変換
        assert_eq!(form_5.number(), 123);
    }
}