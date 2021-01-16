// number := 1|2|3|4|5|6|7|8|9|0+
fn number(s: &str) -> String {
    let mut num = String::new();
    for c in s.chars() {
        if c.is_numeric() {
            num.push(c);
        } else {
            break;
        }
    }
    num
}

#[test]
fn number_test() {
    let form_0 = "";
    let form_1 = "1";
    let form_2 = "25";
    let form_3 = "1+1";
    let form_4 = "12+21+43";

    // 先頭の連続する数字を返す
    assert_eq!(number(form_0), "");
    assert_eq!(number(form_1), "1");
    assert_eq!(number(form_2), "25");
    assert_eq!(number(form_3), "1");
    assert_eq!(number(form_4), "12");

    // i32型に変換
    assert_eq!(number(form_4).parse::<i32>().unwrap(), 12);
}

fn main() {
    let formula = "";
    let num = number(formula);
    println!("{:?}", num);
}
