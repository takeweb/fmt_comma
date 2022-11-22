/// format comma
/// # Example
/// ```
/// use fmt_comma::{format, format_decimal, rm_comma, rm_comma_decimal};
/// let pc_price: f32 = 9800000.0;
/// let company_a_send_cost: f32 = 1200.0;
/// let company_a_pc_price = pc_price * 0.8 + company_a_send_cost;
/// let company_b_pc_price = pc_price * 0.9;
/// println!("Company A:{}yen", format(company_a_pc_price as isize)); // → 7,841,200yen
/// println!("Company B:{}yen", format(company_b_pc_price as isize)); // → 8,820,000yen
/// ```
pub fn format(n: isize) -> String {
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

// format comma for decimal
// # Example
// ```
// println!("Company C:{}yen", format_decimal(10003.333));   // → 10,003.333yen
// println!("Company D:{}yen", format_decimal(3380.2));      // → 3,380.2yen
// println!("Company E:{}yen", format_decimal(3380.0));      // → 3,380yen
// ```

pub fn format_decimal(n: f64) -> String {
    let s: String = n.to_string();
    let vec: Vec<String> = s.split('.')
                                .map(|s| s.to_string())
                                .collect();
    let param = vec[0].parse::<isize>().unwrap();
    let mut result = format(param);
    if vec.len() > 1 {
        result.push('.');
        result.push_str(vec[1].as_str());
    }
    result
}

// remove comma
// # Example
// ```
// println!("Company F:{}yen", rm_comma("10,003")); // → 10003yen
// ```
pub fn rm_comma(s: &str) -> isize {
    s.replace(",", "").parse::<isize>().unwrap()
}

// remove comma for decimal
// # Example
// ```
// println!("Company G:{}yen", rm_comma_decimal("10,003.333")); // → 10003.333yen
// ```
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
fn test_format() {
    assert_eq!(format(338), "338");
    assert_eq!(format(3380), "3,380");
    assert_eq!(format(33800), "33,800");
    assert_eq!(format(338000000), "338,000,000");
    assert_eq!(format(3380000000), "3,380,000,000");
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
    assert_eq!(format_decimal(338 as f64), "338");
    assert_eq!(format_decimal(3380 as f64), "3,380");
    assert_eq!(format_decimal(33800 as f64), "33,800");
    assert_eq!(format_decimal(338000000 as f64), "338,000,000");
    assert_eq!(format_decimal(3380000000u32 as f64), "3,380,000,000");
    assert_eq!(format_decimal(338.2 as f64), "338.2");
    assert_eq!(format_decimal(3380.2 as f64), "3,380.2");
    assert_eq!(format_decimal(33800.2 as f64), "33,800.2");
    assert_eq!(format_decimal(338000000.2 as f64), "338,000,000.2");
    assert_eq!(format_decimal(3380000000.2 as f64), "3,380,000,000.2");
    assert_eq!(format_decimal(338.0 as f64), "338");
    assert_eq!(format_decimal(3380.0 as f64), "3,380");
    assert_eq!(format_decimal(33800.0 as f64), "33,800");
    assert_eq!(format_decimal(338000000.0 as f64), "338,000,000");
    assert_eq!(format_decimal(3380000000.0 as f64), "3,380,000,000");
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