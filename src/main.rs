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
}

fn main() {

}

// number := 1|2|3|4|5|6|7|8|9|0 +
fn number(mut s: Source) -> String {
    let start = s.pos;
    for (iter, num) in s.form.chars().enumerate() {
        if iter < start { continue; }
        if !num.is_numeric() {
            s.pos = iter;
            return s.form
                .chars()
                .enumerate()
                .filter(|&(i, _)| i >= start && i < iter)
                .fold("".to_string(), |s, (_, c)| format!("{}{}", s, c));
        }
    }
    s.form.into_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_test() {
        let form_0 = Source::new("");
        let form_1 = Source::new("1");
        let form_2 = Source::new("25");
        let form_3 = Source::new("1+1");
        let form_4 = Source::new("12+21+43");

        // 先頭の連続する数字を返す
        assert_eq!(number(form_0), "");
        assert_eq!(number(form_1), "1");
        assert_eq!(number(form_2), "25");
        assert_eq!(number(form_3), "1");
        assert_eq!(number(form_4), "12");

        let form_5 = Source::new("123+456");

        // i32型に変換
        assert_eq!(number(form_5).parse::<i32>().unwrap(), 123);
    }
}