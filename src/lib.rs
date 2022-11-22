pub fn fmt_comma(n: isize) -> String {
    let chars: Vec<char> = n.to_string().chars().collect();
    let vec: Vec::<String> = chars.iter()
                                    .enumerate()
                                    .map(|(i, v)| split(i, v, chars.len()))
                                    .collect();
    vec.join("")
}

fn split(i: usize, c: &char, len: usize) -> String {
    let mut s = String::from("");
    let position = len - i;
    if position % 3 == 0 && len != position {
        s.push_str(",");
    }
    s.push(*c);
    s
}

pub fn fmt_comma_decimal(n: f64) -> String {
    let s: String = n.to_string();
    let vec: Vec<String> = s.split('.')
                                .map(|s| s.to_string())
                                .collect();
    let param = vec[0].parse::<isize>().unwrap();
    let mut result = fmt_comma(param);
    if vec.len() > 1 {
        result.push('.');
        result.push_str(vec[1].as_str());
    }
    result
}

pub fn rm_comma(s: &str) -> isize {
    s.replace(",", "").parse::<isize>().unwrap()
}

pub fn rm_comma_decimal(s: &str) -> f64 {
    let vec: Vec<String> = s.split('.')
                                .map(|s| s.to_string())
                                .collect();
    let tmp = rm_comma(vec[0].as_str());
    let mut result = tmp.to_string();
    if vec.len() > 1 {
        result.push('.');
        result.push_str(vec[1].as_str());
    }
    let result = result.parse::<f64>().unwrap();
    result
}

#[test]
fn test_fmt_comma() {
    assert_eq!(fmt_comma(338), "338");
    assert_eq!(fmt_comma(3380), "3,380");
    assert_eq!(fmt_comma(33800), "33,800");
    assert_eq!(fmt_comma(338000000), "338,000,000");
    assert_eq!(fmt_comma(3380000000), "3,380,000,000");
}

#[test]
fn test_rm_comma() {
    assert_eq!(rm_comma("338"), 338);
    assert_eq!(rm_comma("3,380"), 3380);
    assert_eq!(rm_comma("33,800"), 33800);
    assert_eq!(rm_comma("338,000,000"), 338000000);
    assert_eq!(rm_comma("3,380,000,000"), 3380000000);
}

#[test]
fn test_fmt_comma_decimal() {
    assert_eq!(fmt_comma_decimal(338 as f64), "338");
    assert_eq!(fmt_comma_decimal(3380 as f64), "3,380");
    assert_eq!(fmt_comma_decimal(33800 as f64), "33,800");
    assert_eq!(fmt_comma_decimal(338000000 as f64), "338,000,000");
    assert_eq!(fmt_comma_decimal(3380000000u32 as f64), "3,380,000,000");
    assert_eq!(fmt_comma_decimal(338.2 as f64), "338.2");
    assert_eq!(fmt_comma_decimal(3380.2 as f64), "3,380.2");
    assert_eq!(fmt_comma_decimal(33800.2 as f64), "33,800.2");
    assert_eq!(fmt_comma_decimal(338000000.2 as f64), "338,000,000.2");
    assert_eq!(fmt_comma_decimal(3380000000.2 as f64), "3,380,000,000.2");
    assert_eq!(fmt_comma_decimal(338.0 as f64), "338");
    assert_eq!(fmt_comma_decimal(3380.0 as f64), "3,380");
    assert_eq!(fmt_comma_decimal(33800.0 as f64), "33,800");
    assert_eq!(fmt_comma_decimal(338000000.0 as f64), "338,000,000");
    assert_eq!(fmt_comma_decimal(3380000000.0 as f64), "3,380,000,000");
}

#[test]
fn test_rm_comma_decimal() {
    assert_eq!(rm_comma_decimal("338"), 338 as f64);
    assert_eq!(rm_comma_decimal("3,380"), 3380 as f64);
    assert_eq!(rm_comma_decimal("33,800"), 33800 as f64);
    assert_eq!(rm_comma_decimal("338,000,000"), 338000000 as f64);
    assert_eq!(rm_comma_decimal("3,380,000,000"), 3380000000u32 as f64);
    assert_eq!(rm_comma_decimal("338.2"), 338.2 as f64);
    assert_eq!(rm_comma_decimal("3,380.2"), 3380.2 as f64);
    assert_eq!(rm_comma_decimal("33,800.2"), 33800.2 as f64);
    assert_eq!(rm_comma_decimal("338,000,000.2"), 338000000.2 as f64);
    assert_eq!(rm_comma_decimal("3,380,000,000.2"), 3380000000.2 as f64);
    assert_eq!(rm_comma_decimal("338"), 338.0 as f64);
    assert_eq!(rm_comma_decimal("3,380"), 3380.0 as f64);
    assert_eq!(rm_comma_decimal("33,800"), 33800.0 as f64);
    assert_eq!(rm_comma_decimal("338,000,000"), 338000000.0 as f64);
    assert_eq!(rm_comma_decimal("3,380,000,000"), 3380000000.0 as f64);
}