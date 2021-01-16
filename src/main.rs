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
    let mut form = Source::new("124567+12");
    println!("{:?}", number(&mut form));

    println!("{:?}", form);
}

// number := 1|2|3|4|5|6|7|8|9|0 +
fn number(s: &mut Source) -> String {
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
    s.form
        .chars()
        .enumerate()
        .fold("".to_string(), |s, (_, c)| format!("{}{}", s, c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_test() {
        let mut form_0 = Source::new("");
        let mut form_1 = Source::new("1");
        let mut form_2 = Source::new("25");
        let mut form_3 = Source::new("1+1");
        let mut form_4 = Source::new("12+21+43");

        // 先頭の連続する数字を返す
        assert_eq!(number(&mut form_0), "");
        assert_eq!(number(&mut form_1), "1");
        assert_eq!(number(&mut form_2), "25");
        assert_eq!(number(&mut form_3), "1");
        assert_eq!(number(&mut form_4), "12");

        let mut form_5 = Source::new("123+456");

        // i32型に変換
        assert_eq!(number(&mut form_5).parse::<i32>().unwrap(), 123);
    }
}